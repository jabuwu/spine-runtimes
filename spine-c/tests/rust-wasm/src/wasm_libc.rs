use std::alloc::Layout;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

#[allow(non_camel_case_types)]
pub type c_short = i16;
#[allow(non_camel_case_types)]
pub type c_ushort = u16;
#[allow(non_camel_case_types)]
pub type c_int = i32;
#[allow(non_camel_case_types)]
pub type c_uint = u32;
#[allow(non_camel_case_types)]
pub type c_long = i64;
#[allow(non_camel_case_types)]
pub type c_ulong = u64;
#[allow(non_camel_case_types)]
pub type c_schar = i8;
#[allow(non_camel_case_types)]
pub type c_char = std::ffi::c_char;
#[allow(non_camel_case_types)]
pub type c_uchar = u8;
#[allow(non_camel_case_types)]
pub type c_float = f32;
#[allow(non_camel_case_types)]
pub type c_double = f64;

#[allow(non_camel_case_types)]
type size_t = c_ulong;

#[allow(non_camel_case_types)]
#[repr(u8)]
pub enum c_void {
    __variant1,
    __variant2,
}

#[derive(Default)]
struct Allocator {
    allocations: HashMap<*const c_void, Layout>,
}
unsafe impl Send for Allocator {}
unsafe impl Sync for Allocator {}

#[allow(static_mut_refs)]
impl Allocator {
    fn singleton() -> Arc<Mutex<Allocator>> {
        static INSTANCE: OnceLock<Arc<Mutex<Allocator>>> = OnceLock::new();
        INSTANCE
            .get_or_init(|| Arc::new(Mutex::new(Allocator::default())))
            .clone()
    }

    pub fn malloc(&mut self, size: usize) -> *mut c_void {
        if size > 0 {
            let layout = std::alloc::Layout::array::<u8>(size)
                .unwrap()
                .align_to(8)
                .unwrap();
            let ptr = unsafe { std::alloc::alloc(layout) };
            self.allocations.insert(ptr as *const c_void, layout);
            ptr.cast::<c_void>()
        } else {
            std::ptr::null_mut()
        }
    }

    pub unsafe fn realloc(&mut self, ptr: *const c_void, size: usize) -> *mut c_void {
        let new_memory = self.malloc(size);
        if !new_memory.is_null() {
            let layout = self.allocations.get(&ptr).unwrap();
            memcpy(new_memory, ptr, layout.size() as size_t);
        }
        self.free(ptr);
        new_memory
    }

    #[allow(dead_code)]
    pub unsafe fn size(&mut self, ptr: *const c_void) -> usize {
        self.allocations.get(&ptr).unwrap().size()
    }

    pub unsafe fn free(&mut self, ptr: *const c_void) {
        if !ptr.is_null() {
            let layout = self.allocations.remove(&ptr).unwrap();
            unsafe { std::alloc::dealloc(ptr as *mut u8, layout) };
        }
    }

    #[allow(dead_code)]
    pub fn size_allocated(&self) -> usize {
        let mut size = 0;
        for allocation in self.allocations.values() {
            size += allocation.size();
        }
        size
    }
}

#[no_mangle]
extern "C" fn malloc(size: usize) -> *mut c_void {
    let singleton = Allocator::singleton();
    let mut allocator = singleton.lock().unwrap();
    allocator.malloc(size as usize)
}

#[no_mangle]
extern "C" fn free(ptr: *const c_void) {
    unsafe {
        if !ptr.is_null() && ptr as usize != 1 {
            let singleton = Allocator::singleton();
            let mut allocator = singleton.lock().unwrap();
            allocator.free(ptr);
        }
    }
}

#[no_mangle]
extern "C" fn memcpy(
    dst0: *mut c_void,
    src0: *const c_void,
    mut length: size_t,
) -> *mut c_void {
    unsafe {
        type Word = size_t;
        let mut dst: *mut c_char = dst0.cast::<c_char>();
        let mut src: *const c_char = src0.cast::<c_char>();
        let mut t: size_t;
        if !(length == 0 as c_int as c_ulong || dst.cast_const() == src) {
            if dst < src.cast_mut() && dst.offset(length as isize) > src.cast_mut()
                || src < dst.cast_const() && src.offset(length as isize) > dst.cast_const()
            {
                panic!();
            }
            t = src as c_long as size_t;
            if (t | dst as c_long as c_ulong)
                & (::core::mem::size_of::<Word>() as c_ulong).wrapping_sub(1 as c_int as c_ulong)
                != 0
            {
                if (t ^ dst as c_long as c_ulong)
                    & (::core::mem::size_of::<Word>() as c_ulong).wrapping_sub(1 as c_int as c_ulong)
                    != 0
                    || length < ::core::mem::size_of::<Word>() as c_ulong
                {
                    t = length;
                } else {
                    t = (::core::mem::size_of::<Word>() as c_ulong).wrapping_sub(
                        t & (::core::mem::size_of::<Word>() as c_ulong)
                            .wrapping_sub(1 as c_int as c_ulong),
                    );
                }
                length = (length as c_ulong).wrapping_sub(t) as size_t as size_t;
                loop {
                    let fresh0 = src;
                    src = src.offset(1);
                    let fresh1 = dst;
                    dst = dst.offset(1);
                    *fresh1 = *fresh0;
                    t = t.wrapping_sub(1);
                    if t == 0 {
                        break;
                    }
                }
            }
            t = length.wrapping_div(::core::mem::size_of::<Word>() as c_ulong);
            if t != 0 {
                loop {
                    *dst.cast::<Word>() = *(src as *mut Word);
                    src = src.offset(::core::mem::size_of::<Word>() as c_ulong as isize);
                    dst = dst.offset(::core::mem::size_of::<Word>() as c_ulong as isize);
                    t = t.wrapping_sub(1);
                    if t == 0 {
                        break;
                    }
                }
            }
            t = length
                & (::core::mem::size_of::<Word>() as c_ulong).wrapping_sub(1 as c_int as c_ulong);
            if t != 0 {
                loop {
                    let fresh2 = src;
                    src = src.offset(1);
                    let fresh3 = dst;
                    dst = dst.offset(1);
                    *fresh3 = *fresh2;
                    t = t.wrapping_sub(1);
                    if t == 0 {
                        break;
                    }
                }
            }
        }
        dst0
    }
}
