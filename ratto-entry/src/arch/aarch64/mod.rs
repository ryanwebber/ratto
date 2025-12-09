// SPDX-License-Identifier: MIT OR Apache-2.0
//
// Copyright (c) 2021-2023 Andre Richter <andre.o.richter@gmail.com>

//! Architectural boot code.
//!
//! # Orientation
//!
//! Since arch modules are imported into generic modules using the path attribute, the path of this
//! file is:
//!
//! crate::cpu::boot::arch_boot

use core::arch::global_asm;

use ratto_kernel::KernelArgs;

use crate::kernel_init;

pub mod cpu;
pub mod serial;

global_asm!(
    include_str!("entry.s"),
    CONST_CORE_ID_MASK = const 0b11
);

#[unsafe(no_mangle)]
pub unsafe fn _start_rust() -> ! {
    let args = KernelArgs {
        console: Some(&serial::SERIAL_CONSOLE),
    };

    unsafe { kernel_init(args) }
}
