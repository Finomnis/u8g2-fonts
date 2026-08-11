#![no_std]
#![no_main]

use core::hint::black_box;
use cortex_m_rt::entry;
use cortex_m_semihosting::{
    debug::{self, EXIT_SUCCESS},
    hprintln,
};
use panic_semihosting as _;

#[entry]
fn main() -> ! {
    hprintln!("Dummy test");

    let result = run_benchmark();

    black_box(result);
    assert!(result == 1234);

    debug::exit(EXIT_SUCCESS);
    panic!();
}

#[inline(never)]
fn run_benchmark() -> u32 {
    1234
}
