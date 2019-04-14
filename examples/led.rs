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
    let gpio = dp.GPIO.split();
    let mut ioc = dp.IOC.split();
    let mut led = gpio.dio6.into_push_pull_output(&mut ioc.iocfg6);
    loop {
        led.set_high();
        led.set_low();
    }
}
