#[cfg(all(any(target_arch = "x86", target_arch = "x86_64")))]
#[inline(always)]
fn rdtsc() -> u64 {
    #[cfg(target_arch = "x86")]
    use core::arch::x86;
    #[cfg(target_arch = "x86_64")]
    use core::arch::x86_64 as x86;

    unsafe {
        x86::_mm_lfence();
        let rdtsc = x86::_rdtsc();
        x86::_mm_lfence();

        rdtsc
    }
}
