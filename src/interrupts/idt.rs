//! TODO(BSFishy): document this
//! NOTE: it is important to keep in mind possible deadlocks in the functions in this file.
//! TODO: convert this module into an abstraction layer in front of architecture-specific code

use crate::{
    gdt,
    interrupts::{hlt_loop, pic},
};
use lazy_static::lazy_static;


use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame, PageFaultErrorCode};

#[cfg(debug_assertions)]
use sanders_vga_buffer::println;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = { // TODO: handle all the other interrupts
        let mut idt = InterruptDescriptorTable::new();
        prepare_idt(&mut idt);
        pic::prepare_idt(&mut idt);

        idt
    };
}

/// TODO(BSFishy): document this
pub fn init_idt() {
    IDT.load();
}

fn prepare_idt(idt: &mut InterruptDescriptorTable) {
    idt.breakpoint.set_handler_fn(breakpoint_handler);

    unsafe {
        idt.double_fault
            .set_handler_fn(double_fault_handler)
            .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
    }
}

#[allow(unused_variables)]
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    #[cfg(debug_assertions)]
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

#[allow(unused_variables)]
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    #[cfg(debug_assertions)]
    {
        use x86_64::registers::control::Cr2;

        println!("EXCEPTION: PAGE FAULT");
        println!("Accessed Address: {:?}", Cr2::read());
        println!("Error Code: {:?}", error_code);
        println!("{:#?}", stack_frame);
    }

    hlt_loop();
}

#[test_case]
fn test_breakpoint_exception() {
    // invoke a breakpoint exception
    x86_64::instructions::interrupts::int3();
}
