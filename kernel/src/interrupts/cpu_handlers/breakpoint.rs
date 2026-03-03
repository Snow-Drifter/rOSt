use internal_utils::logln;
use x86_64::structures::idt::InterruptStackFrame;

use crate::hlt_loop_hard;

pub extern "x86-interrupt" fn breakpoint_handler(_: InterruptStackFrame) {
    logln!("BREAKPOINT");
    hlt_loop_hard();
}
