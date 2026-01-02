use ratto_core::sync::SpinLock;
use ratto_kernel::{arch::aarch64, console};

pub static SERIAL_CONSOLE: SerialConsole = SerialConsole {
    lock: SpinLock::new(()),
};

pub struct SerialConsole {
    lock: SpinLock<(), aarch64::Cpu>,
}

impl console::Console for SerialConsole {
    fn write_str(&self, s: &str) -> core::fmt::Result {
        self.lock.lock_with(|_| {
            for c in s.chars() {
                unsafe {
                    core::ptr::write_volatile(0x3F20_1000 as *mut u8, c as u8);
                }
            }
        });

        Ok(())
    }
}

impl core::fmt::Debug for SerialConsole {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("QUMUSerialConsole").finish()
    }
}
