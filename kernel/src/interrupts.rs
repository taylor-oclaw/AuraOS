use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
// PIC handled inline
use spin::Mutex;
#[allow(unused_imports)]

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub struct ChainedPics {
    offset1: u8,
    offset2: u8,
}

impl ChainedPics {
    pub const unsafe fn new(offset1: u8, offset2: u8) -> Self {
        Self { offset1, offset2 }
    }

    pub unsafe fn initialize(&mut self) {
        // ICW1: Initialize + ICW4 needed
        core::arch::asm!("out dx, al", in("dx") 0x20u16, in("al") 0x11u8);
        core::arch::asm!("out dx, al", in("dx") 0xA0u16, in("al") 0x11u8);
        // ICW2: Offsets
        core::arch::asm!("out dx, al", in("dx") 0x21u16, in("al") self.offset1);
        core::arch::asm!("out dx, al", in("dx") 0xA1u16, in("al") self.offset2);
        // ICW3
        core::arch::asm!("out dx, al", in("dx") 0x21u16, in("al") 4u8);
        core::arch::asm!("out dx, al", in("dx") 0xA1u16, in("al") 2u8);
        // ICW4: 8086 mode
        core::arch::asm!("out dx, al", in("dx") 0x21u16, in("al") 0x01u8);
        core::arch::asm!("out dx, al", in("dx") 0xA1u16, in("al") 0x01u8);
    }

    pub unsafe fn notify_end_of_interrupt(&mut self, interrupt_id: u8) {
        if interrupt_id >= self.offset2 {
            // Send EOI to slave PIC (port 0xA0)
            core::arch::asm!("out dx, al", in("dx") 0xA0u16, in("al") 0x20u8);
        }
        // Send EOI to master PIC (port 0x20)
        core::arch::asm!("out dx, al", in("dx") 0x20u16, in("al") 0x20u8);
    }
}

pub static PICS: Mutex<ChainedPics> = Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
#[allow(dead_code)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard = PIC_1_OFFSET + 1,
}

static IDT: spin::Lazy<InterruptDescriptorTable> = spin::Lazy::new(|| {
    let mut idt = InterruptDescriptorTable::new();

    idt.breakpoint.set_handler_fn(breakpoint_handler);
    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(crate::gdt::DOUBLE_FAULT_IST_INDEX);
    }

    idt[InterruptIndex::Keyboard as u8 as usize].set_handler_fn(keyboard_interrupt_handler);

    idt
});

pub fn init_idt() {
    IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    crate::serial_println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    crate::serial_println!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
    loop { x86_64::instructions::hlt(); }
}

extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // ABSOLUTE MINIMUM: read scancode, buffer it, EOI
    // All through raw port I/O, no abstractions
    unsafe {
        // Read scancode from keyboard controller
        let scancode: u8;
        core::arch::asm!("in al, dx", out("al") scancode, in("dx") 0x60u16);
        
        // Buffer it
        crate::keyboard::push_key(scancode);
        
        // Send EOI to PIC1
        core::arch::asm!("out dx, al", in("al") 0x20u8, in("dx") 0x20u16);
    }
}
