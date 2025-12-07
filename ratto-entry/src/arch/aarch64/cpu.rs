/// Used by `arch` code to find the early boot core.
#[unsafe(no_mangle)]
#[unsafe(link_section = ".text._start_arguments")]
pub static BOOT_CORE_ID: u64 = 0;

#[allow(dead_code)]
pub fn wait_forever() -> ! {
    use core::arch::asm;

    unsafe {
        loop {
            asm!("wfe", options(nomem, nostack));
        }
    }
}
