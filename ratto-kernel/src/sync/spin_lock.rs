use core::hint::spin_loop;
use core::sync::atomic::Ordering;

use crate::arch::CpuOps;

pub struct SpinLock<T: Sized, Cpu: CpuOps> {
    data: core::cell::UnsafeCell<T>,
    lock: core::sync::atomic::AtomicBool,
    _phantom: core::marker::PhantomData<Cpu>,
}
unsafe impl<T: Sized + Send, Cpu: CpuOps> Sync for SpinLock<T, Cpu> {}
unsafe impl<T: Sized + Send, Cpu: CpuOps> Send for SpinLock<T, Cpu> {}

impl<T, Cpu: CpuOps> SpinLock<T, Cpu> {
    pub const fn new(data: T) -> Self {
        SpinLock {
            data: core::cell::UnsafeCell::new(data),
            lock: core::sync::atomic::AtomicBool::new(false),
            _phantom: core::marker::PhantomData,
        }
    }
}

impl<T: Sized, Cpu: CpuOps> SpinLock<T, Cpu> {
    pub fn lock(&self) -> SpinLockGuard<'_, T, Cpu> {
        let interrupt_state = Cpu::disable_interrupts();
        while self
            .lock
            .compare_exchange_weak(false, true, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            while self.lock.load(Ordering::Relaxed) {
                spin_loop();
            }
        }

        SpinLockGuard {
            lock: self,
            interrupt_state,
        }
    }

    pub fn lock_with<R, F>(&self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        let mut guard = self.lock();
        f(&mut *guard)
    }
}

#[must_use]
pub struct SpinLockGuard<'a, T: Sized, Cpu: CpuOps> {
    lock: &'a SpinLock<T, Cpu>,
    interrupt_state: Cpu::InterruptState,
}

impl<'a, T: Sized, Cpu: CpuOps> core::ops::Deref for SpinLockGuard<'a, T, Cpu> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*self.lock.data.get() }
    }
}

impl<'a, T: Sized, Cpu: CpuOps> core::ops::DerefMut for SpinLockGuard<'a, T, Cpu> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { &mut *self.lock.data.get() }
    }
}

impl<'a, T: Sized, Cpu: CpuOps> Drop for SpinLockGuard<'a, T, Cpu> {
    fn drop(&mut self) {
        self.lock.lock.store(false, Ordering::Release);
        Cpu::enable_interrupts(self.interrupt_state);
    }
}
