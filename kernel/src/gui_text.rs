//! GUI Text Renderer — draws text inside surfaces using the bitmap font

/// Draw a string at pixel position (x, y) in a framebuffer region
pub fn draw_text(
    fb: &mut [u8], stride: usize, bpp: usize,
    text: &str, start_x: i32, start_y: i32,
    r: u8, g: u8, b: u8,
    screen_width: usize, screen_height: usize,
) {
    let mut x = start_x;
    let y = start_y;
    
    for byte in text.bytes() {
        match byte {
            b'\n' => {
                // Newline handling would need y tracking — skip for now
                return;
            }
            0x20..=0x7e => {
                draw_char(fb, stride, bpp, byte, x, y, r, g, b, screen_width, screen_height);
                x += 8; // Character width
            }
            _ => {
                x += 8;
            }
        }
    }
}

/// Draw multiple lines of text
pub fn draw_text_block(
    fb: &mut [u8], stride: usize, bpp: usize,
    lines: &[&str], start_x: i32, start_y: i32,
    r: u8, g: u8, b: u8,
    screen_width: usize, screen_height: usize,
) {
    for (i, line) in lines.iter().enumerate() {
        let y = start_y + (i as i32 * 18); // 16px char + 2px spacing
        if y >= screen_height as i32 { break; }
        draw_text(fb, stride, bpp, line, start_x, y, r, g, b, screen_width, screen_height);
    }
}

fn draw_char(
    fb: &mut [u8], stride: usize, bpp: usize,
    c: u8, x: i32, y: i32,
    r: u8, g: u8, b: u8,
    screen_width: usize, screen_height: usize,
) {
    let glyph = crate::framebuffer::font_data::get_glyph(c);
    
    for dy in 0..16usize {
        let py = y + dy as i32;
        if py < 0 || py >= screen_height as i32 { continue; }
        let row_bits = glyph[dy];
        for dx in 0..8usize {
            if (row_bits >> (7 - dx)) & 1 == 1 {
                let px = x + dx as i32;
                if px < 0 || px >= screen_width as i32 { continue; }
                let offset = (py as usize * stride + px as usize) * bpp;
                if offset + 2 < fb.len() {
                    fb[offset] = b;
                    fb[offset + 1] = g;
                    fb[offset + 2] = r;
                }
            }
        }
    }
}
