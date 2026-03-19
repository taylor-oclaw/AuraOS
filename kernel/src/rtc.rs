//! CMOS Real-Time Clock driver
//! 
//! Reads date/time from the PC CMOS RTC (ports 0x70/0x71).
//! This gives us wall-clock time without any network access.

use x86_64::instructions::port::Port;
use core::sync::atomic::{AtomicI8, Ordering};

/// Timezone offset from UTC in hours (e.g., -5 for EST, -4 for EDT)
/// Set during first boot or by user command
static TIMEZONE_OFFSET: AtomicI8 = AtomicI8::new(-5); // Default: EST

/// Timezone name
static mut TIMEZONE_NAME: [u8; 8] = *b"EST\0\0\0\0\0";

pub fn set_timezone(offset_hours: i8, name: &str) {
    TIMEZONE_OFFSET.store(offset_hours, Ordering::Relaxed);
    unsafe {
        let bytes = name.as_bytes();
        let len = bytes.len().min(7);
        TIMEZONE_NAME[..len].copy_from_slice(&bytes[..len]);
        TIMEZONE_NAME[len] = 0;
    }
}

pub fn timezone_offset() -> i8 {
    TIMEZONE_OFFSET.load(Ordering::Relaxed)
}

pub fn timezone_name() -> &'static str {
    unsafe {
        let ptr = core::ptr::addr_of!(TIMEZONE_NAME);
        let slice = &*ptr;
        let len = slice.iter().position(|&b| b == 0).unwrap_or(8);
        core::str::from_utf8_unchecked(&slice[..len])
    }
}

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

/// Read local time (RTC + timezone offset)
pub fn read_local_time() -> DateTime {
    let mut dt = read_rtc();
    let offset = timezone_offset();
    
    // Apply timezone offset to hours
    let mut hour = dt.hour as i16 + offset as i16;
    let mut day_delta = 0i8;
    
    if hour < 0 {
        hour += 24;
        day_delta = -1;
    } else if hour >= 24 {
        hour -= 24;
        day_delta = 1;
    }
    
    dt.hour = hour as u8;
    
    // Adjust day/weekday if we crossed midnight
    if day_delta != 0 {
        if day_delta > 0 {
            dt.day += 1;
            dt.weekday = if dt.weekday >= 7 { 1 } else { dt.weekday + 1 };
            // Simplified — doesn't handle month rollover
            let days_in_month = match dt.month {
                1|3|5|7|8|10|12 => 31,
                4|6|9|11 => 30,
                2 => if dt.year % 4 == 0 { 29 } else { 28 },
                _ => 30,
            };
            if dt.day > days_in_month {
                dt.day = 1;
                dt.month += 1;
                if dt.month > 12 { dt.month = 1; dt.year += 1; }
            }
        } else {
            if dt.day > 1 {
                dt.day -= 1;
            } else {
                dt.month = if dt.month > 1 { dt.month - 1 } else { 12 };
                dt.day = match dt.month {
                    1|3|5|7|8|10|12 => 31,
                    4|6|9|11 => 30,
                    2 => if dt.year % 4 == 0 { 29 } else { 28 },
                    _ => 30,
                };
                if dt.month == 12 { dt.year -= 1; }
            }
            dt.weekday = if dt.weekday <= 1 { 7 } else { dt.weekday - 1 };
        }
    }
    
    dt
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
