use x86_64::structures::idt::InterruptStackFrame;

/// # Safety
///
/// This should never do stack heavy operations because this handler has a separate stack that has no stack guard page and thus could corrupt the data after it.
pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!(
        "DOUBLE FAULT (error {:#?})\n{:#?}",
        _error_code, stack_frame
    );
}
