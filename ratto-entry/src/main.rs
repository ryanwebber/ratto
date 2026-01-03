#![no_main]
#![no_std]

mod arch;

use core::panic::PanicInfo;
use ratto_kernel::{Kernel, KernelArgs};

unsafe fn kernel_init(args: KernelArgs) -> ! {
    Kernel::init(args);
    Kernel::run();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    Kernel::panic_dump(info);

    #[cfg(feature = "qemu")]
    ratto_qemu::exit_qemu(ratto_qemu::QemuExitCode::Failed);

    #[cfg(not(feature = "qemu"))]
    {
        use ratto_core::cpu::CpuOps;
        use ratto_kernel::arch::Cpu;
        Cpu::wait_forever();
    }
}
