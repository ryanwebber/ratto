#![no_std]

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QemuExitCode {
    Success,
    Failed,
}

#[cfg(target_arch = "x86_64")]
pub fn exit_qemu(exit_code: QemuExitCode) -> ! {
    use x86_64::instructions::{hlt, port::Port};

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(match exit_code {
            QemuExitCode::Success => 0x10u32,
            QemuExitCode::Failed => 0x11u32,
        });
    }

    loop {
        hlt()
    }
}

#[cfg(target_arch = "aarch64")]
pub fn exit_qemu(code: QemuExitCode) -> ! {
    unsafe {
        // For debugging: output a character to the UART0.
        const UART0_DR: *mut u32 = 0x3F201000 as *mut u32;
        core::ptr::write_volatile(UART0_DR, b'W' as u32);

        #[repr(C)]
        struct QEMUParameterBlock {
            arg0: u64,
            arg1: u64,
        }

        let block = QEMUParameterBlock {
            arg0: 0x20026,
            arg1: match code {
                QemuExitCode::Success => 0u64, // QEMU_EXIT_SUCCESS
                QemuExitCode::Failed => 1u64,  // QEMU_EXIT_FAILURE
            },
        };

        core::arch::asm!(
            "hlt #0xF000",
            in("x0") 0x18,
            in("x1") &block as *const _ as u64,
            options(nostack)
        );

        // For the case that the QEMU exit attempt did not work, transition into an infinite loop.
        // Calling `panic!()` here is unfeasible, since there is a good chance this function here is
        // the last expression in the `panic!()` handler itself. This prevents a possible
        // infinite loop.
        loop {
            core::arch::asm!("wfe", options(nomem, nostack));
        }

        core::arch::asm!(
            "mov x0, {addr}",
            "hvc #0",
            addr = in(reg) 0x84000000u64, // PSCI_SYSTEM_OFF
            options(noreturn)
        );
    }
}
