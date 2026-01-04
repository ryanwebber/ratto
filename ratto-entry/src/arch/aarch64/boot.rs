use core::cell::SyncUnsafeCell;

use ratto_kernel::{
    arch::aarch64::mem::{MemoryRegion, MemoryRegionType},
    klog,
};

unsafe extern "C" {
    unsafe static __kernel_start: u8;
    unsafe static __kernel_end: u8;
}

// Set by the entry assembly code to point to the DTB physical address
#[unsafe(no_mangle)]
static mut __dtb_ptr: u64 = 0;

static MEMORY_REGIONS: SyncUnsafeCell<[MemoryRegion; 16]> = SyncUnsafeCell::new(
    [MemoryRegion {
        start: 0,
        size: 0,
        kind: MemoryRegionType::Reserved,
    }; 16],
);

#[derive(Debug)]
pub struct Dtb {
    base: *const u8,
}

impl Dtb {
    pub unsafe fn from_phys_addr(addr: u64) -> Option<Self> {
        Some(Dtb {
            base: addr as *const u8,
        })
    }
}

fn kernel_phys_range() -> (u64, u64) {
    unsafe {
        (
            &__kernel_start as *const _ as u64,
            &__kernel_end as *const _ as u64,
        )
    }
}

// TODO: Proper DTB parsing
unsafe fn parse_memory_map(_dtb: &Dtb) -> &'static [MemoryRegion] {
    // TODO: remove this, just to avoid unused variable warning
    _ = _dtb.base;

    let regions = unsafe { &mut *MEMORY_REGIONS.get() };
    let mut count = 0;

    // Example: RAM (from DTB /memory)
    regions[count] = MemoryRegion {
        start: 0x0000_0000,
        size: 0x3B00_0000,
        kind: MemoryRegionType::Usable,
    };
    count += 1;

    // Firmware reserved
    regions[count] = MemoryRegion {
        start: 0x3B00_0000,
        size: 0x0500_0000,
        kind: MemoryRegionType::Firmware,
    };
    count += 1;

    // MMIO
    regions[count] = MemoryRegion {
        start: 0xFE00_0000,
        size: 0x0200_0000,
        kind: MemoryRegionType::Mmio,
    };
    count += 1;

    &regions[..count]
}

pub fn boot_info() -> ratto_kernel::arch::aarch64::boot::BootInfo {
    // Get DTB pointer, should be set by the entry assembly code
    let dtb_phys = unsafe { __dtb_ptr };
    if dtb_phys == 0 {
        panic!("DTB pointer not set");
    }

    klog!("DTB physical address: {:#x}", dtb_phys);

    let dtb = unsafe { Dtb::from_phys_addr(dtb_phys).expect("invalid DTB") };

    let memory_map = unsafe { parse_memory_map(&dtb) };
    let kernel_range = kernel_phys_range();

    klog!(
        "Kernel physical range: {:#x} - {:#x}",
        kernel_range.0,
        kernel_range.1
    );

    ratto_kernel::arch::aarch64::boot::BootInfo {
        memory_map,
        kernel_phys_start: kernel_range.0,
        kernel_phys_end: kernel_range.1,
    }
}
