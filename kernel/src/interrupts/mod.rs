// This might be reimplemented from scratch in the future.

// TODO: Implement all remaining interrupt handlers for CPU interrupts
// We need to implement all interrupt handlers and add basic handling to them so we don't double fault.
// Better handling for each of them will be added later.

mod cpu_handlers;
mod interrupt_register;
use internal_utils::logln;
pub(crate) mod gdt;
mod pic_handlers;
pub use gdt::GDT;
mod pic;

use crate::interrupts::{
    gdt::reload_gdt,
    interrupt_register::init_idt,
    pic::{PICS, Pics},
};

/// Initializes the GDT, IDT and PIC controllers
pub fn setup() {
    reload_gdt();
    init_idt();
    PICS.initialize();
    logln!("Interrupts set up");
}
