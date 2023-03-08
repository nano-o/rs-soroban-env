use crate::{Host, RawVal};
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

    /// Converts a [`RawVal`] to an [`ScVal`] and combines it with the currently-executing
    /// [`ContractID`] to produce a [`Key`], that can be used to access ledger [`Storage`].
    pub fn storage_key_from_rawval(&self, k: RawVal) -> Result<LedgerKey, Infallible> {
        Ok(LedgerKey::ContractData(LedgerKeyContractData {
            contract_id: self.get_current_contract_id_internal()?,
            key: self.from_host_val(k)?,
        }))
    }
}
