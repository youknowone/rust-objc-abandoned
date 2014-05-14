#![license = "BSD simplified"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![feature(macro_rules)]

#![macro_escape]

extern crate libc;
use libc::intptr_t;

pub type Class = libc::intptr_t;
pub type SEL = libc::intptr_t;
#[allow(non_camel_case_types)]
pub type id = libc::intptr_t;
#[warn(non_camel_case_types)]

pub static Nil: Class = 0 as Class;
pub static nil: id = 0 as id;

pub static NSASCIIStringEncoding: int = 1;
pub static NSUTF8StringEncoding: int = 4;

#[link(name = "Foundation", kind = "framework")]
extern {
    // objc
    pub fn objc_getClass(name: *libc::c_char) -> Class;
    pub fn objc_msgSend(obj: id, sel: SEL, ...) -> id;
    pub fn sel_registerName(name: *libc::c_char) -> SEL;

    // Foundation
    pub fn NSLog(fmt: id, ...);
    pub fn NSStringFromClass(cls: Class) -> id;
    pub fn NSStringFromSelector(sel: SEL) -> id;
}

#[macro_export]
macro_rules! NSLog(
    ($fmt:expr, $($arg:expr)*) => (
        unsafe { objc::NSLog(objc_string!($fmt), ($($arg),*)); }
    );
)

#[macro_export]
macro_rules! c_str(
    ($val:expr) => (
        $val.to_c_str().with_ref(|c_buffer| {
            c_buffer
        });
    );
)


#[macro_export]
macro_rules! objc_string(
    ($val:expr) => (
        unsafe { send![Class!(NSString) stringWithUTF8String:c_str!($val)] }
    );
)


#[macro_export]
macro_rules! Class(
    ($name:ident) => (
        unsafe { objc::objc_getClass(c_str!(stringify!($name))) }
    );
)

#[macro_export]
macro_rules! selector(
    ($name:expr) => (
        unsafe { objc::sel_registerName(c_str!($name)) }
    );
    ($name:ident) => (
        unsafe { objc::sel_registerName(c_str!(stringify!($name))) }
    );
)

#[macro_export]
macro_rules! send(
    [$obj:expr $sel:ident] => (
        unsafe { objc::objc_msgSend($obj, selector!(stringify!($sel))) }
    );
    [$obj:expr $($sel:ident : $arg:expr)+ $(, $argx:expr)*] => (
        unsafe {
            objc::objc_msgSend($obj, selector!(concat!($(stringify!($sel), ":"),+)), $($arg),+)
        }
    );
)

