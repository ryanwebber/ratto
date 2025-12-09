pub mod once_lock;
pub mod spin_lock;

pub use once_lock::OnceLock;
pub use spin_lock::SpinLock;

pub mod arch {
    pub type SpinLock<T> = super::SpinLock<T, crate::arch::Cpu>;
    pub type OnceLock<T> = super::OnceLock<T, crate::arch::Cpu>;
}
