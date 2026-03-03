use internal_utils::logln;
use x86_64::structures::idt::InterruptStackFrame;

use crate::hlt_loop_hard;

pub extern "x86-interrupt" fn alignment_check_handler(
    _stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    logln!(
        "ALIGNMENT CHECK (error {:#?})\n{:#?}",
        _error_code,
        _stack_frame
    );
    hlt_loop_hard();
}