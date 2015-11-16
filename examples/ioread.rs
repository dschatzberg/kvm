// In this example we will construct a single CPU x86 VM which will execute
// "inb 0x01" at ring 0 with paging disabled

extern crate kvm;
extern crate memmap;

use kvm::{Capability, Exit, IoDirection, System, Vcpu, VirtualMachine};
use memmap::{Mmap, Protection};

fn main() {
    // Allocate 2MB for the guest memory
    let mut anon_mmap = Mmap::anonymous(2 * (1 << 20), Protection::ReadWrite)
                            .unwrap();
    let slice = unsafe { anon_mmap.as_mut_slice() };

    // Write the "inb 0x01" instruction at 1MB
    slice[0x100000] = 0xe4;
    slice[0x100001] = 0x01;

    // Initialize the KVM system
    let sys = System::initialize().unwrap();

    // Create a Virtual Machine
    let mut vm = VirtualMachine::create(&sys).unwrap();

    // Ensure that the VM supports memory backing with user memory
    assert!(vm.check_capability(Capability::UserMemory) > 0);
    // Set the 2 MB range to start at physical address 0
    vm.set_user_memory_region(0, slice, 0).unwrap();

    // Create a new VCPU
    let mut vcpu = Vcpu::create(&mut vm).unwrap();

    // Set supported CPUID (KVM fails without doing this)
    let mut cpuid = sys.get_supported_cpuid().unwrap();
    vcpu.set_cpuid2(&mut cpuid).unwrap();

    // Setup the special registers
    let mut sregs = vcpu.get_sregs().unwrap();

    // Set the code segment to have base 0, limit 4GB (flat segmentation)
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

    // We don't need to populate the GDT if we have our segments setup
    // cr0 - protected mode on, paging disabled
    sregs.cr0 = 0x50033;

    // Set the special registers
    vcpu.set_sregs(&sregs).unwrap();

    let mut regs = vcpu.get_regs().unwrap();
    // set the instruction pointer to 1 MB
    regs.rip = 0x100000;
    regs.rflags = 0x2;
    vcpu.set_regs(&regs).unwrap();

    // Actually run the VCPU
    let run = unsafe { vcpu.run() }.unwrap();

    // Ensure that the exit reason we get back indicates that the I/O
    // instruction was executed
    assert!(run.exit_reason == Exit::Io);
    let io = unsafe { *run.io() };
    assert!(io.direction == IoDirection::In);
    assert!(io.size == 1);
    assert!(io.port == 0x1);
    unsafe {
        println!("{:#?}", *run.io());
    }
}
