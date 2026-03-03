use bitflags::bitflags;
use x86_64::VirtAddr;

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Flags: u64 {
        /// FLAGS

        /// Carry Flag
        const CF = 0b00000000000000000000000000000001;
        /// Reserved
        const R1 = 0b00000000000000000000000000000010;
        /// Parity Flag
        const PF = 0b00000000000000000000000000000100;
        /// Reserved
        const R2 = 0b00000000000000000000000000001000;
        /// Auxiliary Carry
        const AF = 0b00000000000000000000000000010000;
        /// Reserved
        const R3 = 0b00000000000000000000000000100000;
        /// Zero Flag
        const ZF = 0b00000000000000000000000001000000;
        /// Sign Flag
        const SF = 0b00000000000000000000000010000000;
        /// Trap Flag (single step)
        const TF = 0b00000000000000000000000100000000;
        /// Interrupt Enable Flag
        const IF = 0b00000000000000000000001000000000;
        /// Direction Flag
        const DF = 0b00000000000000000000010000000000;
        /// Overflow Flag
        const OF = 0b00000000000000000000100000000000;
        /// Ring Privilege 0
        const RP0 =0b00000000000000000000000000000000;
        /// Ring Privilege 1
        const RP1 =0b00000000000000000001000000000000;
        /// Ring Privilege 2
        const RP2 =0b00000000000000000010000000000000;
        /// Ring Privilege 3
        const RP3 =0b00000000000000000011000000000000;
        /// Nested Task Flag
        const NT = 0b00000000000000000100000000000000;
        /// Reserved
        const R4 = 0b00000000000000001000000000000000;

        /// EFLAGS

        /// Resume Flag
        const RF = 0b00000000000000010000000000000000;
        /// Virtual 8086 Mode Flag
        const VM = 0b00000000000000100000000000000000;
        /// Alignment Check
        const AC = 0b00000000000001000000000000000000;
        /// Virtual Interrupt Flag
        const VIF =0b00000000000010000000000000000000;
        /// Virtual Interrupt Pending
        const VIP =0b00000000000100000000000000000000;
        /// CPUID Available
        const ID = 0b00000000001000000000000000000000;

        const _ = !0;
    }
}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct RegistersState {
    // Specific
    pub rsp: VirtAddr,
    // General-purpose registers
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rbp: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    // Interrupt stack frame
    pub rip: VirtAddr,
    pub _reserved: u64, // This is here just so we don't have to re-pack the interrupt frame
    pub rflags: Flags,
}

impl RegistersState {
    pub fn new(rip: VirtAddr, rflags: Flags, rsp: VirtAddr) -> Self {
        RegistersState {
            rflags,
            rip,

            r15: 0,
            r14: 0,
            r13: 0,
            r12: 0,
            r11: 0,
            r10: 0,
            r9: 0,
            r8: 0,
            rdi: 0,
            rsi: 0,
            rbp: 0,
            rdx: 0,
            rcx: 0,
            rbx: 0,
            rax: 0,

            rsp,
            _reserved: 0,
        }
    }
}

#[macro_export]
/// Macro for pushing all general registers onto the stack.
macro_rules! push_registers_state {
    () => {
        concat!(
            "push r15;", //14*8
            "push r14;", //13*8
            "push r13;", //12*8
            "push r12;", //11*8
            "push r11;", //10*8
            "push r10;", // 9*8
            "push r9;",  // 8*8
            "push r8;",  // 7*8
            "push rdi;", // 6*8
            "push rsi;", // 5*8
            "push rbp;", // 4*8
            "push rdx;", // 3*8
            "push rcx;", // 2*8
            "push rbx;", // 1*8
            "push rax;", // 0*8
        )
    };
}

#[macro_export]
/// Macro for mov'ing all registers from a RegistersState struct stored in r9.
macro_rules! unpack_registers_state {
    () => {
        concat!(
            // general-purpose registers start at offset 1*8
            "mov rax, [r9 + 1*8];",
            "mov rbx, [r9 + 2*8];",
            "mov rcx, [r9 + 3*8];",
            "mov rdx, [r9 + 4*8];",
            "mov rbp, [r9 + 5*8];",
            "mov rsi, [r9 + 6*8];",
            "mov rdi, [r9 + 7*8];",
            "mov r8, [r9 + 8*8];",
            "mov r10, [r9 + 10*8];",
            "mov r11, [r9 + 11*8];",
            "mov r12, [r9 + 12*8];",
            "mov r13, [r9 + 13*8];",
            "mov r14, [r9 + 14*8];",
            "mov r15, [r9 + 15*8];",
            "mov r9, [r9 + 9*8];"
        )
    };
}
