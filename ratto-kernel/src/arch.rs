#[cfg(target_arch = "aarch64")]
pub mod aarch64;

#[cfg(target_arch = "aarch64")]
pub use aarch64::AArch64 as Impl;

pub trait ArchImpl {
    type Cpu: ratto_core::cpu::CpuOps;
    type BootInfo: ratto_core::boot::BootInfo;
    type MemoryMapper: ratto_core::mem::MemoryMapper;
    type FrameAllocator: ratto_core::mem::FrameAllocator;

    fn init_memory(
        boot_info: &Self::BootInfo,
    ) -> Result<(Self::MemoryMapper, Self::FrameAllocator), &'static str>;
}

pub type Cpu = <Impl as ArchImpl>::Cpu;
pub type MemoryMapper = <Impl as ArchImpl>::MemoryMapper;
pub type FrameAllocator = <Impl as ArchImpl>::FrameAllocator;
pub type BootInfo = <Impl as ArchImpl>::BootInfo;

pub mod sync {
    pub type SpinLock<T> = ratto_core::sync::SpinLock<T, super::Cpu>;
    pub type OnceLock<T> = ratto_core::sync::OnceLock<T, super::Cpu>;
}
