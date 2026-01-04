#![no_main]
#![no_std]
#![feature(sync_unsafe_cell)]
#![feature(format_args_nl)]

mod arch;

use core::panic::PanicInfo;
use ratto_kernel::Kernel;

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
