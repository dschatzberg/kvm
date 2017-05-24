// Copyright 2015 Dan Schatzberg.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Rust API to Kernel-based Virtual Machine (KVM)
#![deny(missing_docs,
        missing_debug_implementations, missing_copy_implementations,
        trivial_casts, trivial_numeric_casts,
        unstable_features,
        unused_import_braces, unused_qualifications)]

#![cfg_attr(feature = "dev", allow(unstable_features))]
#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

extern crate errno;
extern crate libc;
#[macro_use]
extern crate log;
extern crate memmap;

#[cfg(target_arch = "x86_64")]
#[allow(missing_docs, missing_debug_implementations)]
mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use self::x86_64::*;

use errno::{Errno, errno};
use libc::{E2BIG, ENOMEM, c_int};
use std::fmt;
use std::fs::{File, OpenOptions};
use std::io::{self, Error, ErrorKind};
use std::mem;
use std::ops::DerefMut;
use std::os::unix::io::{AsRawFd, FromRawFd};

use memmap::{Mmap, Protection};

const API_VERSION: i32 = 12;

extern {
    fn kvm_get_api_version(fd: c_int) -> c_int;
    fn kvm_create_vm(fd: c_int, flags: c_int) -> c_int;
    fn kvm_check_extension(fd: c_int, extension: c_int) -> c_int;
    fn kvm_get_vcpu_mmap_size(fd: c_int) -> c_int;
    fn kvm_get_supported_cpuid(fd: c_int, cpuid: *mut Cpuid2) -> c_int;
    fn kvm_create_vcpu(fd: c_int, vcpu_id: c_int) -> c_int;
    fn kvm_set_user_memory_region(fd: c_int,
                                  region: *const UserspaceMemoryRegion)
                                  -> c_int;
    fn kvm_run(fd: c_int) -> c_int;
    fn kvm_get_regs(fd: c_int, regs: *mut Regs) -> c_int;
    fn kvm_set_regs(fd: c_int, regs: *const Regs) -> c_int;
    fn kvm_get_sregs(fd: c_int, sregs: *mut Sregs) -> c_int;
    fn kvm_set_sregs(fd: c_int, sregs: *const Sregs) -> c_int;
    fn kvm_set_cpuid2(fd: c_int, cpuid: *const Cpuid2) -> c_int;
}

/// Handle to the KVM system.
///
/// This is used to create virtual machines and query the system for
/// its capabilities.
#[derive(Debug)]
pub struct System {
    fd: File,
}

/// A Virtual Machine.
///
/// This allows the creation of `Vcpu`s and establishing memory mappings
#[derive(Debug)]
pub struct VirtualMachine<'a> {
    fd: File,
    sys: &'a System,
    mem_slots: Vec<&'a mut [u8]>,
    num_vcpus: u32,
    check_extension: bool,
}

/// Result type used by this crate
pub type Result<T> = io::Result<T>;

/// KVM system capabilities
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(i32)]
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

/// KVM `run` exit reasons
#[allow(missing_docs)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u32)]
pub enum Exit {
    Unknown,
    Exception,
    Io,
    Hypercall,
    Debug,
    Hlt,
    Mmio,
    IrqWindowOpen,
    Shutdown,
    FailEntry,
    Intr,
    SetTpr,
    TprAccess,
    S390Sieic,
    S390Reset,
    Dcr,
    Nmi,
    InternalError,
    Osi,
    PaprHcall,
    S390Ucontrol,
    Watchdog,
    S390Tsch,
    Epr,
    SystemEvent,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
struct UserspaceMemoryRegion {
    pub slot: u32,
    pub flags: u32,
    pub guest_phys_addr: u64,
    pub memory_size: u64,
    pub userspace_addr: u64,
}

/// A Virtual CPU.
pub struct Vcpu<'a> {
    fd: File,
    vm: &'a VirtualMachine<'a>,
    mmap: Mmap,
}

impl<'a> fmt::Debug for Vcpu<'a> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Vcpu")
           .field("fd", &self.fd)
           .field("vm", &self.vm)
           .finish()
    }
}


/// Information about the reason `run` returned
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy)]
pub struct Run {
    request_interrupt_window: u8,
    padding1: [u8; 7usize],
    pub exit_reason: Exit,
    pub ready_for_interrupt_injection: u8,
    pub if_flag: u8,
    pub flags: u16,
    pub cr8: u64,
    pub apic_base: u64,
    _bindgen_data_1_: [u64; 32usize],
    pub kvm_valid_regs: u64,
    pub kvm_dirty_regs: u64,
    pub s: Union_Unnamed26,
}

