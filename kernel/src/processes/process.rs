use alloc::sync::Arc;
use internal_utils::clocks::get_current_tick;
use spin::Mutex;
use x86_64::{registers::control::Cr3, structures::paging::PhysFrame};

use alloc::vec::Vec;

use super::thread::{Thread, ThreadState};

#[derive(Debug)]
pub struct Process {
    /// The process's ID.
    pub id: u64,
    /// The page table the process is using.
    pub cr3: (PhysFrame, u16),
    /// Total ticks the process has been running for.
    pub total_ticks: u64,
    /// The tick the process has been created on.
    pub start_tick: u64,
    /// The tick the process has been last ran on.
    pub last_tick: u64,
    /// Is the process a kernel process (should it run in ring 0 or 3?).
    pub kernel_process: bool,
    /// A count of all threads ever created by this process.
    pub total_threads_created: u64,
    /// The threads of the process that have not started yet.
    pub not_started_threads: Vec<Arc<Mutex<Thread>>>,
    /// The threads of the process that are eligible to run.
    pub ready_threads: Vec<Arc<Mutex<Thread>>>,
    /// The threads of the process that are sleeping.
    pub sleeping_threads: Vec<Arc<Mutex<Thread>>>,
}

impl Process {
    /// Returns the percentage of ticks the process spent running, calculated from the creation time of the process
    pub fn tick_density(&self, current_tick: u64) -> u64 {
        let ticks_maximum = current_tick.saturating_sub(self.start_tick).max(1);
        self.total_ticks * 100 / ticks_maximum
    }

    /// Creates a new process in kernel space.
    ///
    // TODO: Loading the process from e.g. an ELF file
    // We have to look up the structure of an ELF file and prepare the user memory mapping according to it.
    // Then we can load the program and it's data to proper places and create a process out of it.
    pub fn create_blank(id: u64) -> Self {
        Process {
            id,
            cr3: Cr3::read_raw(),
            total_ticks: 0,
            start_tick: get_current_tick(),
            last_tick: 0,
            total_threads_created: 0,
            kernel_process: true,
            not_started_threads: Vec::new(),
            ready_threads: Vec::new(),
            sleeping_threads: Vec::new(),
        }
    }

    /// Updates the sleeping threads, waking them up if they are sleeping for too long.
    pub fn update_sleeping_threads(this: &Arc<Mutex<Process>>) {
        let mut process = this.lock();
        if process.sleeping_threads.is_empty() {
            return;
        }
        let mut drained = Vec::new();
        process.sleeping_threads.retain(|thread| {
            let mut borrowed_thread = thread.lock();
            match borrowed_thread.state {
                ThreadState::Sleeping(ref mut sleep_ticks) => {
                    if *sleep_ticks > 0 {
                        *sleep_ticks -= 1;
                        true
                    } else {
                        borrowed_thread.state = ThreadState::Ready;
                        drained.push(thread.clone());
                        false
                    }
                }
                _ => unreachable!(),
            }
        });
        process.ready_threads.extend(drained);
    }
}
