//! CPU-related operations.

#[percpu::def_percpu]
static CPU_ID: usize = 0;

#[percpu::def_percpu]
static IS_BSP: bool = false;

#[percpu::def_percpu]
static CURRENT_TASK_PTR: usize = 0;

/// Returns the ID of the current CPU.
#[inline]
pub fn this_cpu_id() -> usize {
    CPU_ID.read_current()
}

/// Returns whether the current CPU is the primary CPU (aka the bootstrap
/// processor or BSP)
#[inline]
pub fn this_cpu_is_bsp() -> bool {
    IS_BSP.read_current()
}

#[allow(dead_code)]
/// Initializes the primary CPU for its pointer.
pub fn init_primary(cpu_id: usize) {
    percpu::init(axconfig::SMP);
    percpu::set_local_thread_pointer(cpu_id);
    unsafe {
        CPU_ID.write_current_raw(cpu_id);
        IS_BSP.write_current_raw(true);
    }
}

#[allow(dead_code)]
/// Initializes the secondary CPU for its pointer.
pub fn init_secondary(cpu_id: usize) {
    percpu::set_local_thread_pointer(cpu_id);
    unsafe {
        CPU_ID.write_current_raw(cpu_id);
        IS_BSP.write_current_raw(false);
    }
}
