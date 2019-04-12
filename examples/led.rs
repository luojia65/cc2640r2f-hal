#![no_std]
#![no_main]

extern crate panic_halt;

use embedded_hal::{
    digital::OutputPin,
};

use cortex_m_rt::entry;

use cc2640r2f_hal::{
    prelude::*,
    pac
};

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let gpio = (dp.GPIO, dp.IOC).split();
    let mut dio0 = gpio.dio0.into_push_pull_output(&mut gpio.iocfg0);
    loop {
        dio0.set_high();
        dio0.set_low();
    }
}
