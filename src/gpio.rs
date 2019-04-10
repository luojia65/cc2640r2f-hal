use core::marker::PhantomData;
use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};
use cc2640r2f::{GPIO, IOC};

pub struct Input<MODE> {
    _mode: PhantomData<MODE>,
}

pub struct Output<MODE> {
    _mode: PhantomData<MODE>,
}

// NORMAL, UP
pub struct PullUp;

// NORMAL, DWN
pub struct PullDown;

// NORMAL, DIS
pub struct Floating;

// NORMAL, DIS
pub struct PushPull;

// OPENDR, DIS
pub struct OpenDrain;

// OPENSRC, DIS
pub struct OpenSource;

pub enum Current {
    _2MA,
    _4MA,
    _4_8MA,
}

use cc2640r2f::ioc::iocfg0::IOCURRW;

pub struct DIO0<MODE> {
    _mode: PhantomData<MODE>,
}

impl<MODE> DIO0<MODE> {
    pub fn into_floating_input(self) -> DIO0<Input<Floating>> {
        unsafe { 
            (*IOC::ptr()).iocfg0.modify(|_r, w| {
                w.iomode().normal();
                w.pull_ctl().dis();
                w
            }) 
        }; 
        DIO0 { _mode: PhantomData }
    }

    pub fn into_pull_up_input(self) -> DIO0<Input<PullUp>> {
        unsafe { 
            (*IOC::ptr()).iocfg0.modify(|_r, w| {
                w.iomode().normal();
                w.pull_ctl().up();
                w
            }) 
        }; 
        DIO0 { _mode: PhantomData }
    }
    
    pub fn into_pull_down_input(self) -> DIO0<Input<PullDown>> {
        unsafe { 
            (*IOC::ptr()).iocfg0.modify(|_r, w| {
                w.iomode().normal();
                w.pull_ctl().dwn();
                w
            }) 
        }; 
        DIO0 { _mode: PhantomData }
    }

    // push pull

    pub fn into_open_drain_output(self) -> DIO0<Output<OpenDrain>> {
        unsafe { 
            (*IOC::ptr()).iocfg0.modify(|_r, w| {
                w.iomode().opendr();
                w.pull_ctl().dis();
                w
            }) 
        }; 
        DIO0 { _mode: PhantomData }
    }

    pub fn into_open_source_output(self) -> DIO0<Output<OpenSource>> {
        unsafe { 
            (*IOC::ptr()).iocfg0.modify(|_r, w| {
                w.iomode().opensrc();
                w.pull_ctl().dis();
                w
            }) 
        }; 
        DIO0 { _mode: PhantomData }
    }
}

impl From<Current> for IOCURRW {
    #[inline]
    fn from(current: Current) -> IOCURRW {
        use Current::*;
        match current {
            _2MA => IOCURRW::_2MA,
            _4MA => IOCURRW::_4MA,
            _4_8MA => IOCURRW::_4_8MA,
        }
    }
}

impl<MODE> DIO0<Output<MODE>> {
    pub fn set_current(self, current: Current) -> Self {
        unsafe { (*IOC::ptr()).iocfg0.modify(|_r, w| w.iocurr().variant(current.into())) };
        self
    }
}

impl<MODE> InputPin for DIO0<Input<MODE>> {
    fn is_high(&self) -> bool {
        unsafe { (*GPIO::ptr()).din31_0.read().dio0().bit_is_set() }
    }

    fn is_low(&self) -> bool {        
        unsafe { (*GPIO::ptr()).din31_0.read().dio0().bit_is_clear() }
    }
}

impl<MODE> OutputPin for DIO0<Output<MODE>> {
    fn set_high(&mut self) {
        unsafe { (*GPIO::ptr()).doutset31_0.modify(|_r, w| w.dio0().set_bit()) } 
    }

    fn set_low(&mut self) {
        unsafe { (*GPIO::ptr()).doutclr31_0.modify(|_r, w| w.dio0().set_bit()) } 
    }
}

impl<MODE> StatefulOutputPin for DIO0<Output<MODE>> {
    fn is_set_high(&self) -> bool {        
        unsafe { (*GPIO::ptr()).dout3_0.read().dio0().bit_is_set() } 
    }

    fn is_set_low(&self) -> bool {
        unsafe { (*GPIO::ptr()).dout3_0.read().dio0().bit_is_clear() } 
    }
}

impl<MODE> ToggleableOutputPin for DIO0<Output<MODE>> {
    fn toggle(&mut self) {
        unsafe { (*GPIO::ptr()).douttgl31_0.write(|w| w.bits(1 << 0)) }
    }
}

/*
// RSM 
gpio! { 
    DIO0, DIO1, DIO2, DIO3, 
    DIO4, DIO5, DIO6, DIO7, 
    DIO8, DIO9, 
}

#[cfg(any(feature = "YFV", feature = "RHB", feature = "RGZ"))]
gpio! { 
    DIO10, DIO11, DIO12, DIO13, 
}

#[cfg(any(feature = "RHB", feature = "RGZ"))]
gpio! { 
    DIO14,
}

#[cfg(any(feature = "RGZ"))]
gpio! { 
    DIO15, DIO16, DIO17, DIO18, 
    DIO19, DIO20, DIO21, DIO22, 
    DIO23, DIO24, DIO25, DIO26,
    DIO27, DIO28, DIO29, DIO30,
}
*/