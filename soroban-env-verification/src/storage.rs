extern crate alloc;
use alloc::rc::Rc;
use core::convert::Infallible;

// use soroban_env_common::Compare;

// use crate::xdr::{LedgerEntry, LedgerKey};
// use crate::{Host, host_object::HostObject};
use crate::xdr::ScVal;

use vecmap::map::VecMap;

#[derive(Clone, Default)]
pub struct Storage {
    // TODO: use ScVal (has contract ID) and ScVal
    pub(crate) map: VecMap<Rc<ScVal>, Option<Rc<ScVal>>>, // TODO: why Rc?
}

impl Storage {
    pub(crate) fn get(&mut self, key: &ScVal) -> Result<ScVal, Infallible> {
        match self.map.get(key) {
            None => panic!(),
            Some(None) => panic!(),
            Some(Some(v)) => Ok((**v).clone()),
        }
    }

    fn put_opt(
        &mut self,
        key: &ScVal,
        val: Option<ScVal>,
    ) -> Result<(), Infallible> {
        self.map.insert(Rc::new(key.clone()), val.map(|v| {Rc::new(v)}));
        Ok(())
    }

    pub(crate) fn put(
        &mut self,
        key: &ScVal,
        val: &ScVal,
    ) -> Result<(), Infallible> {
        self.put_opt(key, Some(val.clone()))
    }

    pub(crate) fn del(&mut self, key: &ScVal) -> Result<(), Infallible> {
        self.put_opt(key, None)
    }

    /// Attempts to determine the presence of a [ScVal] associated with a
    /// given [ScVal] in the [Storage], returning `Ok(true)` if an entry
    /// with the key exists and `Ok(false)` if it does not.
    pub(crate) fn has(&self, key: &ScVal) -> Result<bool, Infallible> {
        match self.map.get::<ScVal>(key) {
            Some(opt_ref) => match opt_ref {
                Some(v) => Ok(true),
                None => Ok(false)
            }
            None => Ok(false),
        }
    }
}
