mod boot;

#[cfg(feature = "smp")]
pub mod mp;

extern "C" {
    fn main();
}
unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    axhal::mem::clear_bss();
    axhal::cpu::init_primary(cpu_id);
    axhal::platform::time::init_board_info(dtb);
    axtrap::init_interrupt();
    axlog::init();
    axlog::set_max_level(option_env!("AX_LOG").unwrap_or("")); // no effect if set `log-level-*` features

    axruntime::rust_main(cpu_id, dtb);

    #[cfg(feature = "smp")]
    crate::mp::start_secondary_cpus(cpu_id);

    while !axruntime::is_init_ok() {
        core::hint::spin_loop();
    }

    unsafe {
        main();
    }

    axruntime::exit_main();
}

#[cfg(feature = "smp")]
unsafe extern "C" fn rust_entry_secondary(cpu_id: usize) {
    axtrap::init_interrupt();
    axhal::cpu::init_secondary(cpu_id);
    axruntime::rust_main_secondary(cpu_id);
}
