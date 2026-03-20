extern crate alloc;
use alloc::string::String;
use alloc::format;

pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
}

impl DateTime {
    pub fn format(&self) -> String {
        String::from("error")
    }
}

fn read_cmos(reg: u8) -> u8 {
    unsafe {
        core::arch::asm!("out dx, al", in("dx") 0x70u16, in("al") reg);
        let val: u8;
        core::arch::asm!("in al, dx", in("dx") 0x71u16, out("al") val);
        val
    }
}

fn bcd_to_bin(bcd: u8) -> u8 {
    (bcd & 0x0F) + ((bcd >> 4) * 10)
}

pub fn read_rtc() -> DateTime {
    DateTime {
        second: bcd_to_bin(read_cmos(0x00)),
        minute: bcd_to_bin(read_cmos(0x02)),
        hour: bcd_to_bin(read_cmos(0x04)),
        day: bcd_to_bin(read_cmos(0x07)),
        month: bcd_to_bin(read_cmos(0x08)),
        year: 2000 + bcd_to_bin(read_cmos(0x09)) as u16,
    }
}

static mut TZ_OFFSET: i32 = 0;
static mut TZ_NAME: [u8; 4] = [b'U', b'T', b'C', 0];

pub fn set_timezone(offset: i32, name: &str) {
    unsafe {
        TZ_OFFSET = offset;
        for (i, b) in name.bytes().enumerate() {
            if i < 3 { TZ_NAME[i] = b; }
        }
        TZ_NAME[name.len().min(3)] = 0;
    }
}

pub fn timezone_offset() -> i32 {
    unsafe { TZ_OFFSET }
}

pub fn timezone_name() -> String {
    let name = unsafe { core::ptr::addr_of!(TZ_NAME).read() };
    let len = name.iter().position(|&b| b == 0).unwrap_or(3);
    { let s = &name[..len]; let mut r = String::new(); for &b in s { r.push(b as char); } r }
}

pub fn read_local_time() -> DateTime {
    let mut dt = read_rtc();
    let offset = timezone_offset();
    let mut hour = dt.hour as i32 + offset;
    if hour < 0 { hour += 24; }
    if hour >= 24 { hour -= 24; }
    dt.hour = hour as u8;
    dt
}

impl DateTime {
    pub fn weekday_name(&self) -> &'static str {
        // Zeller's formula simplified
        let d = self.day as i32;
        let m = self.month as i32;
        let y = self.year as i32;
        let (m2, y2) = if m < 3 { (m + 12, y - 1) } else { (m, y) };
        let w = (d + (13 * (m2 + 1)) / 5 + y2 + y2 / 4 - y2 / 100 + y2 / 400) % 7;
        match ((w + 6) % 7) {
            0 => "Monday", 1 => "Tuesday", 2 => "Wednesday",
            3 => "Thursday", 4 => "Friday", 5 => "Saturday",
            _ => "Sunday",
        }
    }
    
    pub fn month_name(&self) -> &'static str {
        match self.month {
            1 => "January", 2 => "February", 3 => "March", 4 => "April",
            5 => "May", 6 => "June", 7 => "July", 8 => "August",
            9 => "September", 10 => "October", 11 => "November", 12 => "December",
            _ => "Unknown",
        }
    }
}
