#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::Impl;
use ratto_core::cpu::CpuOps;

pub trait ArchImpl {
    type Cpu: CpuOps;
}

pub type Cpu = <Impl as ArchImpl>::Cpu;

pub fn current<'a>() -> &'a Impl {
    todo!()
}

pub mod sync {
    pub type SpinLock<T> = ratto_core::sync::SpinLock<T, super::Cpu>;
    pub type OnceLock<T> = ratto_core::sync::OnceLock<T, super::Cpu>;
}
