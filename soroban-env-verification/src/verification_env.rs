use soroban_env_common::{Env, EnvBase, Object, RawVal, Status, Symbol};
use core::{any, convert::Infallible};

// use crate::xdr::{
        // AccountId, Asset, ContractCodeEntry, ContractDataEntry, ContractEventType, ContractId,
        // CreateContractArgs, ExtensionPoint, Hash, HashIdPreimage, HostFunction, HostFunctionType,
        // InstallContractCodeArgs, Int128Parts, LedgerEntryData, LedgerKey, LedgerKeyContractCode,
        // ScAddress, ScContractCode, ScHostContextErrorCode, ScHostFnErrorCode, ScHostObjErrorCode,
        // ScHostStorageErrorCode, ScHostValErrorCode, ScMap, ScMapEntry, ScObject, ScStatusType,
        // ScUnknownErrorCode, ScVal, ScVec,
    // };
// use Convert, InvokerType, Status, TryFromVal, TryIntoVal, VmCaller, VmCallerEnv;

// TODO: we need an allocator specific for seahorn (only using malloc)
// use crate::libc_alloc::LibcAlloc;
// #[global_allocator]
// static ALLOCATOR: LibcAlloc = LibcAlloc;

// Can't do that as it pulls in std:
// extern crate std;
// use std::alloc::System;
// #[global_allocator]
// static A: System = System;

extern crate alloc;
use alloc::vec::Vec;
use alloc::rc::Rc;

#[derive(Clone, Default)]
pub struct NoStdEnv {
    // TODO: a map from contract id to ContractFunctionSet
    contracts: Vec<Rc<dyn ContractFunctionSet>>
}

pub trait ContractFunctionSet {
    fn call(&self, func: &Symbol, env: &NoStdEnv, args: &[RawVal]) -> Option<RawVal>;
}

impl NoStdEnv {
    fn register_contract(&mut self, c: Rc<dyn ContractFunctionSet>) -> Result<(), Infallible> {
        // TODO: return contract ID (Object representing BytesN<32>?
        self.contracts.push(c);
        Ok(())
    }
}

impl EnvBase for NoStdEnv {
    type Error = Infallible; // TODO: is this what we want for verification?

    fn as_mut_any(&mut self) -> &mut dyn any::Any {
        self
    }

    fn check_same_env(&self, _other: &Self) {}

    fn deep_clone(&self) -> Self {
        unimplemented!()
    }

    fn bytes_copy_from_slice(
        &self,
        _b: Object,
        _b_pos: RawVal,
        _mem: &[u8],
    ) -> Result<Object, Self::Error> {
        unimplemented!()
    }

