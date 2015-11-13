extern crate errno;
#[macro_use]
extern crate ioctl;
extern crate libc;
#[macro_use]
extern crate log;
extern crate memmap;

#[cfg(target_arch = "x86_64")]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

use errno::{errno, Errno};
use libc::{E2BIG, ENOMEM};
use std::fs::{File, OpenOptions};
use std::io::{self, Error, ErrorKind};
use std::mem;
use std::os::unix::io::{AsRawFd, FromRawFd};

use memmap::{Mmap, Protection};

const API_VERSION: i32 = 12;

const KVMIO: i32 = 0xAE;

ioctl!(none kvm_get_api_version with KVMIO, 0x00);
ioctl!(bad kvm_create_vm with io!(KVMIO, 0x01));
ioctl!(bad kvm_check_extension with io!(KVMIO, 0x03));
ioctl!(bad kvm_get_vcpu_mmap_size with io!(KVMIO, 0x04));
ioctl!(bad kvm_get_supported_cpuid with iorw!(KVMIO, 0x05, 8));
ioctl!(bad kvm_create_vcpu with io!(KVMIO, 0x41));
ioctl!(write kvm_set_user_memory_region with KVMIO, 0x46; UserspaceMemoryRegion);
ioctl!(read kvm_get_regs with KVMIO, 0x81; Regs);
ioctl!(write kvm_set_regs with KVMIO, 0x82; Regs);
ioctl!(read kvm_get_sregs with KVMIO, 0x83; Sregs);
ioctl!(write kvm_set_sregs with KVMIO, 0x84; Sregs);
ioctl!(bad kvm_set_cpuid2 with iow!(KVMIO, 0x90, 8));

#[derive(Debug)]
pub struct System {
    fd: File,
}

#[derive(Debug)]
pub struct VirtualMachine<'a> {
    fd: File,
    sys: &'a System,
    mem_slots: Vec<&'a mut [u8]>,
    num_vcpus: u32,
    check_extension: bool,
}

pub type Result<T> = io::Result<T>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Capability {
    Irqchip,
    Hlt,
    MmuShadowCacheControl,
    UserMemory,
    SetTssAddr,
    Vapic = 6,
    ExtCpuid,
    ClockSource,
    NrVcpus,
    NrMemSlots,
    Pit,
    NopIoDelay,
    PvMmu,
    MpState,
    CoalescedMmio,
    SyncMmu,
    IoMmu = 18,
    DestroyMemoryRegionWorks = 21,
    UserNmi,
    MaxVcpus = 66,
    CheckExtensionVm = 105,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct UserspaceMemoryRegion {
    pub slot: u32,
    pub flags: u32,
    pub guest_phys_addr: u64,
    pub memory_size: u64,
    pub userspace_addr: u64,
}

pub struct Vcpu<'a> {
    fd: File,
    vm: &'a VirtualMachine<'a>,
    mmap: Mmap,
}

