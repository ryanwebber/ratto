use crate::arch::ArchImpl;

pub mod boot;
pub mod cpu;
pub mod mem;

pub struct AArch64;

impl ArchImpl for AArch64 {
    type Cpu = cpu::Cpu;
    type MemoryMapper = mem::MemoryMapper;
    type FrameAllocator = mem::FrameAllocator;
    type BootInfo = boot::BootInfo;

    fn init_memory(
        boot_info: &Self::BootInfo,
    ) -> Result<(Self::MemoryMapper, Self::FrameAllocator), &'static str> {
        let alloc = Self::FrameAllocator::new(boot_info)?;
        let mapper = Self::MemoryMapper::new(&alloc)?;
        Ok((mapper, alloc))
    }
}
