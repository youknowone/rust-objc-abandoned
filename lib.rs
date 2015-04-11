#![crate_name="objc"]
#![crate_type="rlib"]
#![crate_type="dylib"]
#![feature(libc)]

extern crate libc;
use libc::intptr_t;

use std::ffi::CString;

pub type SEL = libc::intptr_t;

#[allow(non_upper_case_globals)]
pub static NSASCIIStringEncoding: isize = 1;
#[allow(non_upper_case_globals)]
pub static NSUTF8StringEncoding: isize = 4;

#[repr(C)]
pub type id = libc::intptr_t;

pub type Class = id;

#[allow(non_upper_case_globals)]
pub static nil: id = 0;
#[allow(non_upper_case_globals)]
pub static Nil: Class = 0;
#[repr(C)]
pub static NULL: SEL = 0;

/*
pub impl std::rc::RcBoxPtr for id {

}
*/
#[link(name = "Foundation", kind = "framework")]
extern {
    // objc
    pub fn objc_getClass(name: *const libc::c_char) -> Class;
    pub fn objc_msgSend(obj: id, sel: SEL, ...) -> id;
    pub fn sel_registerName(name: *const libc::c_char) -> SEL;

    // Foundation
    pub fn NSLog(fmt: id, ...);
    pub fn NSStringFromClass(cls: Class) -> id;
    pub fn NSStringFromSelector(sel: SEL) -> id;
}

#[macro_export]
macro_rules! NSLog{
    ($fmt:expr, $($arg:expr)*) => (
        unsafe { ::objc::NSLog(objc_string!($fmt), ($($arg),*)); }
    );
}

pub fn _str_to_c_str(s: &str) -> *const i8 {
    CString::new(s).unwrap().as_ptr()
}

#[macro_export]
macro_rules! c_str{
    ($val:expr) => (
        ::objc::_str_to_c_str($val)
    );
}


#[macro_export]
macro_rules! objc_string{
    ($val:expr) => (
        send![(Class!(NSString)) stringWithUTF8String:c_str!($val)]
    );
}


#[macro_export]
macro_rules! Class{
    ($name:ident) => (
        ::objc::objc_getClass(c_str!(stringify!($name)))
    );
}

#[macro_export]
macro_rules! selector{
    ($name:expr) => (
        ::objc::sel_registerName(c_str!($name))
    );
    ($name:ident) => (
        ::objc::sel_registerName(c_str!(stringify!($name)))
    );
}

#[macro_export]
macro_rules! send{
    [($obj:expr) $sel:ident] => (
        ::objc::objc_msgSend($obj, selector!(stringify!($sel)))
    );
    [($obj:expr) $($sel:ident : $arg:expr)+] => (
        ::objc::objc_msgSend($obj, selector!(concat!($(stringify!($sel), ":"),+)), $($arg),+)
    );
    [$obj:ident $sel:ident] => (
        ::objc::objc_msgSend($obj, selector!(stringify!($sel)))
    );

    /*
    [$obj:ident $($sel:ident : $arg:expr)+] => (
        ::objc::objc_msgSend($obj, selector!(concat!($(stringify!($sel), ":"),+)), $($arg),+)
    );
    */
}

