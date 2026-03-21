pub trait KeyboardLayout {
    fn translate(&self, scancode: u8, shifted: bool) -> Option<char>;
}

pub struct UsLayout;

impl KeyboardLayout for UsLayout {
    fn translate(&self, scancode: u8, shifted: bool) -> Option<char> {
        match (scancode, shifted) {
            (0x02, false) => Some('1'),
            (0x03, false) => Some('2'),
            (0x04, false) => Some('3'),
            (0x05, false) => Some('4'),
            (0x06, false) => Some('5'),
            (0x07, false) => Some('6'),
            (0x08, false) => Some('7'),
            (0x09, false) => Some('8'),
            (0x0A, false) => Some('9'),
            (0x0B, false) => Some('0'),
            (0x1E, false) => Some('a'),
            (0x30, false) => Some('b'),
            (0x2E, false) => Some('c'),
            (0x20, false) => Some('d'),
            (0x12, false) => Some('e'),
            (0x21, false) => Some('f'),
            (0x22, false) => Some('g'),
            (0x23, false) => Some('h'),
            (0x17, false) => Some('i'),
            (0x24, false) => Some('j'),
            (0x25, false) => Some('k'),
            (0x26, false) => Some('l'),
            (0x32, false) => Some('m'),
            (0x31, false) => Some('n'),
            (0x18, false) => Some('o'),
            (0x19, false) => Some('p'),
            (0x10, false) => Some('q'),
            (0x13, false) => Some('r'),
            (0x1F, false) => Some('s'),
            (0x14, false) => Some('t'),
            (0x16, false) => Some('u'),
            (0x2F, false) => Some('v'),
            (0x1A, false) => Some('w'),
            (0x2D, false) => Some('x'),
            (0x1C, false) => Some('y'),
            (0x1B, false) => Some('z'),
            (0x39, false) => Some(' '),
            (0x1C, true) => Some('\n'), // Enter
            (0x0E, _) => Some('\x08'),  // Backspace
            (0x0F, _) => Some('\t'),  // Tab

            // Shifted variants
            (0x02, true) => Some('!'),
            (0x03, true) => Some('@'),
            (0x04, true) => Some('#'),
            (0x05, true) => Some('$'),
            (0x06, true) => Some('%'),
            (0x07, true) => Some('^'),
            (0x08, true) => Some('&'),
            (0x09, true) => Some('*'),
            (0x0A, true) => Some('('),
            (0x0B, true) => Some(')'),
            (0x1E, true) => Some('A'),
            (0x30, true) => Some('B'),
            (0x2E, true) => Some('C'),
            (0x20, true) => Some('D'),
            (0x12, true) => Some('E'),
            (0x21, true) => Some('F'),
            (0x22, true) => Some('G'),
            (0x23, true) => Some('H'),
            (0x17, true) => Some('I'),
            (0x24, true) => Some('J'),
            (0x25, true) => Some('K'),
            (0x26, true) => Some('L'),
            (0x32, true) => Some('M'),
            (0x31, true) => Some('N'),
            (0x18, true) => Some('O'),
            (0x19, true) => Some('P'),
            (0x10, true) => Some('Q'),
            (0x13, true) => Some('R'),
            (0x1F, true) => Some('S'),
            (0x14, true) => Some('T'),
            (0x16, true) => Some('U'),
            (0x2F, true) => Some('V'),
            (0x1A, true) => Some('W'),
            (0x2D, true) => Some('X'),
            (0x1C, true) => Some('Y'),
            (0x1B, true) => Some('Z'),

            _ => None,
        }
    }
}

pub struct LayoutManager {
    current: u8,
}

impl LayoutManager {
    pub fn new() -> Self {
        LayoutManager { current: 0 } // Assuming 0 represents US layout
    }

    pub fn translate(&self, scancode: u8, shifted: bool) -> Option<char> {
        match self.current {
            0 => UsLayout.translate(scancode, shifted),
            _ => None,
        }
    }
}
