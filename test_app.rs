
#[macro_use]
extern crate objc;

fn main() {
    unsafe {
        let nsapp = send![(Class!(NSApplication)) sharedApplication];
        let bundle = send![(Class!(NSBundle)) mainBundle];
        let info = send![bundle infoDictionary];
        let main_nib = send![info objectForKey:NSString!("NSMainNibFile")];
        send![(Class!(NSBundle)) loadNibNamed:main_nib owner:nsapp];
        send![nsapp run];
    }
}