extern crate alloc;

use alloc::string::String;
use core::hint::spin_loop;
use x86_64::instructions::port::Port;

pub const ATA_PRIMARY_IO: u16 = 0x1F0;
pub const ATA_SECONDARY_IO: u16 = 0x170;

const REG_DATA: u16 = 0;
const REG_ERROR: u16 = 1;
const REG_SECTOR_COUNT: u16 = 2;
const REG_LBA_LOW: u16 = 3;
const REG_LBA_MID: u16 = 4;
const REG_LBA_HIGH: u16 = 5;
const REG_DRIVE_HEAD: u16 = 6;
const REG_STATUS_COMMAND: u16 = 7;

const STATUS_ERR: u8 = 0x01;
const STATUS_DRQ: u8 = 0x08;
const STATUS_DF: u8 = 0x20;
const STATUS_BSY: u8 = 0x80;

const CMD_IDENTIFY: u8 = 0xEC;
const CMD_READ_SECTORS: u8 = 0x20;
const CMD_WRITE_SECTORS: u8 = 0x30;
const CMD_READ_SECTORS_EXT: u8 = 0x24;
const CMD_WRITE_SECTORS_EXT: u8 = 0x34;
const CMD_FLUSH_CACHE: u8 = 0xE7;
const CMD_FLUSH_CACHE_EXT: u8 = 0xEA;

