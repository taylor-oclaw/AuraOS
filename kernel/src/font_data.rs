// Minimal 8x16 bitmap font for ASCII 32-126
// Each glyph is 16 bytes (one byte per row of 8 pixels)

static FONT_DATA: &[u8] = include_bytes!("font.bin");

pub fn get_glyph(c: u8) -> &'static [u8] {
    let idx = if c >= 32 && c <= 126 { (c - 32) as usize } else { 0 };
    let start = idx * 16;
    let end = start + 16;
    if end <= FONT_DATA.len() {
        &FONT_DATA[start..end]
    } else {
        &FONT_DATA[0..16] // Space as fallback
    }
}
