//! This module contains the [Storage] type and its supporting types, which
//! provide the [Host](crate::Host) with access to durable ledger entries.
//!
//! For more details, see the [Env](crate::Env) data access functions:
//!   - [Env::has_contract_data](crate::Env::has_contract_data)
//!   - [Env::get_contract_data](crate::Env::get_contract_data)
//!   - [Env::put_contract_data](crate::Env::put_contract_data)
//!   - [Env::del_contract_data](crate::Env::del_contract_data)

extern crate alloc;
use alloc::rc::Rc;
use core::{convert::Infallible};

use soroban_env_common::Compare;

use crate::xdr::{LedgerEntry, LedgerKey, ScHostStorageErrorCode};
use crate::Host;

use vecmap::map::VecMap;

#[derive(Clone, Default)]
pub struct Storage {
    pub map: VecMap<Rc<LedgerKey>, Option<Rc<LedgerEntry>>>, // TODO: why Rc?
}

impl Storage {
    // TODO an alternative is to not use xdr types
    // may be confusing when debugging but possibly more expedient for now

    /// Attempts to retrieve the [LedgerEntry] associated with a given
    /// [LedgerKey] in the [Storage], returning an error if the key is not
    /// found.
    pub fn get(&mut self, key: &LedgerKey) -> Result<LedgerEntry, Infallible> {
        match self.map.get(key) {
            None => panic!(),
            Some(None) => panic!(),
            Some(Some(v)) => Ok((**v).clone()),
        }
    }

    fn put_opt(
        &mut self,
        key: &LedgerKey,
        val: Option<LedgerEntry>,
    ) -> Result<(), Infallible> {
        self.map.insert(Rc::new(key.clone()), val.map(|v| {Rc::new(v)}));
        Ok(())
    }

    pub fn put(
        &mut self,
        key: &LedgerKey,
        val: &LedgerEntry,
    ) -> Result<(), Infallible> {
        self.put_opt(key, Some(val.clone()))
    }

    pub fn del(&mut self, key: &LedgerKey) -> Result<(), Infallible> {
        self.put_opt(key, None)
    }

    /// Attempts to determine the presence of a [LedgerEntry] associated with a
    /// given [LedgerKey] in the [Storage], returning `Ok(true)` if an entry
    /// with the key exists and `Ok(false)` if it does not.
    pub fn has(&mut self, key: &LedgerKey) -> Result<bool, Infallible> {
        match self.map.get::<LedgerKey>(key) {
            Some(opt_ref) => match opt_ref {
                Some(v) => Ok(true),
                None => Ok(false)
            }
            None => Ok(false),
        }
    }
}
