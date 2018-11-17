extern "C" {
    pub fn _get_len(key: *const u8, key_len: usize) -> usize;
    pub fn _get(key: *const u8, key_len: usize, value_out: *mut u8);
    pub fn _set(key: *const u8, key_len: usize, value: *const u8, value_len: usize);
}