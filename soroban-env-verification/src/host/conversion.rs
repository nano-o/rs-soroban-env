use crate::{Host, RawVal};
use core::convert::Infallible;

impl Host {
    // Notes on metering: free
    pub(crate) fn usize_from_rawval_u32_input(
        &self,
        r: RawVal,
    ) -> Result<usize, Infallible> {
        self.u32_from_rawval_input(r).map(|u| u as usize)
    }

    // Notes on metering: free
    pub(crate) fn u32_from_rawval_input(
        &self,
        r: RawVal,
    ) -> Result<u32, Infallible> {
        match u32::try_from(r) {
            Ok(v) => Ok(v),
            Err(cvt) => panic!(),
        }
    }

    pub(crate) fn usize_to_rawval_u32(&self, u: usize) -> Result<RawVal, Infallible> {
        match u32::try_from(u) {
            Ok(v) => Ok(v.into()),
            Err(_) => panic!(),
        }
    }
}
