use internal_utils::logln;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::structures::idt::PageFaultErrorCode;

use crate::hlt_loop_hard;

pub extern "x86-interrupt" fn page_fault_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;
    x86_64::instructions::interrupts::disable();

    logln!("PAGE FAULT (error {:#?})\n{:#?}", _error_code, _stack_frame);
    logln!("Page: {:X?}", Cr2::read_raw());
    hlt_loop_hard();
}
