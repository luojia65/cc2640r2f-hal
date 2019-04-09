use core::marker::PhantomData;
use embedded_hal::digital::{InputPin, OutputPin, StatefulOutputPin, ToggleableOutputPin};

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
    _mode: PhantomData<MODE>,
}

impl<MODE> DIO0<MODE> {

}

impl<MODE> OutputPin for DIO0<Output<MODE>> {
    fn set_high(&mut self) {

    }

    fn set_low(&mut self) {

    }
}

impl<MODE> InputPin for DIO0<Input<MODE>> {
    fn is_high(&self) -> bool {
        false
    }

    fn is_low(&self) -> bool {
        true
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