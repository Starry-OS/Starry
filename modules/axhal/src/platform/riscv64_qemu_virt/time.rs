use lazy_init::LazyInit;
use riscv::register::time;

static CPU_FREQ: LazyInit<u64> = LazyInit::new();
static LOOPS_PRE_TICK: LazyInit<u64> = LazyInit::new();
const NANOS_PER_TICK: u64 = crate::time::NANOS_PER_SEC / axconfig::TIMER_FREQUENCY as u64;

// Initializes the frequency of cpu.
#[inline]
fn init_cpu_freq(freq: u64) {
    if !CPU_FREQ.is_init() {
        CPU_FREQ.init_by(freq);
    }

    if !LOOPS_PRE_TICK.is_init() {
        LOOPS_PRE_TICK.init_by(freq / axconfig::TIMER_FREQUENCY as u64);
    }
}

/// Returns the frequency of cpu.
#[inline]
#[allow(unused)]
pub fn cpu_freq() -> u64 {
    *CPU_FREQ
}

/// Returns loops per tick.
#[inline]
pub fn loops_pre_tick() -> u64 {
    *LOOPS_PRE_TICK
}

/// Returns the current clock time in hardware ticks.
#[inline]
pub fn current_ticks() -> u64 {
    time::read() as u64 / loops_pre_tick()
}

/// Converts hardware ticks to nanoseconds.
#[inline]
pub const fn ticks_to_nanos(ticks: u64) -> u64 {
    ticks * NANOS_PER_TICK
}

/// Converts nanoseconds to hardware ticks.
#[inline]
pub const fn nanos_to_ticks(nanos: u64) -> u64 {
    nanos / NANOS_PER_TICK
}

/// Set a one-shot timer.
///
/// A timer interrupt will be triggered at the given deadline (in nanoseconds).
#[cfg(feature = "irq")]
pub fn set_oneshot_timer(deadline_ns: u64) {
    sbi_rt::set_timer(nanos_to_ticks(deadline_ns) * loops_pre_tick());
}

pub(super) fn init_percpu() {
    #[cfg(feature = "irq")]
    sbi_rt::set_timer(0);
}

pub fn init_board_info(dtb: usize) {
    unsafe {
        of::init_fdt_ptr(dtb as *const u8);
    }
    let of_cpus = of::cpus();
    let freq = {
        if let Some(cpu) = of_cpus.expect("Failed to read cpu info").nth(0) {
            cpu.timebase_frequency()
        } else {
            axconfig::TIMER_FREQUENCY
        }
    };
    init_cpu_freq(freq as u64);
}
