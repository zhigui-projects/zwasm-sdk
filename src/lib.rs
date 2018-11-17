mod externs;

pub fn get(key: &str) -> Vec<u8> {
    let key = key.as_bytes();
    let len = unsafe { externs::_get_len(key.as_ptr(), key.len()) };
    let mut val = Vec::with_capacity(len);
    unsafe { val.set_len(len) };
    unsafe { externs::_get(key.as_ptr(), key.len(), val.as_mut_ptr()) };
    val
}

pub fn set(key: &str, val: &[u8]) {
    let key = key.as_bytes();
    unsafe { externs::_set(key.as_ptr(), key.len(), val.as_ptr(), val.len()) };
}