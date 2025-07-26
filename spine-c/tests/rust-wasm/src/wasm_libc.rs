use std::alloc::Layout;
use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

#[allow(non_camel_case_types)]
pub type c_bool = bool;
#[allow(non_camel_case_types)]
pub type c_short = i16;
#[allow(non_camel_case_types)]
pub type c_ushort = u16;
#[allow(non_camel_case_types)]
pub type c_int = i32;
#[allow(non_camel_case_types)]
pub type c_uint = u32;
#[allow(non_camel_case_types)]
pub type c_long = i32;
#[allow(non_camel_case_types)]
pub type c_ulong = u32;
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
type size_t = c_uint;

#[allow(non_camel_case_types)]
pub type c_void = std::ffi::c_void;

#[no_mangle]
extern "C" fn spine_wasm_todo() {
    todo!();
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
            spine_wasm_memcpy(new_memory, ptr, layout.size() as size_t);
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
extern "C" fn spine_wasm_malloc(size: size_t) -> *mut c_void {
    let singleton = Allocator::singleton();
    let mut allocator = singleton.lock().unwrap();
    allocator.malloc(size as usize)
}

// TODO: why do we need this?
#[no_mangle]
extern "C" fn malloc(size: size_t) -> *mut c_void {
    spine_wasm_malloc(size)
}

#[no_mangle]
unsafe extern "C" fn spine_wasm_realloc(ptr: *mut c_void, size: size_t) -> *mut c_void {
    spine_wasm_free(ptr);
    spine_wasm_malloc(size)
}

#[no_mangle]
unsafe extern "C" fn spine_wasm_free(ptr: *mut c_void) {
    if !ptr.is_null() && ptr as usize != 1 {
        let singleton = Allocator::singleton();
        let mut allocator = singleton.lock().unwrap();
        allocator.free(ptr);
    }
}

// TODO: why do we need this?
#[no_mangle]
unsafe extern "C" fn free(ptr: *mut c_void) {
    spine_wasm_free(ptr);
}

#[no_mangle]
extern "C" fn spine_wasm_isnan(arg: c_double) -> c_bool {
    c_double::is_nan(arg)
}

#[no_mangle]
extern "C" fn spine_wasm_isspace(c: c_int) -> c_int {
    if c == '\t' as i32
        || c == '\n' as i32
        || c == '\u{b}' as i32
        || c == '\u{c}' as i32
        || c == '\r' as i32
        || c == ' ' as i32
    {
        1 as c_int
    } else {
        0 as c_int
    }
}

#[no_mangle]
unsafe extern "C" fn spine_wasm_isdigit(c: c_int) -> c_int {
    ((c as c_uint).wrapping_sub('0' as i32 as c_uint) < 10 as c_int as c_uint) as c_int
}

#[no_mangle]
unsafe extern "C" fn spine_wasm_isalpha(c: c_int) -> c_int {
    if c >= 'a' as i32 && c <= 'z' as i32 || c >= 'A' as i32 && c <= 'Z' as i32 {
        1 as c_int
    } else {
        0 as c_int
    }
}

#[no_mangle]
unsafe extern "C" fn spine_wasm_isupper(c: c_int) -> c_int {
    if c >= 'A' as i32 && c <= 'Z' as i32 {
        1 as c_int
    } else {
        0 as c_int
    }
}

#[no_mangle]
extern "C" fn spine_wasm_abs(n: c_int) -> c_int {
    n.abs()
}

#[no_mangle]
extern "C" fn spine_wasm_fmod(x: c_double, y: c_double) -> c_double {
    x % y
}

#[no_mangle]
extern "C" fn spine_wasm_atan2(y: c_double, x: c_double) -> c_double {
    y.atan2(x)
}

#[no_mangle]
extern "C" fn spine_wasm_cos(arg: c_double) -> c_double {
    arg.cos()
}

#[no_mangle]
extern "C" fn spine_wasm_sin(arg: c_double) -> c_double {
    arg.sin()
}

#[no_mangle]
extern "C" fn spine_wasm_sqrt(arg: c_double) -> c_double {
    arg.sqrt()
}

#[no_mangle]
extern "C" fn spine_wasm_acos(arg: c_double) -> c_double {
    arg.acos()
}

#[no_mangle]
extern "C" fn spine_wasm_nan(arg: *const c_char) -> c_double {
    c_double::NAN
}

#[no_mangle]
extern "C" fn spine_wasm_pow(base: c_double, exponent: c_double) -> c_double {
    base.powf(exponent)
}

#[no_mangle]
extern "C" fn spine_wasm_ceil(arg: c_double) -> c_double {
    arg.ceil()
}

#[no_mangle]
extern "C" fn spine_wasm_rand() -> c_int {
    9 // TODO
}

#[no_mangle]
unsafe extern "C" fn spine_wasm_strcmp(mut s1: *const c_char, mut s2: *const c_char) -> c_int {
    loop {
        let fresh0 = s2;
        s2 = s2.offset(1);
        if *s1 as c_int != *fresh0 as c_int {
            break;
        }
        let fresh1 = s1;
        s1 = s1.offset(1);
        if *fresh1 as c_int == 0 as c_int {
            return 0 as c_int;
        }
    }
    s2 = s2.offset(-1);
    *(s1 as *mut c_uchar) as c_int - *(s2 as *mut c_uchar) as c_int
}

#[no_mangle]
unsafe extern "C" fn spine_wasm_strlen(str: *const i8) -> size_t {
    let mut s: *const c_char;
    s = str;
    while *s != 0 {
        s = s.offset(1);
    }
    s.offset_from(str) as size_t
}

#[no_mangle]
unsafe extern "C" fn spine_wasm_memcpy(
    dst0: *mut c_void,
    src0: *const c_void,
    mut length: size_t,
) -> *mut c_void {
    type Word = size_t;
    let mut dst: *mut c_char = dst0.cast::<c_char>();
    let mut src: *const c_char = src0.cast::<c_char>();
    let mut t: size_t;
    if !(length == 0 || dst.cast_const() == src) {
        if dst < src.cast_mut() && dst.offset(length as isize) > src.cast_mut()
            || src < dst.cast_const() && src.offset(length as isize) > dst.cast_const()
        {
            panic!();
        }
        t = src as size_t;
        if (t | dst as size_t)
            & (::core::mem::size_of::<Word>() as size_t).wrapping_sub(1 as size_t)
            != 0
        {
            if (t ^ dst as size_t)
                & (::core::mem::size_of::<Word>() as size_t).wrapping_sub(1 as size_t)
                != 0
                || length < ::core::mem::size_of::<Word>() as size_t
            {
                t = length;
            } else {
                t = (::core::mem::size_of::<Word>() as size_t).wrapping_sub(
                    t & (::core::mem::size_of::<Word>() as size_t)
                        .wrapping_sub(1 as c_int as size_t),
                );
            }
            length = (length as size_t).wrapping_sub(t) as size_t as size_t;
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
        t = length.wrapping_div(::core::mem::size_of::<Word>() as size_t);
        if t != 0 {
            loop {
                *dst.cast::<Word>() = *(src as *mut Word);
                src = src.offset(::core::mem::size_of::<Word>() as size_t as isize);
                dst = dst.offset(::core::mem::size_of::<Word>() as size_t as isize);
                t = t.wrapping_sub(1);
                if t == 0 {
                    break;
                }
            }
        }
        t = length & (::core::mem::size_of::<Word>() as size_t).wrapping_sub(1 as size_t);
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

#[no_mangle]
unsafe extern "C" fn spine_wasm_memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void {
    for offset in 0..n {
        (*(s.cast::<u8>()).offset(offset as isize)) = c as u8;
    }
    s
}

#[no_mangle]
unsafe extern "C" fn spine_wasm_strrchr(mut p: *const c_char, ch: c_int) -> *mut c_void {
    let mut save: *mut c_char = std::ptr::null_mut::<c_char>();
    loop {
        if *p as c_int == ch {
            save = p.cast_mut();
        }
        if *p == 0 {
            return save as *mut c_void;
        }
        p = p.offset(1);
    }
}

#[no_mangle]
unsafe extern "C" fn spine_wasm_strdup(str1: *const c_char) -> *mut c_char {
    // TODO: transpile from clib
    let len = spine_wasm_strlen(str1) + 1;
    let new_str = spine_wasm_malloc(len) as *mut c_char;
    spine_wasm_memcpy(new_str as *mut c_void, str1 as *const c_void, len);
    new_str
}

#[no_mangle]
unsafe extern "C" fn spine_wasm_strtol(
    nptr: *const c_char,
    endptr: *mut *mut c_char,
    mut base: c_int,
) -> c_long {
    let mut s: *const c_char;
    let mut acc: c_long;
    let mut cutoff: c_long;
    let mut c: c_int;
    let neg: c_int;
    let mut any: c_int;
    let mut cutlim: c_int;
    s = nptr;
    loop {
        let fresh0 = s;
        s = s.offset(1);
        c = *fresh0 as c_uchar as c_int;
        if spine_wasm_isspace(c) == 0 {
            break;
        }
    }
    if c == '-' as i32 {
        neg = 1 as c_int;
        let fresh1 = s;
        s = s.offset(1);
        c = *fresh1 as c_int;
    } else {
        neg = 0 as c_int;
        if c == '+' as i32 {
            let fresh2 = s;
            s = s.offset(1);
            c = *fresh2 as c_int;
        }
    }
    if (base == 0 as c_int || base == 16 as c_int)
        && c == '0' as i32
        && (*s as c_int == 'x' as i32 || *s as c_int == 'X' as i32)
    {
        c = *s.offset(1 as c_int as isize) as c_int;
        s = s.offset(2 as c_int as isize);
        base = 16 as c_int;
    }
    if base == 0 as c_int {
        base = if c == '0' as i32 {
            8 as c_int
        } else {
            10 as c_int
        };
    }
    cutoff = (if neg != 0 {
        (c_long::MAX as c_ulong).wrapping_neg()
    } else {
        (c_long::MAX - 1) as c_ulong
    }) as c_long;
    cutlim = (cutoff % base as c_long) as c_int;
    cutoff /= base as c_long;
    if neg != 0 {
        if cutlim > 0 as c_int {
            cutlim -= base;
            cutoff += 1 as c_int as c_long;
        }
        cutlim = -cutlim;
    }
    acc = 0 as c_int as c_long;
    any = 0 as c_int;
    loop {
        if spine_wasm_isdigit(c) != 0 {
            c -= '0' as i32;
        } else {
            if spine_wasm_isalpha(c) == 0 {
                break;
            }
            c -= if spine_wasm_isupper(c) != 0 {
                'A' as i32 - 10 as c_int
            } else {
                'a' as i32 - 10 as c_int
            };
        }
        if c >= base {
            break;
        }
        if any >= 0 as c_int {
            if neg != 0 {
                if acc < cutoff || acc == cutoff && c > cutlim {
                    any = -(1 as c_int);
                    acc = (c_long::MAX as c_ulong).wrapping_neg() as c_long;
                } else {
                    any = 1 as c_int;
                    acc *= base as c_long;
                    acc -= c as c_long;
                }
            } else if acc > cutoff || acc == cutoff && c > cutlim {
                any = -(1 as c_int);
                acc = (c_long::MAX - 1) as c_long;
            } else {
                any = 1 as c_int;
                acc *= base as c_long;
                acc += c as c_long;
            }
        }
        let fresh3 = s;
        s = s.offset(1);
        c = *fresh3 as c_uchar as c_int;
    }
    if !endptr.is_null() {
        *endptr = (if any != 0 {
            s.offset(-(1 as c_int as isize))
        } else {
            nptr
        })
        .cast_mut();
    }
    acc
}

#[no_mangle]
extern "C" fn spine_wasm_strtoul(
    str: *const c_char,
    str_end: *mut *mut c_char,
    base: c_int,
) -> c_ulong {
    todo!()
}

#[no_mangle]
pub unsafe extern "C" fn spine_wasm_strcpy(
    mut to: *mut c_char,
    mut from: *const c_char,
) -> *mut c_char {
    let save: *mut c_char = to;
    loop {
        *to = *from;
        if *to as c_int == '\0' as i32 {
            break;
        }
        from = from.offset(1);
        to = to.offset(1);
    }
    save
}

#[no_mangle]
extern "C" fn spine_wasm_strncat(
    dest: *mut c_char,
    src: *const c_char,
    count: size_t,
) -> *mut c_char {
    todo!()
}

#[no_mangle]
unsafe extern "C" fn spine_wasm_strncmp(
    mut s1: *const c_char,
    mut s2: *const c_char,
    mut n: size_t,
) -> c_int {
    if n == 0 {
        return 0;
    }
    loop {
        let fresh0 = s2;
        s2 = s2.offset(1);
        if *s1 as c_int != *fresh0 as c_int {
            s2 = s2.offset(-1);
            return *(s1 as *mut c_uchar) as c_int - *(s2 as *mut c_uchar) as c_int;
        }
        let fresh1 = s1;
        s1 = s1.offset(1);
        if *fresh1 as c_int == 0 as c_int {
            break;
        }
        n = n.wrapping_sub(1);
        if n == 0 {
            break;
        }
    }
    0 as c_int
}

#[no_mangle]
extern "C" fn spine_wasm_strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int {
    todo!()
}

#[no_mangle]
extern "C" fn spine_wasm_fflush(s: *mut c_void) -> c_int {
    todo!();
}
