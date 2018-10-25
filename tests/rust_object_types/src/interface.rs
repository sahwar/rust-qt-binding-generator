/* generated by rust_qt_binding_generator */
use libc::{c_char, c_ushort, c_int};
use std::slice;
use std::char::decode_utf16;

use std::sync::Arc;
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr::null;

use implementation::*;


#[repr(C)]
pub struct COption<T> {
    data: T,
    some: bool,
}

impl<T> COption<T> {
    #![allow(dead_code)]
    fn into(self) -> Option<T> {
        if self.some {
            Some(self.data)
        } else {
            None
        }
    }
}

impl<T> From<Option<T>> for COption<T>
where
    T: Default,
{
    fn from(t: Option<T>) -> COption<T> {
        if let Some(v) = t {
            COption {
                data: v,
                some: true,
            }
        } else {
            COption {
                data: T::default(),
                some: false,
            }
        }
    }
}


pub enum QString {}

fn set_string_from_utf16(s: &mut String, str: *const c_ushort, len: c_int) {
    let utf16 = unsafe { slice::from_raw_parts(str, to_usize(len)) };
    let characters = decode_utf16(utf16.iter().cloned())
        .map(|r| r.unwrap());
    s.clear();
    s.extend(characters);
}



pub enum QByteArray {}


fn to_usize(n: c_int) -> usize {
    if n < 0 {
        panic!("Cannot cast {} to usize", n);
    }
    n as usize
}


fn to_c_int(n: usize) -> c_int {
    if n > c_int::max_value() as usize {
        panic!("Cannot cast {} to c_int", n);
    }
    n as c_int
}


pub struct ObjectQObject {}

pub struct ObjectEmitter {
    qobject: Arc<AtomicPtr<ObjectQObject>>,
    boolean_changed: fn(*mut ObjectQObject),
    bytearray_changed: fn(*mut ObjectQObject),
    f32_changed: fn(*mut ObjectQObject),
    f64_changed: fn(*mut ObjectQObject),
    i16_changed: fn(*mut ObjectQObject),
    i32_changed: fn(*mut ObjectQObject),
    i64_changed: fn(*mut ObjectQObject),
    i8_changed: fn(*mut ObjectQObject),
    optional_boolean_changed: fn(*mut ObjectQObject),
    optional_bytearray_changed: fn(*mut ObjectQObject),
    optional_string_changed: fn(*mut ObjectQObject),
    optional_u64_changed: fn(*mut ObjectQObject),
    string_changed: fn(*mut ObjectQObject),
    string_by_function_changed: fn(*mut ObjectQObject),
    u16_changed: fn(*mut ObjectQObject),
    u32_changed: fn(*mut ObjectQObject),
    u64_changed: fn(*mut ObjectQObject),
    u8_changed: fn(*mut ObjectQObject),
}

unsafe impl Send for ObjectEmitter {}

impl ObjectEmitter {
    /// Clone the emitter
    ///
    /// The emitter can only be cloned when it is mutable. The emitter calls
    /// into C++ code which may call into Rust again. If emmitting is possible
    /// from immutable structures, that might lead to access to a mutable
    /// reference. That is undefined behaviour and forbidden.
    pub fn clone(&mut self) -> ObjectEmitter {
        ObjectEmitter {
            qobject: self.qobject.clone(),
            boolean_changed: self.boolean_changed,
            bytearray_changed: self.bytearray_changed,
            f32_changed: self.f32_changed,
            f64_changed: self.f64_changed,
            i16_changed: self.i16_changed,
            i32_changed: self.i32_changed,
            i64_changed: self.i64_changed,
            i8_changed: self.i8_changed,
            optional_boolean_changed: self.optional_boolean_changed,
            optional_bytearray_changed: self.optional_bytearray_changed,
            optional_string_changed: self.optional_string_changed,
            optional_u64_changed: self.optional_u64_changed,
            string_changed: self.string_changed,
            string_by_function_changed: self.string_by_function_changed,
            u16_changed: self.u16_changed,
            u32_changed: self.u32_changed,
            u64_changed: self.u64_changed,
            u8_changed: self.u8_changed,
        }
    }
    fn clear(&self) {
        let n: *const ObjectQObject = null();
        self.qobject.store(n as *mut ObjectQObject, Ordering::SeqCst);
    }
    pub fn boolean_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.boolean_changed)(ptr);
        }
    }
    pub fn bytearray_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.bytearray_changed)(ptr);
        }
    }
    pub fn f32_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.f32_changed)(ptr);
        }
    }
    pub fn f64_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.f64_changed)(ptr);
        }
    }
    pub fn i16_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.i16_changed)(ptr);
        }
    }
    pub fn i32_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.i32_changed)(ptr);
        }
    }
    pub fn i64_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.i64_changed)(ptr);
        }
    }
    pub fn i8_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.i8_changed)(ptr);
        }
    }
    pub fn optional_boolean_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.optional_boolean_changed)(ptr);
        }
    }
    pub fn optional_bytearray_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.optional_bytearray_changed)(ptr);
        }
    }
    pub fn optional_string_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.optional_string_changed)(ptr);
        }
    }
    pub fn optional_u64_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.optional_u64_changed)(ptr);
        }
    }
    pub fn string_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.string_changed)(ptr);
        }
    }
    pub fn string_by_function_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.string_by_function_changed)(ptr);
        }
    }
    pub fn u16_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.u16_changed)(ptr);
        }
    }
    pub fn u32_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.u32_changed)(ptr);
        }
    }
    pub fn u64_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.u64_changed)(ptr);
        }
    }
    pub fn u8_changed(&mut self) {
        let ptr = self.qobject.load(Ordering::SeqCst);
        if !ptr.is_null() {
            (self.u8_changed)(ptr);
        }
    }
}

