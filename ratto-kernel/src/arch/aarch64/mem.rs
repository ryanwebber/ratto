use crate::arch::aarch64::boot::BootInfo;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MemoryRegionType {
    Usable,
    Reserved,
    Mmio,
    Firmware,
}

#[derive(Debug, Copy, Clone)]
pub struct MemoryRegion {
    pub start: u64,
    pub size: u64,
    pub kind: MemoryRegionType,
}

pub struct MemoryMapper;

impl MemoryMapper {
    pub fn new(_frame_allocator: &FrameAllocator) -> Result<Self, &'static str> {
        Ok(MemoryMapper)
    }
}

impl ratto_core::mem::MemoryMapper for MemoryMapper {
    type AddressSpace = ();

    fn new_address_space(&mut self) -> Result<Self::AddressSpace, &'static str> {
        todo!()
    }

    unsafe fn map_page(
        &mut self,
        _space: &mut Self::AddressSpace,
        _virtual_address: ratto_core::mem::VirtualAddress,
        _physical_frame: ratto_core::mem::PhysicalFrame,
        _flags: ratto_core::mem::MapFlags,
    ) -> Result<(), &'static str> {
        todo!()
    }

    unsafe fn unmap_page(
        &mut self,
        _space: &mut Self::AddressSpace,
        _virt: ratto_core::mem::VirtualAddress,
    ) -> Result<ratto_core::mem::PhysicalFrame, &'static str> {
        todo!()
    }

    unsafe fn activate(&self, _space: &Self::AddressSpace) {
        todo!()
    }
}

pub struct FrameAllocator;

impl FrameAllocator {
    pub fn new(_boot_info: &BootInfo) -> Result<Self, &'static str> {
        Ok(FrameAllocator)
    }
}

impl ratto_core::mem::FrameAllocator for FrameAllocator {
    const FRAME_SIZE: u64 = 0x1000;

    fn alloc_frame(&mut self) -> Option<ratto_core::mem::PhysicalFrame> {
        todo!()
    }

    unsafe fn free_frame(&mut self, _frame: ratto_core::mem::PhysicalFrame) {
        todo!()
    }
}
