#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

#[no_mangle]
extern "C" fn malloc(size: usize) -> *mut c_void {
    0 as *mut c_void
}

#[no_mangle]
extern "C" fn free(ptr: *const c_void) {}
