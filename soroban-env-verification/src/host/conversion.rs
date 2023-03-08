use crate::{Host, RawVal, Object, TryIntoVal, RawValConvertible};
use core::convert::Infallible;
use crate::xdr::{
    Hash, LedgerKey, LedgerKeyContractData, ScHostFnErrorCode, ScHostObjErrorCode,
    ScHostValErrorCode, ScStatic, ScVal, ScVec, Uint256,
};

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

    pub(crate) fn usize_to_u8_vec_object(&self, i:usize) -> Object {
        let mut i_bytes: [u8; 32] = [0; 32];
        i_bytes[24..].copy_from_slice(&i.to_be_bytes());
        let v = TryIntoVal::<Host, RawVal>::try_into_val(&i_bytes, self);
        unsafe {
            <Object as RawValConvertible>::unchecked_from_val(v.unwrap())
        }
    }
}
