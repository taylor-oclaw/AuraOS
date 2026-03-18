//! Intel e1000/e1000e Ethernet driver
//! This is the primary ethernet driver — works in QEMU and most Intel hardware.

/// e1000 register offsets
#[allow(dead_code)]
mod regs {
    pub const CTRL: u32 = 0x0000;      // Device Control
    pub const STATUS: u32 = 0x0008;    // Device Status
    pub const EERD: u32 = 0x0014;      // EEPROM Read
    pub const ICR: u32 = 0x00C0;       // Interrupt Cause Read
    pub const IMS: u32 = 0x00D0;       // Interrupt Mask Set
    pub const IMC: u32 = 0x00D8;       // Interrupt Mask Clear
    pub const RCTL: u32 = 0x0100;      // Receive Control
    pub const TCTL: u32 = 0x0400;      // Transmit Control
    pub const RDBAL: u32 = 0x2800;     // RX Descriptor Base Low
    pub const RDBAH: u32 = 0x2804;     // RX Descriptor Base High
    pub const RDLEN: u32 = 0x2808;     // RX Descriptor Length
    pub const RDH: u32 = 0x2810;       // RX Descriptor Head
    pub const RDT: u32 = 0x2818;       // RX Descriptor Tail
    pub const TDBAL: u32 = 0x3800;     // TX Descriptor Base Low
    pub const TDBAH: u32 = 0x3804;     // TX Descriptor Base High
    pub const TDLEN: u32 = 0x3808;     // TX Descriptor Length
    pub const TDH: u32 = 0x3810;       // TX Descriptor Head
    pub const TDT: u32 = 0x3818;       // TX Descriptor Tail
    pub const RAL: u32 = 0x5400;       // Receive Address Low
    pub const RAH: u32 = 0x5404;       // Receive Address High
    pub const MTA: u32 = 0x5200;       // Multicast Table Array
}

/// e1000 control register bits
#[allow(dead_code)]
mod ctrl {
    pub const SLU: u32 = 1 << 6;      // Set Link Up
    pub const RST: u32 = 1 << 26;     // Device Reset
}

/// RX descriptor (hardware format)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct RxDescriptor {
    pub buffer_addr: u64,
    pub length: u16,
    pub checksum: u16,
    pub status: u8,
    pub errors: u8,
    pub special: u16,
}

/// TX descriptor (hardware format)
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub struct TxDescriptor {
    pub buffer_addr: u64,
    pub length: u16,
    pub cso: u8,
    pub cmd: u8,
    pub status: u8,
    pub css: u8,
    pub special: u16,
}

/// The e1000 driver instance
pub struct E1000Driver {
    mmio_base: usize,
    mac_address: [u8; 6],
    link_up: bool,
}

impl E1000Driver {
    /// Read a register from MMIO space
    fn read_reg(&self, offset: u32) -> u32 {
        let addr = (self.mmio_base + offset as usize) as *const u32;
        unsafe { core::ptr::read_volatile(addr) }
    }
    
    /// Write a register to MMIO space
    fn write_reg(&self, offset: u32, value: u32) {
        let addr = (self.mmio_base + offset as usize) as *mut u32;
        unsafe { core::ptr::write_volatile(addr, value) }
    }
    
    /// Read MAC address from EEPROM
    pub fn mac_address(&self) -> [u8; 6] {
        self.mac_address
    }
    
    /// Check if link is up
    pub fn is_link_up(&self) -> bool {
        let status = self.read_reg(regs::STATUS);
        status & 0x02 != 0 // Link Up bit
    }
}
