use self::trap::set_exception_vector_base;

mod trap;

#[cfg(feature = "monolithic")]
mod mem_fault;

extern "C" {
    fn exception_vector_base();

}
/// To initialize the exception vector base address.
#[inline]
pub fn init_interrupt() {
    set_exception_vector_base(exception_vector_base as usize);
}