pub trait ObjectTrait {
    fn new(emit: ObjectEmitter) -> Self;
    fn emit(&mut self) -> &mut ObjectEmitter;
    fn boolean(&self) -> bool;
    fn set_boolean(&mut self, value: bool);
    fn bytearray(&self) -> &[u8];
    fn set_bytearray(&mut self, value: &[u8]);
    fn f32(&self) -> f32;
    fn set_f32(&mut self, value: f32);
    fn f64(&self) -> f64;
    fn set_f64(&mut self, value: f64);
    fn i16(&self) -> i16;
    fn set_i16(&mut self, value: i16);
    fn i32(&self) -> i32;
    fn set_i32(&mut self, value: i32);
    fn i64(&self) -> i64;
    fn set_i64(&mut self, value: i64);
    fn i8(&self) -> i8;
    fn set_i8(&mut self, value: i8);
    fn optional_boolean(&self) -> Option<bool>;
    fn set_optional_boolean(&mut self, value: Option<bool>);
    fn optional_bytearray(&self) -> Option<&[u8]>;
    fn set_optional_bytearray(&mut self, value: Option<&[u8]>);
    fn optional_string(&self) -> Option<&str>;
    fn set_optional_string(&mut self, value: Option<String>);
    fn optional_u64(&self) -> Option<u64>;
    fn set_optional_u64(&mut self, value: Option<u64>);
    fn string(&self) -> &str;
    fn set_string(&mut self, value: String);
    fn string_by_function<F>(&self, getter: F) where F: FnOnce(&str);    fn set_string_by_function(&mut self, value: String);
    fn u16(&self) -> u16;
    fn set_u16(&mut self, value: u16);
    fn u32(&self) -> u32;
    fn set_u32(&mut self, value: u32);
    fn u64(&self) -> u64;
    fn set_u64(&mut self, value: u64);
    fn u8(&self) -> u8;
    fn set_u8(&mut self, value: u8);
}

#[no_mangle]
pub extern "C" fn object_new(
    object: *mut ObjectQObject,
    object_boolean_changed: fn(*mut ObjectQObject),
    object_bytearray_changed: fn(*mut ObjectQObject),
    object_f32_changed: fn(*mut ObjectQObject),
    object_f64_changed: fn(*mut ObjectQObject),
    object_i16_changed: fn(*mut ObjectQObject),
    object_i32_changed: fn(*mut ObjectQObject),
    object_i64_changed: fn(*mut ObjectQObject),
    object_i8_changed: fn(*mut ObjectQObject),
    object_optional_boolean_changed: fn(*mut ObjectQObject),
    object_optional_bytearray_changed: fn(*mut ObjectQObject),
    object_optional_string_changed: fn(*mut ObjectQObject),
    object_optional_u64_changed: fn(*mut ObjectQObject),
    object_string_changed: fn(*mut ObjectQObject),
    object_string_by_function_changed: fn(*mut ObjectQObject),
    object_u16_changed: fn(*mut ObjectQObject),
    object_u32_changed: fn(*mut ObjectQObject),
    object_u64_changed: fn(*mut ObjectQObject),
    object_u8_changed: fn(*mut ObjectQObject),
) -> *mut Object {
    let object_emit = ObjectEmitter {
        qobject: Arc::new(AtomicPtr::new(object)),
        boolean_changed: object_boolean_changed,
        bytearray_changed: object_bytearray_changed,
        f32_changed: object_f32_changed,
        f64_changed: object_f64_changed,
        i16_changed: object_i16_changed,
        i32_changed: object_i32_changed,
        i64_changed: object_i64_changed,
        i8_changed: object_i8_changed,
        optional_boolean_changed: object_optional_boolean_changed,
        optional_bytearray_changed: object_optional_bytearray_changed,
        optional_string_changed: object_optional_string_changed,
        optional_u64_changed: object_optional_u64_changed,
        string_changed: object_string_changed,
        string_by_function_changed: object_string_by_function_changed,
        u16_changed: object_u16_changed,
        u32_changed: object_u32_changed,
        u64_changed: object_u64_changed,
        u8_changed: object_u8_changed,
    };
    let d_object = Object::new(object_emit);
    Box::into_raw(Box::new(d_object))
}

