use core::arch::asm;

use crate::arch::ArchImpl;

pub struct Impl;

impl ArchImpl for Impl {
    type Cpu = Cpu;
}

pub struct Cpu;

impl crate::arch::CpuOps for Cpu {
    type InterruptState = u64;

    fn disable_interrupts() -> Self::InterruptState {
        let flags: u64;
        unsafe {
            asm!(
                "mrs {0}, daif",
                "msr daifset, #2",
                out(reg) flags,
                options(nomem, nostack)
            );
        }

        flags
    }

    fn enable_interrupts(flags: Self::InterruptState) {
        unsafe {
            asm!(
                "msr daif, {0}",
                in(reg) flags as u64,
                options(nomem, nostack)
            );
        }
    }

    fn wait_forever() -> ! {
        use core::arch::asm;

        unsafe {
            loop {
                asm!("wfe", options(nomem, nostack));
            }
        }
    }
}
