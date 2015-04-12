
#[macro_use]
extern crate objc;

#[allow(non_snake_case)]
fn main() {
    unsafe {
        let NSAutoreleasePool = Class!(NSAutoreleasePool);
        let NSApplication = Class!(NSApplication);
        let NSBundle = Class!(NSBundle);
        let NSMenu = Class!(NSMenu);
        let NSMenuItem = Class!(NSMenuItem);
        let NSWindow = Class!(NSWindow);

        send![(send![NSAutoreleasePool alloc]) init];

        let NSApp = send![NSApplication sharedApplication];
        send![NSApp setActivationPolicy:0];

        let menu = send![(send![NSMenu alloc]) init];
        send![NSApp setMainMenu:menu];

        let window = send![(send![NSWindow alloc]) init];
        send![window makeKeyAndOrderFront:objc::nil];

        /*
        let bundle = send![NSBundle mainBundle];
        let info = send![bundle infoDictionary];
        let main_nib = send![info objectForKey:NSString!("NSMainNibFile")];
        send![(NSBundle) loadNibNamed:main_nib owner:NSApp];
        */
        send![NSApp activateIgnoringOtherApps:1];
        send![NSApp run];
    }
}