    fn bytes_copy_to_slice(
        &self,
        _b: Object,
        _b_pos: RawVal,
        _mem: &mut [u8],
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn bytes_new_from_slice(&self, _mem: &[u8]) -> Result<Object, Self::Error> {
        unimplemented!()
    }

    fn log_static_fmt_val(&self, _fmt: &'static str, _v: RawVal) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn log_static_fmt_static_str(
        &self,
        _fmt: &'static str,
        _s: &'static str,
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn log_static_fmt_val_static_str(
        &self,
        _fmt: &'static str,
        _v: RawVal,
        _s: &'static str,
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }

    fn log_static_fmt_general(
        &self,
        _fmt: &'static str,
        _vals: &[RawVal],
        _strs: &[&'static str],
    ) -> Result<(), Self::Error> {
        unimplemented!()
    }
}


impl Env for NoStdEnv {
    fn log_value(&self, _: RawVal) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn get_invoking_contract(&self) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn obj_cmp(&self, _: RawVal, _: RawVal) -> Result<i64, Self::Error> {
        unimplemented!()
    }
    fn contract_event(&self, _: Object, _: RawVal) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn get_current_contract_id(&self) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn get_ledger_version(&self) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn get_ledger_sequence(&self) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn get_ledger_timestamp(&self) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn get_current_call_stack(&self) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn fail_with_status(&self, _: Status) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn log_fmt_values(&self, _: Object, _: Object) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn get_invoker_type(&self) -> Result<u64, Self::Error> {
        unimplemented!()
    }
    fn get_ledger_network_id(&self) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn get_current_contract_address(&self) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn obj_from_u64(&self, _: u64) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn obj_to_u64(&self, _: Object) -> Result<u64, Self::Error> {
        unimplemented!()
    }
    fn obj_from_i64(&self, _: i64) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn obj_to_i64(&self, _: Object) -> Result<i64, Self::Error> {
        unimplemented!()
    }
    fn obj_from_u128_pieces(&self, _: u64, _: u64) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn obj_to_u128_lo64(&self, _: Object) -> Result<u64, Self::Error> {
        unimplemented!()
    }
    fn obj_to_u128_hi64(&self, _: Object) -> Result<u64, Self::Error> {
        unimplemented!()
    }
    fn obj_from_i128_pieces(&self, _: u64, _: u64) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn obj_to_i128_lo64(&self, _: Object) -> Result<u64, Self::Error> {
        unimplemented!()
    }
    fn obj_to_i128_hi64(&self, _: Object) -> Result<u64, Self::Error> {
        unimplemented!()
    }
    fn map_new(&self) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn map_put(&self, _: Object, _: RawVal, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn map_get(&self, _: Object, _: RawVal) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn map_del(&self, _: Object, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn map_len(&self, _: Object) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn map_has(&self, _: Object, _: RawVal) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn map_prev_key(&self, _: Object, _: RawVal) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn map_next_key(&self, _: Object, _: RawVal) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn map_min_key(&self, _: Object) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn map_max_key(&self, _: Object) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn map_keys(&self, _: Object) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn map_values(&self, _: Object) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn vec_new(&self, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn vec_put(&self, _: Object, _: RawVal, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn vec_get(&self, _: Object, _: RawVal) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn vec_del(&self, _: Object, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn vec_len(&self, _: Object) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn vec_push_front(&self, _: Object, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn vec_pop_front(&self, _: Object) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn vec_push_back(&self, _: Object, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn vec_pop_back(&self, _: Object) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn vec_front(&self, _: Object) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn vec_back(&self, _: Object) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn vec_insert(&self, _: Object, _: RawVal, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn vec_append(&self, _: Object, _: Object) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn vec_slice(&self, _: Object, _: RawVal, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn vec_first_index_of(&self, _: Object, _: RawVal) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn vec_last_index_of(&self, _: Object, _: RawVal) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn vec_binary_search(&self, _: Object, _: RawVal) -> Result<u64, Self::Error> {
        unimplemented!()
    }
    fn put_contract_data(&self, _: RawVal, _: RawVal) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn has_contract_data(&self, _: RawVal) -> Result<RawVal, Self::Error> {
        // Ok(RawVal::from_bool(true))
        unimplemented!()
    }
    fn get_contract_data(&self, _: RawVal) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn del_contract_data(&self, _: RawVal) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn create_contract_from_contract(&self, _: Object, _: Object) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn call(&self, _: Object, _: Symbol, _: Object) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn try_call(&self, _: Object, _: Symbol, _: Object) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn serialize_to_bytes(&self, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn deserialize_from_bytes(&self, _: Object) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn bytes_copy_to_linear_memory(
        &self,
        _: Object,
        _: RawVal,
        _: RawVal,
        _: RawVal,
    ) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn bytes_copy_from_linear_memory(
        &self,
        _: Object,
        _: RawVal,
        _: RawVal,
        _: RawVal,
    ) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn bytes_new_from_linear_memory(&self, _: RawVal, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn bytes_new(&self) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn bytes_put(&self, _: Object, _: RawVal, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn bytes_get(&self, _: Object, _: RawVal) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn bytes_del(&self, _: Object, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn bytes_len(&self, _: Object) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn bytes_push(&self, _: Object, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn bytes_pop(&self, _: Object) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn bytes_front(&self, _: Object) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn bytes_back(&self, _: Object) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn bytes_insert(&self, _: Object, _: RawVal, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn bytes_append(&self, _: Object, _: Object) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn bytes_slice(&self, _: Object, _: RawVal, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn compute_hash_sha256(&self, _: Object) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn verify_sig_ed25519(&self, _: Object, _: Object, _: Object) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn require_auth_for_args(&self, _: Object, _: Object) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn require_auth(&self, _: Object) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
    fn dummy0(&self) -> Result<RawVal, Self::Error> {
        unimplemented!()
    }
}
