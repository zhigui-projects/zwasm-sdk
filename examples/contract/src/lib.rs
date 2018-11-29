extern crate zwasm_sdk;

#[no_mangle]
pub extern "C" fn invoke(_payload: *const u8, _payload_len: usize) {
    let key = "abc";
    let key1 = "abc1";
    let value = "xyz".as_bytes();
    zwasm_sdk::set(key, value);
    let out_value = zwasm_sdk::get(key);
    let out_value = &out_value[..];
    zwasm_sdk::set(key1, out_value);
}