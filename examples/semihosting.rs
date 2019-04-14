#![no_std]
#![no_main]

extern crate panic_halt;

use cortex_m_rt::entry;

use cortex_m_semihosting::hprintln;

#[entry]
fn main() -> ! {
    let x = 42;
    let y = x + 1;
    loop {
        hprintln!("Hello world!").unwrap();
    }
}
