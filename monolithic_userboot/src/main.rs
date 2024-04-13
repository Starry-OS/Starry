//! The userboot of the operating system, which will start the first user process and go into the user mode
#![cfg_attr(not(test), no_std)]
#![no_main]

mod batch;
extern crate alloc;
use alloc::format;
#[no_mangle]
fn main() {
    axstarry::fs_init();
    #[cfg(feature = "batch")]
    {
        batch::run_batch_testcases();
        axstarry::println(format!("System halted with exit code {}", 0).as_str());
    }
    #[cfg(not(feature = "batch"))]
    {
        let testcase = "busybox sh";
        axstarry::run_testcase(testcase);
        axstarry::println(format!("System halted with exit code {}", 0).as_str());
    }
}
