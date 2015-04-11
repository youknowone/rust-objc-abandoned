
#[macro_use]
extern crate objc;

struct Crawler {
    url: objc::id,
    data: objc::id,
}

impl Crawler {
    pub fn with_url(s: &str) -> Crawler {
        let url = unsafe { send![(send![(Class!(NSURL)) URLWithString:NSString!(s)]) retain] };
        Crawler { url: url, data: objc::nil }
    }

    pub fn crawl(&mut self) -> objc::id {
        NSLog!("saved url: %@", self.url);
        self.data = unsafe { send![(send![(Class!(NSData)) dataWithContentsOfURL:self.url]) retain] };
        return self.data
    }

    pub fn print(&self) {
        unsafe {
            let string = send![(send![(Class!(NSString)) alloc]) initWithData:self.data encoding:objc::NSUTF8StringEncoding];
            assert!(string != objc::nil);
            objc::NSLog(NSString!("%@"), string);
            send![string release];
        }
    }
}

pub fn main() {
    let mut crawler = Crawler::with_url("http://rust-kr.org/");
    crawler.crawl();
    crawler.print();
}