const POLL_LIMIT: usize = 1_000_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AtaDrive {
    pub base_port: u16,
    pub is_master: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DriveInfo {
    pub model: String,
    pub serial: String,
    pub sector_count: u64,
    pub supports_lba48: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DetectedDrive {
    pub drive: AtaDrive,
    pub info: DriveInfo,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AtaError {
    DeviceFault,
    ErrorRegister(u8),
    LbaOutOfRange,
    NotAta,
    NotPresent,
    TimedOut,
}

impl AtaDrive {
    pub const fn new(base_port: u16, is_master: bool) -> Self {
        Self {
            base_port,
            is_master,
        }
    }

    pub fn detect(&self) -> Result<DriveInfo, AtaError> {
        let identify_words = self.identify_words()?;
        let supports_lba48 = (identify_words[83] & (1 << 10)) != 0;
        let sector_count = if supports_lba48 {
            ((identify_words[103] as u64) << 48)
                | ((identify_words[102] as u64) << 32)
                | ((identify_words[101] as u64) << 16)
                | (identify_words[100] as u64)
        } else {
            ((identify_words[61] as u64) << 16) | (identify_words[60] as u64)
        };

        Ok(DriveInfo {
            model: decode_ata_string(&identify_words[27..47]),
            serial: decode_ata_string(&identify_words[10..20]),
            sector_count,
            supports_lba48,
        }
    }

    pub fn read_sector(&self, lba: u64, buf: &mut [u8; 512]) -> Result<(), AtaError> {
        let use_lba48 = lba > 0x0FFF_FFFF;
        if use_lba48 {
            self.program_lba48(lba, 1)?;
            self.write_command(CMD_READ_SECTORS_EXT);
        } else {
            self.program_lba28(lba as u32, 1)?;
            self.write_command(CMD_READ_SECTORS);
        }

        self.wait_for_drq()?;

        for chunk in buf.chunks_exact_mut(2) {
            let word = self.read_data();
            let bytes = word.to_le_bytes();
            chunk[0] = bytes[0];
            chunk[1] = bytes[1];
        }

        self.wait_for_command_completion()
    }

    pub fn write_sector(&self, lba: u64, data: &[u8; 512]) -> Result<(), AtaError> {
        let use_lba48 = lba > 0x0FFF_FFFF;
        if use_lba48 {
            self.program_lba48(lba, 1)?;
            self.write_command(CMD_WRITE_SECTORS_EXT);
        } else {
            self.program_lba28(lba as u32, 1)?;
            self.write_command(CMD_WRITE_SECTORS);
        }

        self.wait_for_drq()?;

        for chunk in data.chunks_exact(2) {
            let word = u16::from_le_bytes([chunk[0], chunk[1]]);
            self.write_data(word);
        }

        self.wait_for_command_completion()?;
        self.write_command(if use_lba48 {
            CMD_FLUSH_CACHE_EXT
        } else {
            CMD_FLUSH_CACHE
        };
        self.wait_for_command_completion()
    }

    fn identify_words(&self) -> Result<[u16; 256], AtaError> {
        self.select_for_identify();
        self.io_delay();
        self.write_reg(REG_SECTOR_COUNT, 0);
        self.write_reg(REG_LBA_LOW, 0);
        self.write_reg(REG_LBA_MID, 0);
        self.write_reg(REG_LBA_HIGH, 0);
        self.write_command(CMD_IDENTIFY);

        let status = self.read_status();
        if status == 0 || status == 0xFF {
            return Err(AtaError::NotPresent);
        }

        if self.read_reg(REG_LBA_MID) != 0 || self.read_reg(REG_LBA_HIGH) != 0 {
            return Err(AtaError::NotAta);
        }

        self.wait_for_drq()?;

        let mut words = [0u16; 256];
        for word in &mut words {
            *word = self.read_data();
        }

        Ok(words)
    }

    fn program_lba28(&self, lba: u32, sector_count: u8) -> Result<(), AtaError> {
        if lba > 0x0FFF_FFFF {
            return Err(AtaError::LbaOutOfRange);
        }

        self.wait_until_not_busy()?;
        self.write_reg(
            REG_DRIVE_HEAD,
            0xE0 | self.drive_bit() | (((lba >> 24) & 0x0F) as u8),
        );
        self.io_delay();
        self.write_reg(REG_SECTOR_COUNT, sector_count);
        self.write_reg(REG_LBA_LOW, lba as u8);
        self.write_reg(REG_LBA_MID, (lba >> 8) as u8);
        self.write_reg(REG_LBA_HIGH, (lba >> 16) as u8);

        Ok(())
    }

    fn program_lba48(&self, lba: u64, sector_count: u16) -> Result<(), AtaError> {
        if lba > 0x0000_FFFF_FFFF_FFFF {
            return Err(AtaError::LbaOutOfRange);
        }

        self.wait_until_not_busy()?;
        self.write_reg(REG_DRIVE_HEAD, 0xE0 | self.drive_bit());
        self.io_delay();

        self.write_reg(REG_SECTOR_COUNT, (sector_count >> 8) as u8);
        self.write_reg(REG_LBA_LOW, (lba >> 24) as u8);
        self.write_reg(REG_LBA_MID, (lba >> 32) as u8);
        self.write_reg(REG_LBA_HIGH, (lba >> 40) as u8);

        self.write_reg(REG_SECTOR_COUNT, sector_count as u8);
        self.write_reg(REG_LBA_LOW, lba as u8);
        self.write_reg(REG_LBA_MID, (lba >> 8) as u8);
        self.write_reg(REG_LBA_HIGH, (lba >> 16) as u8);

        Ok(())
    }

    fn wait_until_not_busy(&self) -> Result<u8, AtaError> {
        for _ in 0..POLL_LIMIT {
            let status = self.read_status();
            if status == 0xFF {
                return Err(AtaError::NotPresent);
            }
            if status & STATUS_BSY == 0 {
                return Ok(status);
            }
            spin_loop();
        }

        Err(AtaError::TimedOut)
    }

    fn wait_for_drq(&self) -> Result<(), AtaError> {
        for _ in 0..POLL_LIMIT {
            let status = self.read_status();
            if status == 0xFF {
                return Err(AtaError::NotPresent);
            }
            if status & STATUS_BSY != 0 {
                spin_loop();
                continue;
            }
            if status & STATUS_DF != 0 {
                return Err(AtaError::DeviceFault);
            }
            if status & STATUS_ERR != 0 {
                return Err(AtaError::ErrorRegister(self.read_reg(REG_ERROR)));
            }
            if status & STATUS_DRQ != 0 {
                return Ok(());
            }
            spin_loop();
        }

        Err(AtaError::TimedOut)
    }

    fn wait_for_command_completion(&self) -> Result<(), AtaError> {
        for _ in 0..POLL_LIMIT {
            let status = self.read_status();
            if status == 0xFF {
                return Err(AtaError::NotPresent);
            }
            if status & STATUS_BSY != 0 {
                spin_loop();
                continue;
            }
            if status & STATUS_DF != 0 {
                return Err(AtaError::DeviceFault);
            }
            if status & STATUS_ERR != 0 {
                return Err(AtaError::ErrorRegister(self.read_reg(REG_ERROR)));
            }
            if status & STATUS_DRQ != 0 {
                spin_loop();
                continue;
            }
            return Ok(());
        }

        Err(AtaError::TimedOut)
    }

    fn select_for_identify(&self) {
        self.write_reg(REG_DRIVE_HEAD, 0xA0 | self.drive_bit());
    }

    fn drive_bit(&self) -> u8 {
        if self.is_master { 0 } else { 0x10 }
    }

    fn io_delay(&self) {
        for _ in 0..4 {
            let _ = self.read_status();
        }
    }

    fn write_command(&self, value: u8) {
        self.write_reg(REG_STATUS_COMMAND, value);
    }

    fn read_status(&self) -> u8 {
        self.read_reg(REG_STATUS_COMMAND)
    }

    fn read_data(&self) -> u16 {
        unsafe {
            let mut port = Port::<u16>::new(self.base_port + REG_DATA);
            port.read()
        }
    }

    fn write_data(&self, value: u16) {
        unsafe {
            let mut port = Port::<u16>::new(self.base_port + REG_DATA);
            port.write(value);
        }
    }

    fn read_reg(&self, offset: u16) -> u8 {
        unsafe {
            let mut port = Port::<u8>::new(self.base_port + offset);
            port.read()
        }
    }

    fn write_reg(&self, offset: u16, value: u8) {
        unsafe {
            let mut port = Port::<u8>::new(self.base_port + offset);
            port.write(value);
        }
    }
}

pub fn detect() -> [Option<DetectedDrive>; 4] {
    let drives = [
        AtaDrive::new(ATA_PRIMARY_IO, true),
        AtaDrive::new(ATA_PRIMARY_IO, false),
        AtaDrive::new(ATA_SECONDARY_IO, true),
        AtaDrive::new(ATA_SECONDARY_IO, false),
    ];

    let mut detected = [None, None, None, None];
    for (index, drive) in drives.into_iter().enumerate() {
        if let Ok(info) = drive.detect() {
            detected[index] = Some(DetectedDrive { drive, info };
        }
    }
    detected
}

fn decode_ata_string(words: &[u16]) -> String {
    let mut value = String::with_capacity(words.len() * 2);
    for &word in words {
        let [hi, lo] = word.to_be_bytes();
        if hi != 0 {
            value.push(hi as char);
        }
        if lo != 0 {
            value.push(lo as char);
        }
    }

    String::from(value.trim_matches(|c| c == ' ' || c == '\0'))
)))}
