#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn get_log_level() -> i32 {
  0
}

#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn config_get(_key: u64) -> i32 {
  -1
}

#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn length(_ptr: u64) -> u64 {
  0
}

#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn load_u8(_ptr: u64) -> u8 {
  0
}

#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn load_u64(_ptr: u64) -> u64 {
  0
}

#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn store_u8(_ptr: u64, _value: u8) {}

#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn store_u64(_ptr: u64, _value: u64) {}

#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn alloc(_size: u64) -> u64 {
  0
}

#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn log_trace(_ptr: u64) {}

#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn log_debug(_ptr: u64) {}

#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn log_info(_ptr: u64) {}

#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn log_warn(_ptr: u64) {}

#[cfg(all(test, not(feature = "wasm")))]
#[no_mangle]
pub extern "C" fn log_error(_ptr: u64) {}
