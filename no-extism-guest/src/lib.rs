use std::ptr;
use std::slice;

#[no_mangle]
pub extern "C" fn say_hello(mem_addr: *const u8, len: usize, ret_len_addr: *mut u32) -> i32 {
    let data: Vec<u8> = unsafe { slice::from_raw_parts(mem_addr, len).to_vec() };
    let greeting = format!(
        "Hello there, {}.  That was a lot of memory management to pass a string!",
        String::from_utf8_lossy(&data)
    );

    // determine the length of the modified string and set that value to the in/out ret_len_addr
    unsafe {
        ptr::write(ret_len_addr, greeting.len() as u32);
    };

    greeting.as_ptr() as i32
}
