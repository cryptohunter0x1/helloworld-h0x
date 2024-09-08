use solana_program::entrypoint::SUCCESS;
use std::os::raw::c_int;

#[no_mangle]
pub extern "C" fn getrandom(buf: *mut u8, len: usize, _flags: u32) -> c_int {
    unsafe {
        for i in 0..len {
            *buf.add(i) = 0;
        }
    }
    SUCCESS.try_into().unwrap()
}
