#[no_mangle]
extern crate libc;

use libc::c_char;
use std::ffi::CStr;
use std::str;

#[no_mangle]
pub unsafe extern "C" fn say_hello(cname: *const i8) {
    let c_buf: *const c_char = unsafe { cname };
    let c_str: &CStr = unsafe { CStr::from_ptr(c_buf) };
    let str_slice: &str = c_str.to_str().unwrap();
    println!("Hello, {}!", str_slice);
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
