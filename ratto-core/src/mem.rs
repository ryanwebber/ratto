pub trait MemoryMapper {
    type AddressSpace;

    /// Create a new address space (kernel or user).
    fn new_address_space(&mut self) -> Result<Self::AddressSpace, &'static str>;

    /// Map a single page.
    ///
    /// # Safety
    /// Caller must ensure the virtual address is unused and valid.
    unsafe fn map_page(
        &mut self,
        space: &mut Self::AddressSpace,
        virtual_address: VirtualAddress,
        physical_frame: PhysicalFrame,
        flags: MapFlags,
    ) -> Result<(), &'static str>;

    /// Unmap a single page.
    unsafe fn unmap_page(
        &mut self,
        space: &mut Self::AddressSpace,
        virt: VirtualAddress,
    ) -> Result<PhysicalFrame, &'static str>;

    /// Activate an address space (load page table root).
    unsafe fn activate(&self, space: &Self::AddressSpace);
}

pub trait FrameAllocator {
    const FRAME_SIZE: u64;

    /// Allocate a single physical frame.
    fn alloc_frame(&mut self) -> Option<PhysicalFrame>;

    /// Free a previously allocated frame.
    ///
    /// # Safety
    /// Caller must ensure the frame is no longer mapped or in use.
    unsafe fn free_frame(&mut self, frame: PhysicalFrame);
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PhysicalFrame {
    pub addr: PhysicalAddress,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct PhysicalAddress(pub u64);

#[derive(Copy, Clone, Debug)]
pub struct VirtualAddress(pub u64);

bitflags::bitflags! {
    pub struct MapFlags: u32 {
        const READ    = 1 << 0;
        const WRITE   = 1 << 1;
        const EXEC    = 1 << 2;
        const USER    = 1 << 3;
        const GLOBAL  = 1 << 4;
    }
}