#[allow(missing_docs)]
impl Run {
    pub fn hw(&self) -> *const Struct_Unnamed9 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn hw_mut(&mut self) -> *mut Struct_Unnamed9 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn fail_entry(&self) -> *const Struct_Unnamed10 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn fail_entry_mut(&mut self) -> *mut Struct_Unnamed10 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn ex(&self) -> *const Struct_Unnamed11 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn ex_mut(&mut self) -> *mut Struct_Unnamed11 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn io(&self) -> *const ExitIo {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn io_mut(&mut self) -> *mut ExitIo {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn debug(&self) -> *const Struct_Unnamed13 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn debug_mut(&mut self) -> *mut Struct_Unnamed13 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn mmio(&self) -> *const Struct_Unnamed14 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn mmio_mut(&mut self) -> *mut Struct_Unnamed14 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn hypercall(&self) -> *const Struct_Unnamed15 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn hypercall_mut(&mut self) -> *mut Struct_Unnamed15 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn tpr_access(&self) -> *const Struct_Unnamed16 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn tpr_access_mut(&mut self) -> *mut Struct_Unnamed16 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn s390_sieic(&self) -> *const Struct_Unnamed17 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn s390_sieic_mut(&mut self) -> *mut Struct_Unnamed17 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn s390_reset_flags(&self) -> *const u64 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn s390_reset_flags_mut(&mut self) -> *mut u64 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn s390_ucontrol(&self) -> *const Struct_Unnamed18 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn s390_ucontrol_mut(&mut self) -> *mut Struct_Unnamed18 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn dcr(&self) -> *const Struct_Unnamed19 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn dcr_mut(&mut self) -> *mut Struct_Unnamed19 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn internal(&self) -> *const Struct_Unnamed20 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn internal_mut(&mut self) -> *mut Struct_Unnamed20 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn osi(&self) -> *const Struct_Unnamed21 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn osi_mut(&mut self) -> *mut Struct_Unnamed21 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn papr_hcall(&self) -> *const Struct_Unnamed22 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn papr_hcall_mut(&mut self) -> *mut Struct_Unnamed22 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn s390_tsch(&self) -> *const Struct_Unnamed23 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn s390_tsch_mut(&mut self) -> *mut Struct_Unnamed23 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn epr(&self) -> *const Struct_Unnamed24 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn epr_mut(&mut self) -> *mut Struct_Unnamed24 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn system_event(&self) -> *const Struct_Unnamed25 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn system_event_mut(&mut self) -> *mut Struct_Unnamed25 {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_1_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
}
impl ::std::clone::Clone for Run {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Run {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl fmt::Debug for Run {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let mut s = fmt.debug_struct("Run");
        s.field("request_interrupt_window", &self.request_interrupt_window)
         .field("exit_reason", &self.exit_reason)
         .field("ready_for_interrupt_injection",
                &self.ready_for_interrupt_injection)
         .field("if_flag", &self.if_flag)
         .field("flags", &self.flags)
         .field("cr8", &self.cr8);
        unsafe {
            match self.exit_reason {
                Exit::Unknown => s.field("hw", &*self.hw()),
                Exit::FailEntry => s.field("fail_entry", &*self.fail_entry()),
                Exit::Exception => s.field("ex", &*self.ex()),
                Exit::Io => s.field("io", &*self.io()),
                Exit::Debug => s.field("debug", &*self.debug()),
                Exit::Mmio => s.field("mmio", &*self.mmio()),
                Exit::Hypercall => s.field("hypercall", &*self.hypercall()),
                Exit::TprAccess => s.field("tpr_access", &*self.tpr_access()),
                Exit::S390Sieic => s.field("s390_sieic", &*self.s390_sieic()),
                Exit::S390Reset =>
                    s.field("s390_reset_flags", &*self.s390_reset_flags()),
                Exit::S390Ucontrol =>
                    s.field("s390_ucontrol", &*self.s390_ucontrol()),
                Exit::Dcr => s.field("dcr", &*self.dcr()),
                Exit::Osi => s.field("osi", &*self.osi()),
                Exit::PaprHcall => s.field("papr_hcall", &*self.papr_hcall()),
                Exit::S390Tsch => s.field("s390_tsch", &*self.s390_tsch()),
                Exit::Epr => s.field("epr", &*self.epr()),
                Exit::SystemEvent =>
                    s.field("system_event", &*self.system_event()),
                _ => &mut s,
            }
        }
        .finish()
    }
}
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_Unnamed9 {
    pub hardware_exit_reason: u64,
}
impl ::std::clone::Clone for Struct_Unnamed9 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed9 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_Unnamed10 {
    pub hardware_entry_failure_reason: u64,
}
impl ::std::clone::Clone for Struct_Unnamed10 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed10 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_Unnamed11 {
    pub exception: u32,
    pub error_code: u32,
}
impl ::std::clone::Clone for Struct_Unnamed11 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed11 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[allow(missing_docs)]
#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum IoDirection {
    In,
    Out,
}

#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct ExitIo {
    pub direction: IoDirection,
    pub size: u8,
    pub port: u16,
    pub count: u32,
    pub data_offset: u64,
}
impl ::std::clone::Clone for ExitIo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for ExitIo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_Unnamed13 {
    pub arch: DebugExitArch,
}
impl ::std::clone::Clone for Struct_Unnamed13 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed13 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_Unnamed14 {
    pub phys_addr: u64,
    pub data: [u8; 8usize],
    pub len: u32,
    pub is_write: u8,
}
impl ::std::clone::Clone for Struct_Unnamed14 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed14 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_Unnamed15 {
    pub nr: u64,
    pub args: [u64; 6usize],
    pub ret: u64,
    pub longmode: u32,
    pub pad: u32,
}
impl ::std::clone::Clone for Struct_Unnamed15 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed15 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_Unnamed16 {
    pub rip: u64,
    pub is_write: u32,
    pub pad: u32,
}
impl ::std::clone::Clone for Struct_Unnamed16 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed16 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_Unnamed17 {
    pub icptcode: u8,
    pub ipa: u16,
    pub ipb: u32,
}
impl ::std::clone::Clone for Struct_Unnamed17 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed17 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_Unnamed18 {
    pub trans_exc_code: u64,
    pub pgm_code: u32,
}
impl ::std::clone::Clone for Struct_Unnamed18 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed18 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_Unnamed19 {
    pub dcrn: u32,
    pub data: u32,
    pub is_write: u8,
}
impl ::std::clone::Clone for Struct_Unnamed19 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed19 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_Unnamed20 {
    pub suberror: u32,
    pub ndata: u32,
    pub data: [u64; 16usize],
}
impl ::std::clone::Clone for Struct_Unnamed20 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed20 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_Unnamed21 {
    pub gprs: [u64; 32usize],
}
impl ::std::clone::Clone for Struct_Unnamed21 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed21 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_Unnamed22 {
    pub nr: u64,
    pub ret: u64,
    pub args: [u64; 9usize],
}
impl ::std::clone::Clone for Struct_Unnamed22 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed22 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_Unnamed23 {
    pub subchannel_id: u16,
    pub subchannel_nr: u16,
    pub io_int_parm: u32,
    pub io_int_word: u32,
    pub ipb: u32,
    pub dequeued: u8,
}
impl ::std::clone::Clone for Struct_Unnamed23 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed23 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_Unnamed24 {
    pub epr: u32,
}
impl ::std::clone::Clone for Struct_Unnamed24 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed24 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(missing_docs)]
#[repr(C)]
#[derive(Copy, Debug)]
pub struct Struct_Unnamed25 {
    pub _type: u32,
    pub flags: u64,
}
impl ::std::clone::Clone for Struct_Unnamed25 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed25 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[allow(missing_docs, missing_debug_implementations)]
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed26 {
    pub _bindgen_data_: [u8; 1024usize],
}
#[allow(missing_docs)]
impl Union_Unnamed26 {
    pub fn regs(&self) -> *const SyncRegs {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
    pub fn regs_mut(&mut self) -> *mut SyncRegs {
        unsafe {
            let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
            ::std::mem::transmute(raw.offset(0))
        }
    }
}
impl ::std::clone::Clone for Union_Unnamed26 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Union_Unnamed26 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

impl System {
    /// Initialize the KVM system
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

