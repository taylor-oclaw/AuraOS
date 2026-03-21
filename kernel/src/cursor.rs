extern crate alloc;

#[derive(Clone, Copy)]
pub enum CursorStyle {
    Arrow,
    Hand,
    Text,
    Wait,
    Crosshair,
}

const ARROW_BITMAP: [u16; 16] = [
    0b1000000000000000,
    0b1100000000000000,
    0b1110000000000000,
    0b1111000000000000,
    0b1111100000000000,
    0b1111110000000000,
    0b1111111000000000,
    0b1111100000000000,
    0b1101100000000000,
    0b1000110000000000,
    0b0000110000000000,
    0b0000011000000000,
    0b0000011000000000,
    0b0000000000000000,
    0b0000000000000000,
    0b0000000000000000,
];

pub fn draw_cursor(buf: &mut [u32], stride: usize, x: usize, y: usize, _style: &CursorStyle, color: u32) {
    let bitmap = &ARROW_BITMAP;
    for row in 0..16 {
        if y + row >= buf.len() / stride { break; }
        for col in 0..16 {
            if x + col >= stride { break; }
            if (bitmap[row] >> (15 - col)) & 1 == 1 {
                let idx = (y + row) * stride + (x + col);
                if idx < buf.len() {
                    buf[idx] = color;
                }
            }
        }
    }
}
