
#![feature(libc)]
extern crate libc;
#[macro_use]
extern crate objc;

#[allow(non_snake_case)]
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
        } as usize;
        assert!(str.len() == len);
    }

    #[test]
    pub fn test_class() {
        let NSObject = unsafe { Class!(NSObject) };
        assert!(NSObject != objc::Nil);
    }

    #[test]
    pub fn test_selector() {
        unsafe {
            let selector = selector!("stringWithUTF8String:");
            assert!(selector != objc::NULL);
            let string = objc::NSStringFromSelector(selector);
            assert!(string != objc::nil);
            objc::NSLog(string);
        }
    }

    #[test]
    pub fn test_send() {
        unsafe {
            let NSString = Class!(NSString);
            assert!(NSString != objc::Nil);
            let selector = selector!("stringWithUTF8String:");
            assert!(selector != objc::NULL);
            let string = send![(NSString) stringWithUTF8String:c_str!("test string end")];
            assert!(string != objc::nil);
            objc::NSLog(string);
        }
    }

    #[test]
    pub fn test_send2() {
        unsafe {
            let NSString = Class!(NSString);
            assert!(NSString != objc::Nil);
            let selector = selector!("stringWithUTF8String:");
            assert!(selector != objc::NULL);
            let string = send![(NSString) stringWithUTF8String:c_str!("test string end")];
            assert!(string != objc::nil);
            objc::NSLog(string);
        }
    }

    #[test]
    pub fn test_string() {
        unsafe {
            let NSString = Class!(NSString);
            let str1 = objc_string!("test");
            let str2 = send![(NSString) stringWithUTF8String:c_str!("test")];
            assert!(send![str1 length] == 4);
            assert!(send![str2 length] == 4);
            assert!(send![(str1) isEqual:str2] != 0);
        }
    }

    #[test]
    pub fn test_nslog() {
        unsafe {
            let NSString = Class!(NSString);
            let name = objc::NSStringFromClass(NSString);
            assert!(name != objc::nil);
            assert!(send![name length] == 8);
            objc::NSLog(name);

            let format = objc_string!("Test Log");
            assert!(format != objc::nil);
            objc::NSLog(format);
        }
    }

}
