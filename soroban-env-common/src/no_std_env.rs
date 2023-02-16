use super::{Env, EnvBase, Object, RawVal, Status, Symbol};
use core::any;

/// A dummy implementation of the [Env] trait that fails with `unimplemented!()` in
/// all functions. Useful for certain testing scenarios.
#[derive(Clone, Default)]
pub struct NoStdEnv;

impl EnvBase for NoStdEnv {
    fn as_mut_any(&mut self) -> &mut dyn any::Any {
        self
    }

    fn check_same_env(&self, _other: &Self) {}

    fn deep_clone(&self) -> Self {
        Self
    }

    fn bytes_copy_from_slice(
        &self,
        _b: Object,
        _b_pos: RawVal,
        _mem: &[u8],
    ) -> Result<Object, Status> {
        unimplemented!()
    }

    fn bytes_copy_to_slice(
        &self,
        _b: Object,
        _b_pos: RawVal,
        _mem: &mut [u8],
    ) -> Result<(), Status> {
        unimplemented!()
    }

    fn bytes_new_from_slice(&self, _mem: &[u8]) -> Result<Object, Status> {
        unimplemented!()
    }

    fn log_static_fmt_val(&self, _fmt: &'static str, _v: RawVal) -> Result<(), Status> {
        unimplemented!()
    }

    fn log_static_fmt_static_str(
        &self,
        _fmt: &'static str,
        _s: &'static str,
    ) -> Result<(), Status> {
        unimplemented!()
    }

    fn log_static_fmt_val_static_str(
        &self,
        _fmt: &'static str,
        _v: RawVal,
        _s: &'static str,
    ) -> Result<(), Status> {
        unimplemented!()
    }

