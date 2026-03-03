use alloc::vec::Vec;
use internal_utils::logln;

pub struct SchedulerTable(pub Vec<ProcessInfo>);

pub struct ProcessInfo {
    pub id: u64,
    pub kernel_process: bool,
    pub load: u64,
    pub threads: Vec<ThreadInfo>,
}

pub struct ThreadInfo {
    pub id: u64,
    pub state: &'static str,
    pub load: u64,
}

impl SchedulerTable {
    pub fn log(&self) {
        logln!(" Process   | Thread    | CPU Usage | State     ");
        for p in self.0.iter() {
            logln!(
                "{: >10} |           | {: >8}% | {: >9} ",
                p.id,
                p.load,
                if p.kernel_process { "ring 0" } else { "ring 3" }
            );
            for t in p.threads.iter() {
                logln!(
                    "           | {: >9} | {: >8}% |{: >11}",
                    t.id,
                    t.load,
                    t.state
                );
            }
        }
    }
}
