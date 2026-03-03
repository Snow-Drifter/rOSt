use core::arch::naked_asm;

use alloc::sync::Arc;
use internal_utils::clocks::get_current_tick;
use spin::Mutex;
use x86_64::registers::control::Cr3;
use x86_64::structures::paging::PhysFrame;

use crate::interrupts::GDT;
use crate::processes::registers_state::Flags;
use crate::unpack_registers_state;

use super::RegistersState;
use super::thread::Thread;

/// Runs the thread immediately.
pub fn dispatch_thread(thread: Arc<Mutex<Thread>>) -> ! {
    let code_selector_id: u16;
    let data_selector_id: u16;
    let cr3: (PhysFrame, u16);
    let state: RegistersState;
    x86_64::instructions::interrupts::disable();
    {
        let tick = get_current_tick();
        let mut thread_mut = thread.lock();
        thread_mut.last_tick = tick;
        let mut process = thread_mut.process.lock();
        process.last_tick = tick;
        code_selector_id = if process.kernel_process {
            GDT.1.kernel_code_selector.0
        } else {
            GDT.1.user_code_selector.0
        };
        data_selector_id = if process.kernel_process {
            GDT.1.kernel_data_selector.0
        } else {
            GDT.1.user_data_selector.0
        };
        cr3 = process.cr3;
        state = thread_mut.registers_state.clone();
    }

    let flags = (state.rflags | Flags::IF) & Flags::NT.complement();
    unsafe {
        // We decrement the counter forcefully because that function doesn't return by Rust.
        Arc::decrement_strong_count(Arc::into_raw(thread));
        Cr3::write_raw(cr3.0, cr3.1);
        switch_to_thread(
            code_selector_id as u64,
            data_selector_id as u64,
            state.rip.as_u64(),
            state.rsp.as_u64(),
            flags.bits(),
            &state as *const RegistersState as *const u8,
        );
    }
}

#[unsafe(no_mangle)]
#[unsafe(naked)]
unsafe extern "sysv64" fn switch_to_thread(
    code_selector_id: u64, // rdi
    data_selector_id: u64, // rsi
    rip: u64,              // rdx
    rsp: u64,              // rcx
    flags: u64,            // r8
    state: *const u8,      // r9
) -> ! {
    naked_asm!(
        "push rsi",
        "push rcx",
        "push r8",                 // rflags
        "push rdi",                // code selector
        "push rdx",                // instruction address to return to
        unpack_registers_state!(), // Loading registers (RAX-R15) before jumping into thread
        "iretq",                   // Let's go!
    )
}
