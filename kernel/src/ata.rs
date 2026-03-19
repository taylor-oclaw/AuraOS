extern crate alloc;
use alloc::vec::Vec;

pub const ATA_PRIMARY: u16 = 0x1F0;
pub const ATA_DATA: u16 = ATA_PRIMARY;
pub const ATA_SECTOR_COUNT: u16 = ATA_PRIMARY + 2;
pub const ATA_LBA_LO: u16 = ATA_PRIMARY + 3;
pub const ATA_LBA_MID: u16 = ATA_PRIMARY + 4;
pub const ATA_LBA_HI: u16 = ATA_PRIMARY + 5;
pub const ATA_DRIVE: u16 = ATA_PRIMARY + 6;
pub const ATA_COMMAND: u16 = ATA_PRIMARY + 7;
pub const ATA_STATUS: u16 = ATA_PRIMARY + 7;

fn outb(port: u16, val: u8) {
    unsafe { core::arch::asm!("out dx, al", in("dx") port, in("al") val); }
}

fn inb(port: u16) -> u8 {
    let val: u8;
    unsafe { core::arch::asm!("in al, dx", in("dx") port, out("al") val); }
    val
}

fn inw(port: u16) -> u16 {
    let val: u16;
    unsafe { core::arch::asm!("in ax, dx", in("dx") port, out("ax") val); }
    val
}

pub fn wait_ready() {
    while inb(ATA_STATUS) & 0x80 != 0 {}
}

pub fn read_sector(lba: u32, buf: &mut [u16; 256]) {
    outb(ATA_DRIVE, 0xE0 | ((lba >> 24) as u8 & 0x0F));
    outb(ATA_SECTOR_COUNT, 1);
    outb(ATA_LBA_LO, lba as u8);
    outb(ATA_LBA_MID, (lba >> 8) as u8);
    outb(ATA_LBA_HI, (lba >> 16) as u8);
    outb(ATA_COMMAND, 0x20);
    wait_ready();
    for i in 0..256 {
        buf[i] = inw(ATA_DATA);
    }
}

pub fn write_sector(lba: u32, buf: &[u16; 256]) {
    outb(ATA_DRIVE, 0xE0 | ((lba >> 24) as u8 & 0x0F));
    outb(ATA_SECTOR_COUNT, 1);
    outb(ATA_LBA_LO, lba as u8);
    outb(ATA_LBA_MID, (lba >> 8) as u8);
    outb(ATA_LBA_HI, (lba >> 16) as u8);
    outb(ATA_COMMAND, 0x30);
    wait_ready();
    for i in 0..256 {
        unsafe { core::arch::asm!("out dx, ax", in("dx") ATA_DATA, in("ax") buf[i]); }
    }
}
