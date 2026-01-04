use crate::arch::aarch64::mem::MemoryRegion;

#[derive(Debug, Clone)]
pub struct BootInfo {
    /// Physical memory regions
    pub memory_map: &'static [MemoryRegion],

    /// Kernel physical base
    pub kernel_phys_start: u64,
    pub kernel_phys_end: u64,
}

impl ratto_core::boot::BootInfo for BootInfo {}
