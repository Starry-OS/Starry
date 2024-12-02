use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    axtask::dump_curr_backtrace();
    axhal::misc::terminate()
}
