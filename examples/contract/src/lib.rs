extern crate zwasm_sdk;

pub fn invoke(_payload: *const u8, _payload_len: usize) {
    let key = "abc";
    let value = "xyz".as_bytes();
    zwasm_sdk::set(key, value);
    let out_value = zwasm_sdk::get(key);
    match String::from_utf8(out_value) {
        Ok(content) => println!("value {}", content),
        Err(err) => println!("err {}", err),
    }
}