    /// Check for the existence of a capability.
    ///
    /// Where possible use the associated function on a `VirtualMachine` rather
    /// than the `System` since `VirtualMachine`s may have different
    /// capabilities
    pub fn check_capability(&self, cap: Capability) -> i32 {
        unsafe { kvm_check_extension(self.fd.as_raw_fd(), cap as c_int) }
    }

    /// Recommended maximum number of `Vcpu`s
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

    /// Maximum number of `Vcpu`s
    pub fn max_vcpus(&self) -> u32 {
        let r = self.check_capability(Capability::MaxVcpus);
        if r != 0 {
            r as u32
        } else {
            self.recommended_vcpus()
        }
    }

    fn get_vcpu_mmap_size(&self) -> usize {
        let ret = unsafe { kvm_get_vcpu_mmap_size(self.fd.as_raw_fd()) };
        assert!(ret > 0 && ret as usize >= mem::size_of::<Run>());
        ret as usize
    }
}

#[cfg(target_arch = "x86_64")]
const CPUID_ENTRIES: u32 = 64;

#[cfg(target_arch = "x86_64")]
impl System {
    /// Get CPUID features supported by this host
    pub fn get_supported_cpuid(&self) -> Result<CpuidHandle> {
        let mut nent = CPUID_ENTRIES;
        loop {
            let mut c = CpuidHandle::new(nent);
            let err = unsafe {
                kvm_get_supported_cpuid(self.fd.as_raw_fd(), c.deref_mut())
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

/// Instruct KVM to keep track of writes to memory within the slot
pub const MEM_LOG_DIRTY_PAGES: u32 = 1;
/// If `Capability::ReadonlyMem`, make this mapping read-only
pub const MEM_READONLY: u32 = 1 << 1;

impl<'a> VirtualMachine<'a> {
    /// Create a `VirtualMachine`
    pub fn create(s: &'a System) -> Result<Self> {
        let f = unsafe { kvm_create_vm(s.fd.as_raw_fd(), 0) };
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

    /// Check for a capability on this `VirtualMachine`
    pub fn check_capability(&mut self, cap: Capability) -> i32 {
        if self.check_extension {
            unsafe { kvm_check_extension(self.fd.as_raw_fd(), cap as c_int) }
        } else {
            self.sys.check_capability(cap)
        }
    }

    /// Establish a guest memory mapping.
    ///
    /// The slice specified by `user_addr` is mapped at `phys_addr`. Flags is
    /// the bitwise or of `MEM_LOG_DIRTY_PAGES` and/or `MEM_READONLY`.
    pub fn set_user_memory_region(&mut self,
                                  phys_addr: u64,
                                  user_addr: &'a mut [u8],
                                  flags: u32)
                                  -> Result<()> {
        let slot = self.mem_slots.len();
        let region = UserspaceMemoryRegion {
            slot: slot as u32,
            flags: flags,
            guest_phys_addr: phys_addr,
            memory_size: user_addr.len() as u64,
            userspace_addr: user_addr.as_mut_ptr() as u64,
        };
        let ret = unsafe {
            kvm_set_user_memory_region(self.fd.as_raw_fd(), &region)
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
    /// Create a `Vcpu` on the specified `VirtualMachine`
    pub fn create(vm: &'a mut VirtualMachine<'a>) -> Result<Self> {
        if vm.num_vcpus >= vm.sys.max_vcpus() {
            return Err(Error::new(ErrorKind::AlreadyExists,
                                  "Would exceed max_vcpus"));
        } else if vm.num_vcpus >= vm.sys.recommended_vcpus() {
            warn!("Exceeding recommended_vcpus");
        }
        let fd = unsafe {
            File::from_raw_fd(kvm_create_vcpu(vm.fd.as_raw_fd(),
                                              vm.num_vcpus as c_int))
        };
        vm.num_vcpus += 1;
        let mmap_size = vm.sys.get_vcpu_mmap_size();
        let m = try!(Mmap::open_with_offset(&fd,
                                            Protection::ReadWrite,
                                            0,
                                            mmap_size));
        Ok(Vcpu {
            fd: fd,
            vm: vm,
            mmap: m,
        })
    }

    /// Run the `Vcpu`
    pub unsafe fn run(&mut self) -> Result<Run> {
        let ret = kvm_run(self.fd.as_raw_fd());
        if ret == 0 {
            Ok(*(self.mmap.mut_ptr() as *mut Run))
        } else {
            Err(Error::last_os_error())
        }
    }

    /// Get registers
    pub fn get_regs(&self) -> Result<Regs> {
        let mut regs = Regs::default();
        let ret = unsafe { kvm_get_regs(self.fd.as_raw_fd(), &mut regs) };
        if ret == 0 {
            Ok(regs)
        } else {
            Err(Error::last_os_error())
        }
    }

    /// Set registers
    pub fn set_regs(&mut self, regs: &Regs) -> Result<()> {
        let ret = unsafe { kvm_set_regs(self.fd.as_raw_fd(), regs) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
}

#[cfg(target_arch = "x86_64")]
impl<'a> Vcpu<'a> {
    /// Set the response to the CPUID instruction
    pub fn set_cpuid2(&mut self, cpuid: &mut Cpuid2) -> Result<()> {
        let ptr: *mut Cpuid2 = cpuid;
        let ret = unsafe { kvm_set_cpuid2(self.fd.as_raw_fd(), ptr) };
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::last_os_error())
        }
    }
    /// Get special registers
    pub fn get_sregs(&self) -> Result<Sregs> {
        let mut sregs = Sregs::default();
        let ret = unsafe { kvm_get_sregs(self.fd.as_raw_fd(), &mut sregs) };
        if ret == 0 {
            Ok(sregs)
        } else {
            Err(Error::last_os_error())
        }
    }
    /// Set special registers
    pub fn set_sregs(&mut self, sregs: &Sregs) -> Result<()> {
        let ret = unsafe { kvm_set_sregs(self.fd.as_raw_fd(), sregs) };
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
