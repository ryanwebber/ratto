#![no_main]
#![no_std]

mod arch;

use core::panic::PanicInfo;

unsafe fn kernel_init() -> ! {
    panic!()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    #[cfg(feature = "qemu")]
    ratto_qemu::exit_qemu(ratto_qemu::QemuExitCode::Failed);

    #[cfg(not(feature = "qemu"))]
    arch::cpu::wait_forever();
}
