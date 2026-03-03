mod debug;
mod frame_allocator;
mod heap;
mod memory_init;
mod page_table;
use alloc::sync::Arc;
use bootloader_api::BootInfo;
use internal_utils::kernel_information::frame_allocator::{FullFrameAllocator, print_memory};
use memory_init::init_page_tables;

use spin::{Mutex, Once};
use x86_64::{
    registers::control::{Cr3, Cr3Flags},
    structures::paging::PhysFrame,
};

use crate::memory::{
    debug::print_memory_map, frame_allocator::BitmapFrameAllocator, heap::init_heap,
    page_table::MEMORY_MAPPER,
};

static KERNEL_CR3: Once<(PhysFrame, Cr3Flags)> = Once::new();

/// Saves the current paging table used as the kernel's paging table.
pub fn init_kernel_memory(
    boot_info: &'static BootInfo,
) -> Arc<Mutex<dyn FullFrameAllocator + Send + Sync>> {
    KERNEL_CR3.call_once(x86_64::registers::control::Cr3::read);
    print_memory_map(&boot_info.memory_regions);

    let mut allocator = BitmapFrameAllocator::init(boot_info);
    init_page_tables(boot_info);

    let mut mapper = MEMORY_MAPPER.lock();
    init_heap(mapper.as_mut().unwrap(), &mut allocator).expect("heap initialization failed");

    let allocator = Arc::new(Mutex::new(allocator));

    print_memory(allocator.clone());

    allocator
}

/// Switches the paging table used to the kernel's paging table.
fn switch_to_kernel_memory() {
    let kernel_cr3 = KERNEL_CR3.get();
    if let Some(guard) = kernel_cr3 {
        unsafe {
            Cr3::write(guard.0, guard.1);
        }
    }
}

/// Performs an action while having kernel paging table. Then switches back.
pub fn with_kernel_memory<V>(action: impl FnOnce() -> V) -> V {
    let cr3 = Cr3::read();
    switch_to_kernel_memory();
    let result = action();
    unsafe {
        Cr3::write(cr3.0, cr3.1);
    }
    result
}