#[repr(C)]
#[derive(Copy)]
pub struct Run {
    pub request_interrupt_window: u8,
    pub padding1: [u8; 7usize],
    pub exit_reason: u32,
    pub ready_for_interrupt_injection: u8,
    pub if_flag: u8,
    pub padding2: [u8; 2usize],
    pub cr8: u64,
    pub apic_base: u64,
    pub _bindgen_data_1_: [u64; 32usize],
    pub kvm_valid_regs: u64,
    pub kvm_dirty_regs: u64,
    pub s: Union_Unnamed26,
}
impl Run {
    pub unsafe fn hw(&mut self) -> *mut Struct_Unnamed9 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn fail_entry(&mut self) -> *mut Struct_Unnamed10 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn ex(&mut self) -> *mut Struct_Unnamed11 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn io(&mut self) -> *mut Struct_Unnamed12 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn debug(&mut self) -> *mut Struct_Unnamed13 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn mmio(&mut self) -> *mut Struct_Unnamed14 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn hypercall(&mut self) -> *mut Struct_Unnamed15 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn tpr_access(&mut self) -> *mut Struct_Unnamed16 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn s390_sieic(&mut self) -> *mut Struct_Unnamed17 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn s390_reset_flags(&mut self) -> *mut u64 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn s390_ucontrol(&mut self) -> *mut Struct_Unnamed18 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn dcr(&mut self) -> *mut Struct_Unnamed19 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn internal(&mut self) -> *mut Struct_Unnamed20 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn osi(&mut self) -> *mut Struct_Unnamed21 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn papr_hcall(&mut self) -> *mut Struct_Unnamed22 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn s390_tsch(&mut self) -> *mut Struct_Unnamed23 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn epr(&mut self) -> *mut Struct_Unnamed24 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn system_event(&mut self) -> *mut Struct_Unnamed25 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn padding(&mut self) -> *mut [::libc::c_char; 256usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Run {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Run {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed9 {
    pub hardware_exit_reason: u64,
}
impl ::std::clone::Clone for Struct_Unnamed9 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed9 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed10 {
    pub hardware_entry_failure_reason: u64,
}
impl ::std::clone::Clone for Struct_Unnamed10 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed10 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed11 {
    pub exception: u32,
    pub error_code: u32,
}
impl ::std::clone::Clone for Struct_Unnamed11 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed11 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed12 {
    pub direction: u8,
    pub size: u8,
    pub port: u16,
    pub count: u32,
    pub data_offset: u64,
}
impl ::std::clone::Clone for Struct_Unnamed12 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed12 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed13 {
    pub arch: DebugExitArch,
}
impl ::std::clone::Clone for Struct_Unnamed13 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed13 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed14 {
    pub phys_addr: u64,
    pub data: [u8; 8usize],
    pub len: u32,
    pub is_write: u8,
}
impl ::std::clone::Clone for Struct_Unnamed14 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed14 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed15 {
    pub nr: u64,
    pub args: [u64; 6usize],
    pub ret: u64,
    pub longmode: u32,
    pub pad: u32,
}
impl ::std::clone::Clone for Struct_Unnamed15 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed15 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed16 {
    pub rip: u64,
    pub is_write: u32,
    pub pad: u32,
}
impl ::std::clone::Clone for Struct_Unnamed16 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed16 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed17 {
    pub icptcode: u8,
    pub ipa: u16,
    pub ipb: u32,
}
impl ::std::clone::Clone for Struct_Unnamed17 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed17 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed18 {
    pub trans_exc_code: u64,
    pub pgm_code: u32,
}
impl ::std::clone::Clone for Struct_Unnamed18 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed18 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed19 {
    pub dcrn: u32,
    pub data: u32,
    pub is_write: u8,
}
impl ::std::clone::Clone for Struct_Unnamed19 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed19 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed20 {
    pub suberror: u32,
    pub ndata: u32,
    pub data: [u64; 16usize],
}
impl ::std::clone::Clone for Struct_Unnamed20 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed20 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed21 {
    pub gprs: [u64; 32usize],
}
impl ::std::clone::Clone for Struct_Unnamed21 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed21 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed22 {
    pub nr: u64,
    pub ret: u64,
    pub args: [u64; 9usize],
}
impl ::std::clone::Clone for Struct_Unnamed22 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed22 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed23 {
    pub subchannel_id: u16,
    pub subchannel_nr: u16,
    pub io_int_parm: u32,
    pub io_int_word: u32,
    pub ipb: u32,
    pub dequeued: u8,
}
impl ::std::clone::Clone for Struct_Unnamed23 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed23 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed24 {
    pub epr: u32,
}
impl ::std::clone::Clone for Struct_Unnamed24 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed24 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed25 {
    pub _type: u32,
    pub flags: u64,
}
impl ::std::clone::Clone for Struct_Unnamed25 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Struct_Unnamed25 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed26 {
    pub _bindgen_data_: [u8; 1024usize],
}
impl Union_Unnamed26 {
    pub unsafe fn regs(&mut self) -> *mut SyncRegs {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn padding(&mut self) -> *mut [::libc::c_char; 1024usize] {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed26 {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for Union_Unnamed26 {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}

impl System {
    pub fn initialize() -> Result<Self> {
        let f = try!(OpenOptions::new()
                         .read(true)
                         .write(true)
                         .open("/dev/kvm"));
        let vers = unsafe { kvm_get_api_version(f.as_raw_fd()) };
        if vers == API_VERSION {
            Ok(System { fd: f })
        } else {
            Err(Error::new(ErrorKind::NotFound, "Unexpected API Version"))
        }
    }

    pub fn check_capability(&self, cap: Capability) -> i32 {
        unsafe {
            kvm_check_extension(self.fd.as_raw_fd(), cap as usize as *mut u8)
        }
    }

    pub fn recommended_vcpus(&self) -> u32 {
        let r = self.check_capability(Capability::NrVcpus);
        if r != 0 {
            r as u32
        } else {
            // From api.txt: If the KVM_CAP_NR_VCPUS does not exist, you should
            // assume that max_vcpus is 4 cpus max.
            4
        }
    }

    pub fn max_vcpus(&self) -> u32 {
        let r = self.check_capability(Capability::MaxVcpus);
        if r != 0 {
            r as u32
        } else {
            self.recommended_vcpus()
        }
    }

    fn get_vcpu_mmap_size(&self) -> usize {
        let ret = unsafe {
            kvm_get_vcpu_mmap_size(self.fd.as_raw_fd(), 0usize as *mut u8)
        };
        assert!(ret > 0 && ret as usize >= mem::size_of::<Run>());
        ret as usize
    }
}

#[cfg(target_arch = "x86_64")]
const CPUID_ENTRIES: u32 = 64;

#[cfg(target_arch = "x86_64")]
impl System {
    pub fn get_supported_cpuid(&self) -> Result<CpuidHandle> {
        let mut nent = CPUID_ENTRIES;
        loop {
            let mut c = CpuidHandle::new(nent);
            let err = unsafe {
                let (ptr, _): (*mut u8, usize) = mem::transmute(&mut *c);
                kvm_get_supported_cpuid(self.fd.as_raw_fd(), ptr)
            };
            if err != 0 {
                if errno() == Errno(E2BIG) {
                    nent *= 2;
                    continue;
                } else if errno() == Errno(ENOMEM) {
                    nent = c.nent;
                    continue;
                } else {
                    return Err(Error::last_os_error());
                }
            } else {
                return Ok(c);
            }
        }
    }
}

pub const MEM_LOG_DIRTY_PAGES: u32 = 1 << 0;
pub const MEM_READONLY: u32 = 1 << 1;

impl<'a> VirtualMachine<'a> {
    pub fn create(s: &'a System) -> Result<Self> {
        let f = unsafe {
            kvm_create_vm(s.fd.as_raw_fd(), 0 as usize as *mut u8)
        };
        if f == -1 {
            return Err(Error::last_os_error());
        }

        let check_extension =
            s.check_capability(Capability::CheckExtensionVm) != 0;
        Ok(VirtualMachine {
            fd: unsafe { File::from_raw_fd(f) },
            sys: s,
            mem_slots: Vec::new(),
            num_vcpus: 0,
            check_extension: check_extension,
        })
    }

    pub fn check_capability(&mut self, cap: Capability) -> i32 {
        if self.check_extension {
            unsafe {
                kvm_check_extension(self.fd.as_raw_fd(),
                                    cap as usize as *mut u8)
            }
        } else {
            self.sys.check_capability(cap)
        }
    }

    pub fn set_user_memory_region(&mut self,
                                  phys_addr: u64,
                                  user_addr: &'a mut [u8],
                                  flags: u32)
                                  -> Result<()> {
        let slot = self.mem_slots.len();
        let mut region = UserspaceMemoryRegion {
            slot: slot as u32,
            flags: flags,
            guest_phys_addr: phys_addr,
            memory_size: user_addr.len() as u64,
            userspace_addr: user_addr.as_mut_ptr() as u64,
        };
        let ret = unsafe {
            kvm_set_user_memory_region(self.fd.as_raw_fd(), &mut region)
        };
        if ret == 0 {
            self.mem_slots.push(user_addr);
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Other, "Unknown Error"))
        }
    }
}

impl<'a> Vcpu<'a> {
    pub fn create(vm: &'a mut VirtualMachine<'a>) -> Result<Self> {
        if vm.num_vcpus >= vm.sys.max_vcpus() {
            return Err(Error::new(ErrorKind::AlreadyExists,
                                  "Would exceed max_vcpus"));
        } else if vm.num_vcpus >= vm.sys.recommended_vcpus() {
            warn!("Exceeding recommended_vcpus");
        }
        let fd = unsafe {
            File::from_raw_fd(kvm_create_vcpu(vm.fd.as_raw_fd(),
                            vm.num_vcpus as usize as *mut u8))
        };
        vm.num_vcpus += 1;
        let mmap_size = vm.sys.get_vcpu_mmap_size();
        let m = try!(Mmap::open_with_offset(&fd, Protection::ReadWrite, 0, mmap_size));
        Ok(Vcpu {
            fd: fd,
            vm: vm,
            mmap: m,
        })
    }

    pub unsafe fn run(&mut self) -> &mut Run {
        return &mut *(self.mmap.mut_ptr() as *mut Run)
    }

    pub fn get_regs(&self) -> Result<Regs> {
        let mut regs = Regs::default();
        let ret = unsafe {
            kvm_get_regs(self.fd.as_raw_fd(), &mut regs)
        };
        if ret == 0 {
            Ok(regs)
        } else {
            Err(Error::last_os_error())
        }
    }

    pub fn set_regs(&mut self, regs: &Regs) -> Result<()> {
        let ret = unsafe {
            kvm_set_regs(self.fd.as_raw_fd(), regs)
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}

#[cfg(target_arch = "x86_64")]
impl<'a> Vcpu<'a> {
    pub fn set_cpuid2(&mut self, cpuid: &mut Cpuid2) -> Result<()> {
        let ret = unsafe {
            let (ptr, _): (*mut u8, usize) = mem::transmute(cpuid);
            kvm_set_cpuid2(self.fd.as_raw_fd(), ptr)
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }

    pub fn get_sregs(&self) -> Result<Sregs> {
        let mut sregs = Sregs::default();
        let ret = unsafe {
            kvm_get_sregs(self.fd.as_raw_fd(), &mut sregs)
        };
        if ret == 0 {
            Ok(sregs)
        } else {
            Err(Error::last_os_error())
        }
    }

    pub fn set_sregs(&mut self, sregs: &Sregs) -> Result<()> {
        let ret = unsafe {
            kvm_set_sregs(self.fd.as_raw_fd(), sregs)
        };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}

#[test]
fn init_test() {
    System::initialize().unwrap();
}

#[test]
fn check_capability_test() {
    let h = System::initialize().unwrap();
    assert!(h.check_capability(Capability::UserMemory) != 0);
}

#[test]
fn vcpus_test() {
    let h = System::initialize().unwrap();
    assert!(h.max_vcpus() >= h.recommended_vcpus());
}

#[test]
fn create_vm_test() {
    let h = System::initialize().unwrap();
    VirtualMachine::create(&h).unwrap();
}

#[test]
fn set_memory_test() {
    let mut anon_mmap = Mmap::anonymous(16 * (1 << 12), Protection::ReadWrite)
                            .unwrap();
    let slice = unsafe { anon_mmap.as_mut_slice() };
    let h = System::initialize().unwrap();
    let mut vm = VirtualMachine::create(&h).unwrap();
    vm.set_user_memory_region(0, slice, 0).unwrap();
}

#[test]
fn create_vcpu_test() {
    let h = System::initialize().unwrap();
    let mut vm = VirtualMachine::create(&h).unwrap();
    Vcpu::create(&mut vm).unwrap();
}

#[cfg(target_arch = "x86_64")]
#[test]
fn cpuid_test() {
    let h = System::initialize().unwrap();
    if h.check_capability(Capability::ExtCpuid) == 1 {
        let mut cpuid = h.get_supported_cpuid().unwrap();
        let mut vm = VirtualMachine::create(&h).unwrap();
        let mut vcpu = Vcpu::create(&mut vm).unwrap();
        vcpu.set_cpuid2(&mut cpuid).unwrap();
    }
}

#[cfg(target_arch = "x86_64")]
#[test]
fn sreg_test() {
    let h = System::initialize().unwrap();
    let mut vm = VirtualMachine::create(&h).unwrap();
    let mut vcpu = Vcpu::create(&mut vm).unwrap();
    let mut sregs = vcpu.get_sregs().unwrap();
    sregs.cr0 = 0x1;
    vcpu.set_sregs(&sregs).unwrap();
    assert!(vcpu.get_sregs().unwrap().cr0 == 0x1);
}

#[cfg(target_arch = "x86_64")]
#[test]
fn reg_test() {
    let h = System::initialize().unwrap();
    let mut vm = VirtualMachine::create(&h).unwrap();
    let mut vcpu = Vcpu::create(&mut vm).unwrap();
    let mut regs = vcpu.get_regs().unwrap();
    regs.rax = 0x1;
    vcpu.set_regs(&regs).unwrap();
    assert!(vcpu.get_regs().unwrap().rax == 0x1);
}
