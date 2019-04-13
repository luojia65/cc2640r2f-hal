use crate::pac;

pub trait IocExt {
    type Parts;

    fn split(self) -> Self::Parts;
}

impl IocExt for pac::IOC {
    type Parts = Parts;

    fn split(self) -> Self::Parts {
        Parts {
            iocfg0: IOCFG0 { _mark_owned: () }
        }
    }
}

pub struct Parts {

    pub iocfg0: IOCFG0,
}

pub struct IOCFG0 {
    _mark_owned: ()
}

impl IOCFG0 {
    pub(crate) fn iocfg0(&mut self) -> &pac::ioc::IOCFG0 {
        unsafe { &(*pac::IOC::ptr()).iocfg0 }
    }
}
