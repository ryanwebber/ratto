use crate::{cpu::CpuOps, sync::SpinLock};

pub struct OnceLock<T, Cpu: CpuOps> {
    inner: SpinLock<Option<T>, Cpu>,
}

impl<T, Cpu: CpuOps> OnceLock<T, Cpu> {
    pub const fn new() -> Self {
        OnceLock {
            inner: SpinLock::new(None),
        }
    }

    pub fn get(&self) -> Option<&T> {
        let guard = self.inner.lock();
        if let Some(ref value) = *guard {
            // Safe because the lock is held, so the value cannot be modified or removed.
            let ptr = value as *const T;
            unsafe { Some(&*ptr) }
        } else {
            None
        }
    }

    pub fn get_mut(&self) -> Option<&mut T> {
        let mut guard = self.inner.lock();
        if let Some(ref mut value) = *guard {
            // Safe because the lock is held, so the value cannot be modified or removed.
            let ptr = value as *mut T;
            unsafe { Some(&mut *ptr) }
        } else {
            None
        }
    }

    pub fn get_or_init<F>(&self, init: F) -> &T
    where
        F: FnOnce() -> T,
    {
        let mut guard = self.inner.lock();
        if guard.is_none() {
            *guard = Some(init());
        }
        // Safe because the lock is held, so the value cannot be modified or removed.
        let ptr = guard.as_ref().unwrap() as *const T;
        unsafe { &*ptr }
    }
}

unsafe impl<T: Send, Cpu: CpuOps> Sync for OnceLock<T, Cpu> {}
unsafe impl<T: Send, Cpu: CpuOps> Send for OnceLock<T, Cpu> {}
