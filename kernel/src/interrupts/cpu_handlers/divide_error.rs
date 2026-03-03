use internal_utils::logln;
use x86_64::structures::idt::InterruptStackFrame;

use crate::hlt_loop_hard;

pub extern "x86-interrupt" fn divide_error_handler(_stack_frame: InterruptStackFrame) {
    logln!("DIVIDE ERROR\n{:#?}", _stack_frame);
    hlt_loop_hard();
}
