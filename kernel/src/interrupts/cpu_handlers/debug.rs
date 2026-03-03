use internal_utils::logln;
use x86_64::structures::idt::InterruptStackFrame;

use crate::hlt_loop_hard;

pub extern "x86-interrupt" fn debug_handler(_: InterruptStackFrame) {
    logln!("DEBUG");
    hlt_loop_hard();
}