#[no_mangle]
pub unsafe extern "C" fn object_free(ptr: *mut Object) {
    Box::from_raw(ptr).emit().clear();
}

#[no_mangle]
pub unsafe extern "C" fn object_boolean_get(ptr: *const Object) -> bool {
    (&*ptr).boolean()
}

#[no_mangle]
pub unsafe extern "C" fn object_boolean_set(ptr: *mut Object, v: bool) {
    (&mut *ptr).set_boolean(v);
}

#[no_mangle]
pub unsafe extern "C" fn object_bytearray_get(
    ptr: *const Object,
    p: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, c_int),
) {
    let o = &*ptr;
    let v = o.bytearray();
    let s: *const c_char = v.as_ptr() as (*const c_char);
    set(p, s, to_c_int(v.len()));
}

#[no_mangle]
pub unsafe extern "C" fn object_bytearray_set(ptr: *mut Object, v: *const c_char, len: c_int) {
    let o = &mut *ptr;
    let v = slice::from_raw_parts(v as *const u8, to_usize(len));
    o.set_bytearray(v);
}

#[no_mangle]
pub unsafe extern "C" fn object_f32_get(ptr: *const Object) -> f32 {
    (&*ptr).f32()
}

#[no_mangle]
pub unsafe extern "C" fn object_f32_set(ptr: *mut Object, v: f32) {
    (&mut *ptr).set_f32(v);
}

#[no_mangle]
pub unsafe extern "C" fn object_f64_get(ptr: *const Object) -> f64 {
    (&*ptr).f64()
}

#[no_mangle]
pub unsafe extern "C" fn object_f64_set(ptr: *mut Object, v: f64) {
    (&mut *ptr).set_f64(v);
}

#[no_mangle]
pub unsafe extern "C" fn object_i16_get(ptr: *const Object) -> i16 {
    (&*ptr).i16()
}

#[no_mangle]
pub unsafe extern "C" fn object_i16_set(ptr: *mut Object, v: i16) {
    (&mut *ptr).set_i16(v);
}

#[no_mangle]
pub unsafe extern "C" fn object_i32_get(ptr: *const Object) -> i32 {
    (&*ptr).i32()
}

#[no_mangle]
pub unsafe extern "C" fn object_i32_set(ptr: *mut Object, v: i32) {
    (&mut *ptr).set_i32(v);
}

#[no_mangle]
pub unsafe extern "C" fn object_i64_get(ptr: *const Object) -> i64 {
    (&*ptr).i64()
}

#[no_mangle]
pub unsafe extern "C" fn object_i64_set(ptr: *mut Object, v: i64) {
    (&mut *ptr).set_i64(v);
}

#[no_mangle]
pub unsafe extern "C" fn object_i8_get(ptr: *const Object) -> i8 {
    (&*ptr).i8()
}

#[no_mangle]
pub unsafe extern "C" fn object_i8_set(ptr: *mut Object, v: i8) {
    (&mut *ptr).set_i8(v);
}

#[no_mangle]
pub unsafe extern "C" fn object_optional_boolean_get(ptr: *const Object) -> COption<bool> {
    match (&*ptr).optional_boolean() {
        Some(value) => COption { data: value, some: true },
        None => COption { data: bool::default(), some: false}
    }
}

#[no_mangle]
pub unsafe extern "C" fn object_optional_boolean_set(ptr: *mut Object, v: bool) {
    (&mut *ptr).set_optional_boolean(Some(v));
}

#[no_mangle]
pub unsafe extern "C" fn object_optional_boolean_set_none(ptr: *mut Object) {
    let o = &mut *ptr;
    o.set_optional_boolean(None);
}

