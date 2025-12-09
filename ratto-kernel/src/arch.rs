pub trait ArchImpl {
    type Cpu: CpuOps;
}

#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::Impl;

pub type Cpu = <Impl as ArchImpl>::Cpu;

pub fn current<'a>() -> &'a Impl {
    todo!()
}

pub trait CpuOps {
    type InterruptState: Copy;

    fn disable_interrupts() -> Self::InterruptState;
    fn enable_interrupts(state: Self::InterruptState);

    fn wait_forever() -> ! {
        loop {
            core::hint::spin_loop();
        }
    }
}
