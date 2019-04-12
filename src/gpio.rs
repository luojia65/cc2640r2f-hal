use core::marker::PhantomData;
use embedded_hal::digital::{
    InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin
};
use crate::{
    pac,
};

pub trait GpioExt {
    type Parts;

    fn split(self) -> Self::Parts;
}

impl GpioExt for (pac::GPIO, pac::IOC) {
    type Parts = Parts;

    fn split(self) -> Self::Parts {
        unsafe { (*pac::GPIO::ptr()).doe31_0.modify(|_r, w| w.bits(0xffff_ffff)) };
        Parts {
            dio0: DIO0 { _mode: PhantomData },
            iocfg0: IOCFG0 { _mark_owned: () }, 
        }
    }
}

pub struct Parts {
    pub dio0: DIO0<Input<Floating>>,

    pub iocfg0: IOCFG0,
}

pub enum Current {
    LC_2MA,
    HC_4MA,
    EC_4_8MA,
}

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

pub struct DIO0<MODE> {
    pub(crate) _mode: PhantomData<MODE>,
}

impl<MODE> InputPin for DIO0<Input<MODE>> {
    fn is_high(&self) -> bool {
        unsafe { (*pac::GPIO::ptr()).din31_0.read().dio0().bit_is_set() }
    }

    fn is_low(&self) -> bool {        
        unsafe { (*pac::GPIO::ptr()).din31_0.read().dio0().bit_is_clear() }
    }
}

impl<MODE> OutputPin for DIO0<Output<MODE>> {
    fn set_high(&mut self) {
        unsafe { (*pac::GPIO::ptr()).doutset31_0.modify(|_r, w| w.dio0().set_bit()) } 
    }

    fn set_low(&mut self) {
        unsafe { (*pac::GPIO::ptr()).doutclr31_0.modify(|_r, w| w.dio0().set_bit()) } 
    }
}

impl<MODE> StatefulOutputPin for DIO0<Output<MODE>> {
    fn is_set_high(&self) -> bool {        
        unsafe { (*pac::GPIO::ptr()).dout31_0.read().dio0().bit_is_set() } 
    }

    fn is_set_low(&self) -> bool {
        unsafe { (*pac::GPIO::ptr()).dout31_0.read().dio0().bit_is_clear() } 
    }
}

impl<MODE> ToggleableOutputPin for DIO0<Output<MODE>> {
    fn toggle(&mut self) {
        unsafe { (*pac::GPIO::ptr()).douttgl31_0.write(|w| w.bits(1 << 0)) }
    }
}

pub struct IOCFG0 {
    _mark_owned: ()
}

impl IOCFG0 {
    pub(crate) fn iocfg0(&mut self) -> &pac::ioc::IOCFG0 {
        unsafe { &(*pac::IOC::ptr()).iocfg0 }
    }
}

impl From<Current> for pac::ioc::iocfg0::IOCURRW {
    #[inline]
    fn from(current: Current) -> Self {
        use pac::ioc::iocfg0::IOCURRW;
        match current {
            Current::LC_2MA => IOCURRW::_2MA,
            Current::HC_4MA => IOCURRW::_4MA,
            Current::EC_4_8MA => IOCURRW::_4_8MA,
        }
    }
}

impl<MODE> DIO0<Output<MODE>> {
    pub fn set_current(self, current: Current, iocfg0: &mut IOCFG0) -> Self {
        iocfg0.iocfg0().modify(|_r, w| w.iocurr().variant(current.into()));
        self
    }
}
impl<MODE> DIO0<MODE> {
    pub fn into_floating_input(self, iocfg0: &mut IOCFG0) -> DIO0<Input<Floating>> {
        iocfg0.iocfg0().modify(|_r, w| {
            w.iomode().normal();
            w.pull_ctl().dis();
            w
        });
        DIO0 { _mode: PhantomData }
    }

    pub fn into_pull_up_input(self, iocfg0: &mut IOCFG0) -> DIO0<Input<PullUp>> {
        iocfg0.iocfg0().modify(|_r, w| {
            w.iomode().normal();
            w.pull_ctl().up();
            w
        });
        DIO0 { _mode: PhantomData }
    }
    
    pub fn into_pull_down_input(self, iocfg0: &mut IOCFG0) -> DIO0<Input<PullDown>> {
        iocfg0.iocfg0().modify(|_r, w| {
            w.iomode().normal();
            w.pull_ctl().dwn();
            w
        });
        DIO0 { _mode: PhantomData }
    }

    pub fn into_push_pull_output(self, iocfg0: &mut IOCFG0) -> DIO0<Output<PushPull>> {
        iocfg0.iocfg0().modify(|_r, w| {
            w.iomode().normal();
            w.pull_ctl().dis();
            w
        });
        DIO0 { _mode: PhantomData }
    }

    pub fn into_open_drain_output(self, iocfg0: &mut IOCFG0) -> DIO0<Output<OpenDrain>> {
        iocfg0.iocfg0().modify(|_r, w| {
            w.iomode().opendr();
            w.pull_ctl().dis();
            w
        });
        DIO0 { _mode: PhantomData }
    }

    pub fn into_open_source_output(self, iocfg0: &mut IOCFG0) -> DIO0<Output<OpenSource>> {
        iocfg0.iocfg0().modify(|_r, w| {
            w.iomode().opensrc();
            w.pull_ctl().dis();
            w
        });
        DIO0 { _mode: PhantomData }
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