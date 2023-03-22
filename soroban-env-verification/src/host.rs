// use soroban_env_common::{Env, EnvBase, Object, RawVal, Status, Symbol};
use crate::{Env, EnvBase, Object, RawVal, RawValConvertible, Symbol};
use core::{any, convert::Infallible};
// mod error;
// pub use error::HostError;
use crate::{
    xdr::{
        AccountId, Asset, ContractCodeEntry, ContractDataEntry, ContractEventType, ContractId,
        CreateContractArgs, ExtensionPoint, Hash, HashIdPreimage, HostFunction, HostFunctionType,
        InstallContractCodeArgs, Int128Parts, LedgerEntryData, LedgerKey, LedgerKeyContractCode,
        ScAddress, ScContractCode, ScHostContextErrorCode, ScHostFnErrorCode, ScHostObjErrorCode,
        ScHostStorageErrorCode, ScHostValErrorCode, ScMap, ScMapEntry, ScObject, ScStatusType,
        ScUnknownErrorCode, ScVal, ScVec,
    },
    Convert, InvokerType, Status, TryFromVal, TryIntoVal, VmCaller, VmCallerEnv,
};

// mod error;
// pub use error::HostError;
mod conversion;

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
use core::cell::RefCell;

use crate::host_object::{HostObject, HostObjectType};
use crate::storage::Storage;

#[derive(Clone, Default)]
pub(crate) struct HostImpl {
    contracts: RefCell<Vec<Rc<dyn ContractFunctionSet>>>, // TODO: why Rc, why RefCell?
    objects: RefCell<Vec<HostObject>>,
    storage: RefCell<Storage>,
}
// Host is a newtype on Rc<HostImpl> so we can impl Env for it below.
#[derive(Default, Clone)]
pub struct Host(pub(crate) Rc<HostImpl>);

pub trait ContractFunctionSet {
    fn call(&self, func: &Symbol, env: &Host, args: &[RawVal]) -> Option<RawVal>;
}

impl Host {
    pub fn register_contract(&self, c: Rc<dyn ContractFunctionSet>) -> Result<Object, Infallible> {
        // TODO: take the contract ID as parameter?
        self.0.contracts.borrow_mut().push(c);
        // return index of c as contract ID
        Ok(self.usize_to_u8_vec_object(self.0.contracts.borrow().len()-1))
    }
}