#[no_mangle]
pub unsafe extern "C" fn object_optional_bytearray_get(
    ptr: *const Object,
    p: *mut QByteArray,
    set: fn(*mut QByteArray, *const c_char, c_int),
) {
    let o = &*ptr;
    let v = o.optional_bytearray();
    if let Some(v) = v {
        let s: *const c_char = v.as_ptr() as (*const c_char);
        set(p, s, to_c_int(v.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn object_optional_bytearray_set(ptr: *mut Object, v: *const c_char, len: c_int) {
    let o = &mut *ptr;
    let v = slice::from_raw_parts(v as *const u8, to_usize(len));
    o.set_optional_bytearray(Some(v.into()));
}

#[no_mangle]
pub unsafe extern "C" fn object_optional_bytearray_set_none(ptr: *mut Object) {
    let o = &mut *ptr;
    o.set_optional_bytearray(None);
}

#[no_mangle]
pub unsafe extern "C" fn object_optional_string_get(
    ptr: *const Object,
    p: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let o = &*ptr;
    let v = o.optional_string();
    if let Some(v) = v {
        let s: *const c_char = v.as_ptr() as (*const c_char);
        set(p, s, to_c_int(v.len()));
    }
}

#[no_mangle]
pub unsafe extern "C" fn object_optional_string_set(ptr: *mut Object, v: *const c_ushort, len: c_int) {
    let o = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, v, len);
    o.set_optional_string(Some(s));
}

#[no_mangle]
pub unsafe extern "C" fn object_optional_string_set_none(ptr: *mut Object) {
    let o = &mut *ptr;
    o.set_optional_string(None);
}

#[no_mangle]
pub unsafe extern "C" fn object_optional_u64_get(ptr: *const Object) -> COption<u64> {
    match (&*ptr).optional_u64() {
        Some(value) => COption { data: value, some: true },
        None => COption { data: u64::default(), some: false}
    }
}

#[no_mangle]
pub unsafe extern "C" fn object_optional_u64_set(ptr: *mut Object, v: u64) {
    (&mut *ptr).set_optional_u64(Some(v));
}

#[no_mangle]
pub unsafe extern "C" fn object_optional_u64_set_none(ptr: *mut Object) {
    let o = &mut *ptr;
    o.set_optional_u64(None);
}

#[no_mangle]
pub unsafe extern "C" fn object_string_get(
    ptr: *const Object,
    p: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let o = &*ptr;
    let v = o.string();
    let s: *const c_char = v.as_ptr() as (*const c_char);
    set(p, s, to_c_int(v.len()));
}

#[no_mangle]
pub unsafe extern "C" fn object_string_set(ptr: *mut Object, v: *const c_ushort, len: c_int) {
    let o = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, v, len);
    o.set_string(s);
}

#[no_mangle]
pub unsafe extern "C" fn object_string_by_function_get(
    ptr: *const Object,
    p: *mut QString,
    set: fn(*mut QString, *const c_char, c_int),
) {
    let o = &*ptr;
    o.string_by_function(|v| {
        let s: *const c_char = v.as_ptr() as (*const c_char);
        set(p, s, to_c_int(v.len()));
    });
}

#[no_mangle]
pub unsafe extern "C" fn object_string_by_function_set(ptr: *mut Object, v: *const c_ushort, len: c_int) {
    let o = &mut *ptr;
    let mut s = String::new();
    set_string_from_utf16(&mut s, v, len);
    o.set_string_by_function(s);
}

#[no_mangle]
pub unsafe extern "C" fn object_u16_get(ptr: *const Object) -> u16 {
    (&*ptr).u16()
}

#[no_mangle]
pub unsafe extern "C" fn object_u16_set(ptr: *mut Object, v: u16) {
    (&mut *ptr).set_u16(v);
}

#[no_mangle]
pub unsafe extern "C" fn object_u32_get(ptr: *const Object) -> u32 {
    (&*ptr).u32()
}

#[no_mangle]
pub unsafe extern "C" fn object_u32_set(ptr: *mut Object, v: u32) {
    (&mut *ptr).set_u32(v);
}

#[no_mangle]
pub unsafe extern "C" fn object_u64_get(ptr: *const Object) -> u64 {
    (&*ptr).u64()
}

#[no_mangle]
pub unsafe extern "C" fn object_u64_set(ptr: *mut Object, v: u64) {
    (&mut *ptr).set_u64(v);
}

#[no_mangle]
pub unsafe extern "C" fn object_u8_get(ptr: *const Object) -> u8 {
    (&*ptr).u8()
}

#[no_mangle]
pub unsafe extern "C" fn object_u8_set(ptr: *mut Object, v: u8) {
    (&mut *ptr).set_u8(v);
}
