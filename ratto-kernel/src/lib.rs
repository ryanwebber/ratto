#![cfg_attr(not(test), no_std)]
#![feature(sync_unsafe_cell)]
#![feature(format_args_nl)]

use core::cell::SyncUnsafeCell;
use core::fmt::Debug;
use core::panic::PanicInfo;

use crate::arch::ArchImpl;
use crate::console::Console;

pub mod arch;
pub mod console;
pub mod print;

static KERNEL_INSTANCE: KernelCell = KernelCell::new();

pub struct Kernel {
    console: Option<&'static dyn Console>,
}

impl Debug for Kernel {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("Kernel")
            .field("console", &self.console)
            .finish()
    }
}

impl Kernel {
    pub fn init(args: KernelArgs) {
        assert!(
            matches!(kernel(), KernelState::Uninit),
            "Kernel::init() called more than once"
        );

        KERNEL_INSTANCE.promote(KernelState::Initializing(args.console));

        klog!("Kernel initialization started...");

        arch::Current::try_init().expect("Failed architecture initialization");

        let kernel = Kernel {
            console: args.console,
        };

        KERNEL_INSTANCE.promote(KernelState::Ready(kernel));
        klog!("Kernel initialization completed.");
    }

    pub fn run() -> ! {
        assert!(
            kernel().is_ready(),
            "Kernel::run() called before Kernel::init()"
        );

        klog!("Kernel main loop starting...");
        panic!("Unimplemented: Kernel main loop");
    }

    pub fn panic_dump(info: &PanicInfo) {
        kerr!("!!! Kernel panic !!!");

        if let Some(location) = info.location() {
            kraw!("At: {}", location);
        }

        kraw!("Reason: {}", info.message());
        kraw!("Kernel State: {:#?}", kernel());
    }
}

pub struct KernelArgs {
    pub console: Option<&'static dyn Console>,
}

#[derive(Debug)]
pub enum KernelState {
    Uninit,
    Initializing(Option<&'static dyn Console>),
    Ready(Kernel),
}

impl KernelState {
    pub fn console(&self) -> Option<&dyn Console> {
        match self {
            KernelState::Uninit => None,
            KernelState::Initializing(console) => *console,
            KernelState::Ready(kernel) => kernel.console,
        }
    }

    pub fn is_ready(&self) -> bool {
        matches!(self, KernelState::Ready(..))
    }

    pub fn as_ready(&self) -> &Kernel {
        match self {
            KernelState::Ready(kernel) => kernel,
            _ => panic!("Kernel is not ready"),
        }
    }

    pub fn try_as_ready(&self) -> Option<&Kernel> {
        match self {
            KernelState::Ready(kernel) => Some(kernel),
            _ => None,
        }
    }
}

pub struct KernelCell {
    inner: SyncUnsafeCell<KernelState>,
}

impl KernelCell {
    pub const fn new() -> Self {
        KernelCell {
            inner: SyncUnsafeCell::new(KernelState::Uninit),
        }
    }

    pub fn get(&self) -> &KernelState {
        unsafe { &*self.inner.get() }
    }

    pub fn promote(&self, state: KernelState) {
        unsafe {
            let ptr = self.inner.get();
            core::ptr::write(ptr, state);
        }
    }
}

pub fn kernel() -> &'static KernelState {
    let kernel_ptr = KERNEL_INSTANCE.get();
    &*kernel_ptr
}
