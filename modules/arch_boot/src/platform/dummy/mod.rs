#[cfg(feature = "smp")]
pub mod mp {
    use axhal::mem::PhysAddr;

    pub fn start_given_secondary_cpu(_cpu_id: usize, _stack_top: PhysAddr) {}
}
