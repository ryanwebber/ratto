use core::arch::global_asm;

use ratto_kernel::{Kernel, KernelArgs, console::Console};

mod boot;
mod cpu;

global_asm!(
    include_str!("entry.s"),
    CONST_CORE_ID_MASK = const 0b11
);

#[unsafe(no_mangle)]
pub unsafe fn _start_rust() -> ! {
    let console: Option<&'static dyn Console> = {
        #[cfg(feature = "qemu")]
        {
            static SERIAL_CONSOLE: ratto_qemu::SerialConsole = ratto_qemu::SerialConsole::new();
            Some(&SERIAL_CONSOLE)
        }

        #[cfg(not(feature = "qemu"))]
        {
            None
        }
    };

    Kernel::init1(console);

    let args = KernelArgs {
        boot_info: boot::boot_info(),
        console,
    };

    Kernel::init2(args);
    Kernel::run();
}
