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

use crate::kernel_init;

pub mod cpu;

// Assembly counterpart to this file.
global_asm!(
    include_str!("entry.s"),
    CONST_CORE_ID_MASK = const 0b11
);

//--------------------------------------------------------------------------------------------------
// Public Code
//--------------------------------------------------------------------------------------------------

/// The Rust entry of the `kernel` binary.
///
/// The function is called from the assembly `_start` function.
#[unsafe(no_mangle)]
pub unsafe fn _start_rust() -> ! {
    // For debugging: output a character to the UART0.
    const UART0_DR: *mut u32 = 0x3F201000 as *mut u32;
    unsafe {
        core::ptr::write_volatile(UART0_DR, b'X' as u32);
    }

    unsafe { kernel_init() }
}
