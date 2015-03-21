#![feature(macro_rules)]
#![feature(phase)]

extern crate std;
extern crate libc;
#[phase(syntax,link)]
extern crate objc;

#[cfg(test)]
mod tests {
    use libc;
    use objc;

    #[test]
    pub fn test_c_str() {
        let str = "test c_str end";
        let c_str = c_str!(str);
        let len = unsafe {
            libc::strlen(c_str)
        } as uint;
        assert!(str.len() == len);
    }

    #[test]
    pub fn test_class() {
        let NSObject = Class!(NSObject);
        assert!(NSObject != objc::Nil);
    }

    #[test]
    pub fn test_selector() {
        let selector = selector!("stringWithUTF8String:");
        assert!(selector != objc::NULL);
        let string = unsafe { objc::NSStringFromSelector(selector) };
        assert!(string != objc::nil);
        unsafe { objc::NSLog(string); }
    }

    #[test]
    pub fn test_send() {
        let NSString = Class!(NSString);
        assert!(NSString != objc::Nil);
        let selector = selector!("stringWithUTF8String:");
        assert!(selector != objc::NULL);
        let string = send![NSString; stringWithUTF8String:c_str!("test string end")];
        assert!(string != objc::nil);
        unsafe { objc::NSLog(string); }
    }

    #[test]
    pub fn test_send2() {
        let NSString = Class!(NSString);
        assert!(NSString != objc::Nil);
        let selector = selector!("stringWithUTF8String:");
        assert!(selector != objc::NULL);
        let string = send![NSString; stringWithUTF8String:c_str!("test string end")];
        assert!(string != objc::nil);
        unsafe { objc::NSLog(string); }
    }

    #[test]
    pub fn test_string() {
        let NSString = Class!(NSString);
        let str1 = objc_string!("test");
        let str2 = send![NSString; stringWithUTF8String:c_str!("test")];
        assert!(send![str1; length].value == 4);
        assert!(send![str2; length].value == 4);
        assert!(send![str1; isEqual:str2].value != 0);
    }

    #[test]
    pub fn test_nslog() {
        let NSString = Class!(NSString);
        let name = unsafe { objc::NSStringFromClass(NSString) };
        assert!(name != objc::nil);
        assert!(send![name; length].value == 8);
        unsafe { objc::NSLog(name); }

        let format = objc_string!("Test Log");
        assert!(format != objc::nil);
        unsafe { objc::NSLog(format); }
    }

}
