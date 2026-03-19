//! CMOS Real-Time Clock driver
//! 
//! Reads date/time from the PC CMOS RTC (ports 0x70/0x71).
//! This gives us wall-clock time without any network access.

use x86_64::instructions::port::Port;

#[derive(Debug, Clone, Copy)]
pub struct DateTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub weekday: u8, // 1=Sunday, 7=Saturday
}

/// Read the current date/time from the CMOS RTC
pub fn read_rtc() -> DateTime {
    // Wait for update-not-in-progress
    while is_updating() {}
    
    let second = read_cmos(0x00);
    let minute = read_cmos(0x02);
    let hour = read_cmos(0x04);
    let weekday = read_cmos(0x06);
    let day = read_cmos(0x07);
    let month = read_cmos(0x08);
    let year_low = read_cmos(0x09);
    
    // Read register B to check format
    let reg_b = read_cmos(0x0B);
    let is_bcd = reg_b & 0x04 == 0;
    let is_24h = reg_b & 0x02 != 0;
    
    let (second, minute, mut hour, day, month, year_low) = if is_bcd {
        (
            bcd_to_bin(second),
            bcd_to_bin(minute),
            bcd_to_bin(hour & 0x7F),
            bcd_to_bin(day),
            bcd_to_bin(month),
            bcd_to_bin(year_low),
        )
    } else {
        (second, minute, hour & 0x7F, day, month, year_low)
    };
    
    // Handle 12-hour format
    if !is_24h && (hour & 0x80 != 0) {
        hour = ((hour & 0x7F) + 12) % 24;
    }
    
    // Assume 2000s
    let year = 2000u16 + year_low as u16;
    
    DateTime {
        year,
        month,
        day,
        hour,
        minute,
        second,
        weekday,
    }
}

fn is_updating() -> bool {
    read_cmos(0x0A) & 0x80 != 0
}

fn read_cmos(reg: u8) -> u8 {
    unsafe {
        let mut addr_port: Port<u8> = Port::new(0x70);
        let mut data_port: Port<u8> = Port::new(0x71);
        addr_port.write(reg);
        data_port.read()
    }
}

fn bcd_to_bin(bcd: u8) -> u8 {
    (bcd >> 4) * 10 + (bcd & 0x0F)
}

impl DateTime {
    pub fn weekday_name(&self) -> &'static str {
        match self.weekday {
            1 => "Sunday",
            2 => "Monday",
            3 => "Tuesday",
            4 => "Wednesday",
            5 => "Thursday",
            6 => "Friday",
            7 => "Saturday",
            _ => "Unknown",
        }
    }
    
    pub fn month_name(&self) -> &'static str {
        match self.month {
            1 => "January", 2 => "February", 3 => "March",
            4 => "April", 5 => "May", 6 => "June",
            7 => "July", 8 => "August", 9 => "September",
            10 => "October", 11 => "November", 12 => "December",
            _ => "Unknown",
        }
    }
}
