#![no_std]

pub use cc2640r2f as pac;

pub mod ioc;
pub mod gpio;

pub mod prelude {
    pub use crate::gpio::GpioExt as _cc2640r2f_hal_gpio_GpioExt;
    pub use crate::ioc::IocExt as _cc2640r2f_hal_gpio_IocExt;
}
