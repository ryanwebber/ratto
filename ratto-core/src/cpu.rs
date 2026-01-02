pub trait CpuOps {
    type InterruptState: Copy;

    fn disable_interrupts() -> Self::InterruptState;
    fn enable_interrupts(state: Self::InterruptState);

    fn wait_forever() -> ! {
        loop {
            core::hint::spin_loop();
        }
    }
}
