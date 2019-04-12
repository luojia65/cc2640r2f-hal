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
    LC2MA,
    HC4MA,
    EC4_8MA,
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
        unsafe { (*pac::GPIO::ptr()).dout3_0.read().dio0().bit_is_set() } 
    }

    fn is_set_low(&self) -> bool {
        unsafe { (*pac::GPIO::ptr()).dout3_0.read().dio0().bit_is_clear() } 
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
            Current::LC2MA => IOCURRW::_2MA,
            Current::HC4MA => IOCURRW::_4MA,
            Current::EC4_8MA => IOCURRW::_4_8MA,
        }
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

impl<MODE> DIO0<Output<MODE>> {
    pub fn set_current(self, current: Current, iocfg0: &mut IOCFG0) -> Self {
        iocfg0.iocfg0().modify(|_r, w| w.iocurr().variant(current.into()));
        self
    }
}

/*
#[cfg(any(
    feature = "RSM", 
    feature = "YFV", 
    feature = "RHB", 
    feature = "RGZ"
))]
dio! { 
    DIO0: (dio0, IOCFG0, iocfg0, dout3_0, 0), 
    DIO1: (dio1, IOCFG1, iocfg1, dout3_0, 1), 
    DIO2: (dio2, IOCFG2, iocfg2, dout3_0, 2), 
    DIO3: (dio3, IOCFG3, iocfg3, dout3_0, 3), 
    DIO4: (dio4, IOCFG4, iocfg4, dout7_4, 0), 
    DIO5: (dio5, IOCFG5, iocfg5, dout7_4, 1), 
    DIO6: (dio6, IOCFG6, iocfg6, dout7_4, 2),
    DIO7: (dio7, IOCFG7, iocfg7, dout7_4, 3), 
    DIO8: (dio8, IOCFG8, iocfg8, dout11_8, 0), 
    DIO9: (dio9, IOCFG9, iocfg9, dout11_8, 1), 
}

#[cfg(any(
    feature = "YFV", 
    feature = "RHB", 
    feature = "RGZ"
))]
dio! { 
    DIO10: (dio10, IOCFG10, iocfg10, dout11_8, 2),
    DIO11: (dio11, IOCFG11, iocfg11, dout11_8, 3), 
    DIO12: (dio12, IOCFG12, iocfg12, dout15_12, 0), 
    DIO13: (dio13, IOCFG13, iocfg13, dout15_12, 1), 
}

#[cfg(any(
    feature = "RHB", 
    feature = "RGZ"
))]
dio! { 
    DIO14: (dio14, IOCFG14, iocfg14, dout15_12, 2), 
}

#[cfg(any(feature = "RGZ"))]
dio! { 
    DIO15: (dio15, IOCFG15, iocfg15, dout15_12, 3), 
    DIO16: (dio16, IOCFG16, iocfg16, dout19_16, 0), 
    DIO17: (dio17, IOCFG17, iocfg17, dout19_16, 1), 
    DIO18: (dio18, IOCFG18, iocfg18, dout19_16, 2),
    DIO19: (dio19, IOCFG19, iocfg19, dout19_16, 3), 
    DIO20: (dio20, IOCFG20, iocfg20, dout23_20, 0), 
    DIO21: (dio21, IOCFG21, iocfg21, dout23_20, 1), 
    DIO22: (dio22, IOCFG22, iocfg22, dout23_20, 2),
    DIO23: (dio23, IOCFG23, iocfg23, dout23_20, 3), 
    DIO24: (dio24, IOCFG24, iocfg24, dout27_24, 0), 
    DIO25: (dio25, IOCFG25, iocfg25, dout27_24, 1), 
    DIO26: (dio26, IOCFG26, iocfg26, dout27_24, 2),
    DIO27: (dio27, IOCFG27, iocfg27, dout27_24, 3), 
    DIO28: (dio28, IOCFG28, iocfg28, dout31_28, 0), 
    DIO29: (dio29, IOCFG29, iocfg29, dout31_28, 1), 
    DIO30: (dio30, IOCFG30, iocfg30, dout31_28, 2),
}
*/

/*
    DIO31 is only a logical identifier, none of existing packages
    have physically ported this name to an actual pin.
    Ref: P984, TI Literature swcu117h
*/
