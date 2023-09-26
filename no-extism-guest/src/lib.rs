use std::mem;
use std::ptr;
use std::slice;

#[no_mangle]
pub extern "C" fn say_hello(mem_addr: *const u8, len: usize, ret_len_addr: *mut u32) -> i32 {
    // read input string
    let data: Vec<u8> = unsafe { slice::from_raw_parts(mem_addr, len).to_vec() };

    // create output string
    let greeting = format!(
        "Hello there, {}.  That was a lot of memory management to pass a string!",
        String::from_utf8_lossy(&data)
    );

    // determine the length of the output string and set that value in the address of ret_len_addr
    unsafe {
        ptr::write(ret_len_addr, greeting.len() as u32);
    };

    greeting.as_ptr() as i32
}

#[no_mangle]
pub extern "C" fn alloc() -> *const u8 {
    let mut buf = Vec::with_capacity(1024);
    let ptr = buf.as_mut_ptr();

    // tell Rust not to clean this up
    mem::forget(buf);

    ptr
}

#[no_mangle]
pub unsafe extern "C" fn dealloc(ptr: &mut u8) {
    let _ = Vec::from_raw_parts(ptr, 0, 1024);
}