    fn log_static_fmt_general(
        &self,
        _fmt: &'static str,
        _vals: &[RawVal],
        _strs: &[&'static str],
    ) -> Result<(), Status> {
        unimplemented!()
    }
}

impl Env for NoStdEnv {
    fn log_value(&self, v: RawVal) -> RawVal {
        core::unimplemented!();
    }
    fn get_invoking_contract(&self) -> Object {
        core::unimplemented!();
    }
    fn obj_cmp(&self, a: RawVal, b: RawVal) -> i64 {
        core::unimplemented!();
    }
    fn contract_event(&self, topics: Object, data: RawVal) -> RawVal {
        core::unimplemented!();
    }
    fn get_current_contract(&self) -> Object {
        core::unimplemented!();
    }
    fn get_ledger_version(&self) -> RawVal {
        core::unimplemented!();
    }
    fn get_ledger_sequence(&self) -> RawVal {
        core::unimplemented!();
    }
    fn get_ledger_timestamp(&self) -> Object {
        core::unimplemented!();
    }
    fn get_ledger_network_passphrase(&self) -> Object {
        core::unimplemented!();
    }
    fn get_current_call_stack(&self) -> Object {
        core::unimplemented!();
    }
    fn fail_with_status(&self, status: Status) -> RawVal {
        core::unimplemented!();
    }
    fn log_fmt_values(&self, fmt: Object, args: Object) -> RawVal {
        core::unimplemented!();
    }
    fn get_invoker_type(&self) -> u64 {
        core::unimplemented!();
    }
    fn get_invoking_account(&self) -> Object {
        core::unimplemented!();
    }
    fn get_ledger_network_id(&self) -> Object {
        core::unimplemented!();
    }
    fn obj_from_u64(&self, v: u64) -> Object {
        core::unimplemented!();
    }
    fn obj_to_u64(&self, obj: Object) -> u64 {
        core::unimplemented!();
    }
    fn obj_from_i64(&self, v: i64) -> Object {
        core::unimplemented!();
    }
    fn obj_to_i64(&self, obj: Object) -> i64 {
        core::unimplemented!();
    }
    fn map_new(&self) -> Object {
        core::unimplemented!();
    }
    fn map_put(&self, m: Object, k: RawVal, v: RawVal) -> Object {
        core::unimplemented!();
    }
    fn map_get(&self, m: Object, k: RawVal) -> RawVal {
        core::unimplemented!();
    }
    fn map_del(&self, m: Object, k: RawVal) -> Object {
        core::unimplemented!();
    }
    fn map_len(&self, m: Object) -> RawVal {
        core::unimplemented!();
    }
    fn map_has(&self, m: Object, k: RawVal) -> RawVal {
        core::unimplemented!();
    }
    fn map_prev_key(&self, m: Object, k: RawVal) -> RawVal {
        core::unimplemented!();
    }
    fn map_next_key(&self, m: Object, k: RawVal) -> RawVal {
        core::unimplemented!();
    }
    fn map_min_key(&self, m: Object) -> RawVal {
        core::unimplemented!();
    }
    fn map_max_key(&self, m: Object) -> RawVal {
        core::unimplemented!();
    }
    fn map_keys(&self, m: Object) -> Object {
        core::unimplemented!();
    }
    fn map_values(&self, m: Object) -> Object {
        core::unimplemented!();
    }
    fn vec_new(&self, c: RawVal) -> Object {
        core::unimplemented!();
    }
    fn vec_put(&self, v: Object, i: RawVal, x: RawVal) -> Object {
        core::unimplemented!();
    }
    fn vec_get(&self, v: Object, i: RawVal) -> RawVal {
        core::unimplemented!();
    }
    fn vec_del(&self, v: Object, i: RawVal) -> Object {
        core::unimplemented!();
    }
    fn vec_len(&self, v: Object) -> RawVal {
        core::unimplemented!();
    }
    fn vec_push_front(&self, v: Object, x: RawVal) -> Object {
        core::unimplemented!();
    }
    fn vec_pop_front(&self, v: Object) -> Object {
        core::unimplemented!();
    }
    fn vec_push_back(&self, v: Object, x: RawVal) -> Object {
        core::unimplemented!();
    }
    fn vec_pop_back(&self, v: Object) -> Object {
        core::unimplemented!();
    }
    fn vec_front(&self, v: Object) -> RawVal {
        core::unimplemented!();
    }
    fn vec_back(&self, v: Object) -> RawVal {
        core::unimplemented!();
    }
    fn vec_insert(&self, v: Object, i: RawVal, x: RawVal) -> Object {
        core::unimplemented!();
    }
    fn vec_append(&self, v1: Object, v2: Object) -> Object {
        core::unimplemented!();
    }
    fn vec_slice(&self, v: Object, start: RawVal, end: RawVal) -> Object {
        core::unimplemented!();
    }
    fn vec_first_index_of(&self, v: Object, x: RawVal) -> RawVal {
        core::unimplemented!();
    }
    fn vec_last_index_of(&self, v: Object, x: RawVal) -> RawVal {
        core::unimplemented!();
    }
    fn vec_binary_search(&self, v: Object, x: RawVal) -> u64 {
        core::unimplemented!();
    }
    fn put_contract_data(&self, k: RawVal, v: RawVal) -> RawVal {
        core::unimplemented!();
    }
    fn has_contract_data(&self, k: RawVal) -> RawVal {
        core::unimplemented!();
    }
    fn get_contract_data(&self, k: RawVal) -> RawVal {
        core::unimplemented!();
    }
    fn del_contract_data(&self, k: RawVal) -> RawVal {
        core::unimplemented!();
    }
    fn create_contract_from_ed25519(
        &self,
        v: Object,
        salt: Object,
        key: Object,
        sig: Object,
    ) -> Object {
        core::unimplemented!();
    }
    fn create_contract_from_contract(&self, v: Object, salt: Object) -> Object {
        core::unimplemented!();
    }
    fn create_token_from_ed25519(&self, salt: Object, key: Object, sig: Object) -> Object {
        core::unimplemented!();
    }
    fn create_token_from_contract(&self, salt: Object) -> Object {
        core::unimplemented!();
    }
    fn create_token_from_asset(&self, asset: Object) -> Object {
        core::unimplemented!();
    }
    fn create_contract_from_source_account(&self, v: Object, salt: Object) -> Object {
        core::unimplemented!();
    }
    fn create_token_from_source_account(&self, salt: Object) -> Object {
        core::unimplemented!();
    }
    fn call(&self, contract: Object, func: Symbol, args: Object) -> RawVal {
        core::unimplemented!();
    }
    fn try_call(&self, contract: Object, func: Symbol, args: Object) -> RawVal {
        core::unimplemented!();
    }
    fn bigint_from_u64(&self, x: u64) -> Object {
        core::unimplemented!();
    }
    fn bigint_to_u64(&self, x: Object) -> u64 {
        core::unimplemented!();
    }
    fn bigint_from_i64(&self, x: i64) -> Object {
        core::unimplemented!();
    }
    fn bigint_to_i64(&self, x: Object) -> i64 {
        core::unimplemented!();
    }
    fn bigint_add(&self, x: Object, y: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_sub(&self, x: Object, y: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_mul(&self, x: Object, y: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_div(&self, x: Object, y: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_rem(&self, x: Object, y: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_and(&self, x: Object, y: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_or(&self, x: Object, y: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_xor(&self, x: Object, y: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_shl(&self, x: Object, y: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_shr(&self, x: Object, y: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_is_zero(&self, x: Object) -> RawVal {
        core::unimplemented!();
    }
    fn bigint_neg(&self, x: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_not(&self, x: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_gcd(&self, x: Object, y: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_lcm(&self, x: Object, y: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_pow(&self, x: Object, y: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_pow_mod(&self, p: Object, q: Object, m: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_sqrt(&self, x: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_bits(&self, x: Object) -> u64 {
        core::unimplemented!();
    }
    fn bigint_to_bytes_be(&self, x: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_to_radix_be(&self, x: Object, radix: RawVal) -> Object {
        core::unimplemented!();
    }
    fn bigint_from_bytes_be(&self, sign: RawVal, bytes: Object) -> Object {
        core::unimplemented!();
    }
    fn bigint_from_radix_be(&self, sign: RawVal, buf: Object, radix: RawVal) -> Object {
        core::unimplemented!();
    }
    fn serialize_to_bytes(&self, v: RawVal) -> Object {
        core::unimplemented!();
    }
    fn deserialize_from_bytes(&self, b: Object) -> RawVal {
        core::unimplemented!();
    }
    fn bytes_copy_to_linear_memory(
        &self,
        b: Object,
        b_pos: RawVal,
        lm_pos: RawVal,
        len: RawVal,
    ) -> RawVal {
        core::unimplemented!();
    }
    fn bytes_copy_from_linear_memory(
        &self,
        b: Object,
        b_pos: RawVal,
        lm_pos: RawVal,
        len: RawVal,
    ) -> Object {
        core::unimplemented!();
    }
    fn bytes_new_from_linear_memory(&self, lm_pos: RawVal, len: RawVal) -> Object {
        core::unimplemented!();
    }
    fn bytes_new(&self) -> Object {
        core::unimplemented!();
    }
    fn bytes_put(&self, b: Object, i: RawVal, u: RawVal) -> Object {
        core::unimplemented!();
    }
    fn bytes_get(&self, b: Object, i: RawVal) -> RawVal {
        core::unimplemented!();
    }
    fn bytes_del(&self, b: Object, i: RawVal) -> Object {
        core::unimplemented!();
    }
    fn bytes_len(&self, b: Object) -> RawVal {
        core::unimplemented!();
    }
    fn bytes_push(&self, b: Object, u: RawVal) -> Object {
        core::unimplemented!();
    }
    fn bytes_pop(&self, b: Object) -> Object {
        core::unimplemented!();
    }
    fn bytes_front(&self, b: Object) -> RawVal {
        core::unimplemented!();
    }
    fn bytes_back(&self, b: Object) -> RawVal {
        core::unimplemented!();
    }
    fn bytes_insert(&self, b: Object, i: RawVal, u: RawVal) -> Object {
        core::unimplemented!();
    }
    fn bytes_append(&self, b1: Object, b2: Object) -> Object {
        core::unimplemented!();
    }
    fn bytes_slice(&self, b: Object, start: RawVal, end: RawVal) -> Object {
        core::unimplemented!();
    }
    fn hash_from_bytes(&self, x: Object) -> Object {
        core::unimplemented!();
    }
    fn hash_to_bytes(&self, x: Object) -> Object {
        core::unimplemented!();
    }
    fn public_key_from_bytes(&self, x: Object) -> Object {
        core::unimplemented!();
    }
    fn public_key_to_bytes(&self, x: Object) -> Object {
        core::unimplemented!();
    }
    fn compute_hash_sha256(&self, x: Object) -> Object {
        core::unimplemented!();
    }
    fn verify_sig_ed25519(&self, x: Object, k: Object, s: Object) -> RawVal {
        core::unimplemented!();
    }
    fn account_get_low_threshold(&self, a: Object) -> RawVal {
        core::unimplemented!();
    }
    fn account_get_medium_threshold(&self, a: Object) -> RawVal {
        core::unimplemented!();
    }
    fn account_get_high_threshold(&self, a: Object) -> RawVal {
        core::unimplemented!();
    }
    fn account_get_signer_weight(&self, a: Object, s: Object) -> RawVal {
        core::unimplemented!();
    }
    fn account_exists(&self, a: Object) -> RawVal {
        core::unimplemented!();
    }
}
