use std::io;
use std::fs;

pub struct Kvm {
    handle: fs::File
}

impl Kvm {
    pub fn open() -> io::Result<Kvm> {
        let f = try!(fs::File::open("/dev/kvm"));
        Ok(Kvm { handle: f })
    }
}

#[test]
fn open_test() {
    Kvm::open().unwrap();
}