impl EnvBase for Host {
    type Error = Infallible; // we'll just panic on any error

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
        b: Object,
        b_pos: RawVal,
        mem: &mut [u8],
    ) -> Result<(), Self::Error> {
        let b_pos = u32::try_from(b_pos).unwrap();
        let len = u32::try_from(mem.len()).unwrap();
        self.visit_obj(b, move |hv: &Vec<u8>| {
            let end_idx = b_pos
                .checked_add(len)
                .unwrap();
            if b_pos as usize >= mem.len() {
                panic!()
            }
            if end_idx as usize > mem.len() {
                panic!()
            }
            mem.copy_from_slice(&hv.as_slice()[b_pos as usize..end_idx as usize]);
            Ok(())
        })
    }

    fn bytes_new_from_slice(&self, mem: &[u8]) -> Result<Object, Self::Error> {
        self.add_host_object::<Vec<u8>>(mem.into())
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


impl Env for Host {
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
    fn vec_new(&self, c: RawVal) -> Result<Object, Self::Error> {
        let capacity: usize = if c.is_void() {
            0
        } else {
            self.usize_from_rawval_u32_input(c)?
        };
        self.add_host_object::<Vec<RawVal>>(Vec::with_capacity(capacity))
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
    fn vec_len(&self, v: Object) -> Result<RawVal, Self::Error> {
        let len = self.visit_obj(v, |hv: &Vec<RawVal>| Ok(hv.len()))?;
        self.usize_to_rawval_u32(len)
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
    fn put_contract_data(&self, k: RawVal, v: RawVal) -> Result<RawVal, Self::Error> {
        let key:&ScVal = &ScVal::try_from_val(self, &k).unwrap();
        let val:&ScVal = &ScVal::try_from_val(self, &v).unwrap();
        self.0.storage.borrow_mut().put(key, val);
        Ok(().into())
    }
    fn has_contract_data(&self, k: RawVal) -> Result<RawVal, Self::Error> {
        // TODO: we need to generically convert the RawVal k to a HostObject
        // There does not seem to be code that does that already (only case by case with
        // visit_obj, but that's only for complex objects (ScObject) it seems).
        // For now we are assuming that the key is a symbol
        // let obj_k:HostObject = unsafe {
            // let sym:Symbol = <Symbol as RawValConvertible>::unchecked_from_val(k);
            // let res: Result<Vec<u8>, _> = sym.into_iter().map(<u8 as TryFrom<char>>::try_from).collect();
            // res.unwrap().inject()
        // };
        let key = ScVal::try_from_val(self, &k).unwrap();
        let res = self.0.storage.borrow().has(&key).unwrap();
        Ok(RawVal::from_bool(res))
        // unimplemented!()
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
    fn call(&self, o: Object, f: Symbol, _: Object) -> Result<RawVal, Self::Error> {
        // TODO move the following to conversion.rs
        let id_raw:RawVal = o.into();
        let id_32_bytes:[u8; 32] = id_raw.try_into_val(self).unwrap();
        // panic if contract ID does not fit into 8 bytes:
        for i in 0..23 {
            if id_32_bytes[i] != 0 {
                panic!();
            }
        }
        let mut id_8_bytes:[u8; 8] = [0;8];
        id_8_bytes.copy_from_slice(&id_32_bytes[24..]);
        let id:usize = usize::from_be_bytes(id_8_bytes);

        if self.0.contracts.borrow().len() < id {
            panic!()
        }
        let v = self.0.contracts.borrow();
        let cfs_opt = v.get(id);
        let cfs = cfs_opt.unwrap();
        cfs.call(&f, self, &[]).ok_or_else(|| panic!())
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
    fn bytes_get(&self, b: Object, i: RawVal) -> Result<RawVal, Self::Error> {
        let i = self.usize_from_rawval_u32_input(i)?;
        self.visit_obj(b, |hv: &Vec<u8>| {
            hv.get(i)
                .map(|u| Into::<RawVal>::into(Into::<u32>::into(*u)))
                .ok_or_else(|| panic!())
        })
    }
    fn bytes_del(&self, _: Object, _: RawVal) -> Result<Object, Self::Error> {
        unimplemented!()
    }
    fn bytes_len(&self, b: Object) -> Result<RawVal, Self::Error> {
        let len = self.visit_obj(b, |hv: &Vec<u8>| Ok(hv.len()))?;
        self.usize_to_rawval_u32(len)
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

impl Convert<&Object, ScObject> for Host {
    type Error = Infallible;
    fn convert(&self, ob: &Object) -> Result<ScObject, Self::Error> {
        unimplemented!()
        // self.from_host_obj(*ob)
    }
}

impl Convert<Object, ScObject> for Host {
    type Error = Infallible;
    fn convert(&self, ob: Object) -> Result<ScObject, Self::Error> {
        unimplemented!()
        // self.from_host_obj(ob)
    }
}

impl Convert<&ScObject, Object> for Host {
    type Error = Infallible;
    fn convert(&self, ob: &ScObject) -> Result<Object, Self::Error> {
        unimplemented!()
        // self.to_host_obj(ob)
    }
}

impl Convert<ScObject, Object> for Host {
    type Error = Infallible;
    fn convert(&self, ob: ScObject) -> Result<Object, Self::Error> {
        unimplemented!()
        // self.to_host_obj(&ob)
    }
}

impl Host {
    pub(crate) fn add_host_object<HOT: HostObjectType>(
        &self,
        hot: HOT,
    ) -> Result<Object, Infallible> {
        let prev_len = self.0.objects.borrow().len();
        if prev_len > u32::MAX as usize {
            panic!();
        }
        self.0
            .objects
            .borrow_mut()
            .push(HOT::inject(hot));
        let handle = prev_len as u32;
        Ok(Object::from_type_and_handle(HOT::get_type(), handle))
    }

    pub(crate) unsafe fn unchecked_visit_val_obj<F, U>(
        &self,
        val: RawVal,
        f: F,
    ) -> Result<U, Infallible>
    where
        F: FnOnce(Option<&HostObject>) -> Result<U, Infallible>,
    {
        let r = self.0.objects.borrow();
        let index = <Object as RawValConvertible>::unchecked_from_val(val).get_handle() as usize;
        f(r.get(index))
    }

    pub(crate) fn visit_obj<HOT: HostObjectType, F, U>(
        &self,
        obj: Object,
        f: F,
    ) -> Result<U, Infallible>
    where
        F: FnOnce(&HOT) -> Result<U, Infallible>,
    {
        unsafe {
            self.unchecked_visit_val_obj(obj.into(), |hopt| match hopt {
                None => panic!(),
                Some(hobj) => match HOT::try_extract(hobj) {
                    None => panic!(),
                    Some(hot) => f(hot),
                },
            })
        }
    }

}
