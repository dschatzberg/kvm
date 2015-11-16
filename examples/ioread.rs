extern crate kvm;
extern crate memmap;

use kvm::{Exit, IoDirection, System, Vcpu, VirtualMachine};
use memmap::{Mmap, Protection};

fn main() {
    let mut anon_mmap = Mmap::anonymous(2 * (1 << 20), Protection::ReadWrite)
                            .unwrap();
    let slice = unsafe { anon_mmap.as_mut_slice() };

    slice[0x100000] = 0xe4;
    slice[0x100001] = 0x01;

    let sys = System::initialize().unwrap();

    let mut vm = VirtualMachine::create(&sys).unwrap();

    vm.set_user_memory_region(0, slice, 0).unwrap();

    let mut vcpu = Vcpu::create(&mut vm).unwrap();

    let mut cpuid = sys.get_supported_cpuid().unwrap();
    vcpu.set_cpuid2(&mut cpuid).unwrap();

    let mut sregs = vcpu.get_sregs().unwrap();

    sregs.cs.base = 0x0;
    sregs.cs.limit = 0xffffffff;
    sregs.cs.selector = 0x8;
    sregs.cs._type = 0xb;
    sregs.cs.present = 1;
    sregs.cs.dpl = 0;
    sregs.cs.db = 0;
    sregs.cs.s = 1;
    sregs.cs.l = 0;
    sregs.cs.g = 1;
    sregs.cs.avl = 0;

    sregs.ss.base = 0x0;
    sregs.ss.limit = 0xffffffff;
    sregs.ss.selector = 0;
    sregs.ss._type = 0;
    sregs.ss.present = 0;
    sregs.ss.dpl = 0;
    sregs.ss.db = 1;
    sregs.ss.s = 0;
    sregs.ss.l = 0;
    sregs.ss.g = 1;
    sregs.ss.avl = 0;

    sregs.ds.base = 0x0;
    sregs.ds.limit = 0xffffffff;
    sregs.ds.selector = 0;
    sregs.ds._type = 0;
    sregs.ds.present = 0;
    sregs.ds.dpl = 0;
    sregs.ds.db = 1;
    sregs.ds.s = 0;
    sregs.ds.l = 0;
    sregs.ds.g = 1;
    sregs.ds.avl = 0;

    sregs.es.base = 0x0;
    sregs.es.limit = 0xffffffff;
    sregs.es.selector = 0;
    sregs.es._type = 0;
    sregs.es.present = 0;
    sregs.es.dpl = 0;
    sregs.es.db = 1;
    sregs.es.s = 0;
    sregs.es.l = 0;
    sregs.es.g = 1;
    sregs.es.avl = 0;

    sregs.fs.base = 0x0;
    sregs.fs.limit = 0xffffffff;
    sregs.fs.selector = 0;
    sregs.fs._type = 0;
    sregs.fs.present = 0;
    sregs.fs.dpl = 0;
    sregs.fs.db = 1;
    sregs.fs.s = 0;
    sregs.fs.l = 0;
    sregs.fs.g = 1;
    sregs.fs.avl = 0;

    sregs.gs.base = 0x0;
    sregs.gs.limit = 0xffffffff;
    sregs.gs.selector = 0;
    sregs.gs._type = 0;
    sregs.gs.present = 0;
    sregs.gs.dpl = 0;
    sregs.gs.db = 1;
    sregs.gs.s = 0;
    sregs.gs.l = 0;
    sregs.gs.g = 1;
    sregs.gs.avl = 0;

    sregs.cr0 = 0x50033;
    sregs.cr4 = 0x1046b0;

    vcpu.set_sregs(&sregs).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    regs.rip = 0x100000;
    regs.rflags = 0x246;
    vcpu.set_regs(&regs).unwrap();
    let run = unsafe { vcpu.run() }.unwrap();
    assert!(run.exit_reason == Exit::Io);
    let io = unsafe { *run.io() };
    assert!(io.direction == IoDirection::In);
    assert!(io.size == 1);
    assert!(io.port == 0x1);
    unsafe {
        println!("{:#?}", *run.io());
    }
}
