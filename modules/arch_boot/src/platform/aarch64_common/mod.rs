use axruntime::rust_main;

mod boot;

cfg_if::cfg_if! {
    if #[cfg(all(feature = "smp", platform_family = "aarch64-raspi"))] {
        pub mod raspi_mp;
        pub use raspi_mp as mp;
    } else if #[cfg(all(feature = "smp"))]  {
        pub mod mp;
    }
}

/// The earliest entry point for the secondary CPUs.
pub(crate) unsafe extern "C" fn rust_entry(cpu_id: usize, dtb: usize) {
    use axhal::mem::phys_to_virt;
    axhal::mem::clear_bss();

    axhal::cpu::init_primary(cpu_id);

    // init fdt
    axhal::platform::mem::idmap_device(dtb);
    of::init_fdt_ptr(phys_to_virt(dtb.into()).as_usize() as *const u8);

    // HugeMap all device memory for allocator
    of::memory_nodes().map(|nodes| {
        for m in nodes {
            for r in m.regions() {
                axhal::platform::mem::idmap_device(r.starting_address as usize);
            }
        }
    });

    axhal::console::init_early();
    axhal::platform::time::init_early();
    // disable low address access
    axhal::arch::write_page_table_root0(0.into());

    axtrap::init_interrupt();
    axlog::init();
    axlog::set_max_level(option_env!("AX_LOG").unwrap_or("")); // no effect if set `log-level-*` features

    rust_main(cpu_id, dtb);

    #[cfg(feature = "smp")]
    crate::mp::start_secondary_cpus(cpu_id);

    while !axruntime::is_init_ok() {
        core::hint::spin_loop();
    }

    extern "C" {
        fn main();
    }
    unsafe {
        main();
    }

    axruntime::exit_main();
}

#[cfg(feature = "smp")]
pub(crate) unsafe extern "C" fn rust_entry_secondary(cpu_id: usize) {
    use axruntime::rust_main_secondary;

    axtrap::init_interrupt();
    axhal::arch::write_page_table_root0(0.into()); // disable low address access
    axhal::cpu::init_secondary(cpu_id);
    rust_main_secondary(cpu_id);
}
