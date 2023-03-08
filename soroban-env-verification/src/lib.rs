#![no_std]
pub use host::{Host, ContractFunctionSet};
pub use soroban_env_common::*;
use seahorn_stubs::error;
use libc_alloc::LibcAlloc;
use core::panic::PanicInfo;

#[global_allocator]
static ALLOCATOR: LibcAlloc = LibcAlloc;

#[panic_handler]
#[no_mangle]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    error()
}

mod host;
pub(crate) mod host_object;
pub mod storage;
