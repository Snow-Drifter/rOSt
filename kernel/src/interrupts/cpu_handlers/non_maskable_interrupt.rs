use x86_64::structures::idt::InterruptStackFrame;

/// # Safety
///
/// This should never do stack heavy operations because this handler has a separate stack that has no stack guard page and thus could corrupt the data after it.
pub extern "x86-interrupt" fn nmi_handler(stack_frame: InterruptStackFrame) {
    panic!("NMI\n{:#?}", stack_frame);
}
