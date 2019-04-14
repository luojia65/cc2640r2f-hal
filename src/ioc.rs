use crate::pac;

pub trait IocExt {
    type Parts;

    fn split(self) -> Self::Parts;
}

impl IocExt for pac::IOC {
    type Parts = Parts;

    fn split(self) -> Self::Parts {
        Parts {
            iocfg6: IOCFG6 { _mark_owned: () }
        }
    }
}

pub struct Parts {

    pub iocfg6: IOCFG6,
}

pub struct IOCFG6 {
    _mark_owned: ()
}

impl IOCFG6 {
    pub(crate) fn iocfg6(&mut self) -> &pac::ioc::IOCFG6 {
        unsafe { &(*pac::IOC::ptr()).iocfg6 }
    }
}
