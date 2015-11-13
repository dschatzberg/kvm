use libc::{c_void, calloc, free};
use std::{mem, slice};
use std::ops::{Deref, DerefMut};

#[repr(C)]
#[derive(Copy)]
pub struct PicState {
    pub last_irr: u8,
    pub irr: u8,
    pub imr: u8,
    pub isr: u8,
    pub priority_add: u8,
    pub irq_base: u8,
    pub read_reg_select: u8,
    pub poll: u8,
    pub special_mask: u8,
    pub init_state: u8,
    pub auto_eoi: u8,
    pub rotate_on_auto_eoi: u8,
    pub special_fully_nested_mode: u8,
    pub init4: u8,
    pub elcr: u8,
    pub elcr_mask: u8,
}
impl ::std::clone::Clone for PicState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for PicState {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct IoapicState {
    pub base_address: u64,
    pub ioregsel: u32,
    pub id: u32,
    pub irr: u32,
    pub pad: u32,
    pub redirtbl: [Union_Unnamed3; 24usize],
}
impl ::std::clone::Clone for IoapicState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for IoapicState {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Union_Unnamed3 {
    pub _bindgen_data_: [u64; 1usize],
}
impl Union_Unnamed3 {
    pub unsafe fn bits(&mut self) -> *mut u64 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
    pub unsafe fn fields(&mut self) -> *mut Struct_Unnamed4 {
        let raw: *mut u8 = ::std::mem::transmute(&self._bindgen_data_);
        ::std::mem::transmute(raw.offset(0))
    }
}
impl ::std::clone::Clone for Union_Unnamed3 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Union_Unnamed3 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed4 {
    pub vector: u8,
    pub _bindgen_bitfield_1_: u8,
    pub _bindgen_bitfield_2_: u8,
    pub _bindgen_bitfield_3_: u8,
    pub _bindgen_bitfield_4_: u8,
    pub _bindgen_bitfield_5_: u8,
    pub _bindgen_bitfield_6_: u8,
    pub _bindgen_bitfield_7_: u8,
    pub _bindgen_bitfield_8_: u8,
    pub reserved: [u8; 4usize],
    pub dest_id: u8,
}
impl ::std::clone::Clone for Struct_Unnamed4 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_Unnamed4 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Regs {
    pub rax: u64,
    pub rbx: u64,
    pub rcx: u64,
    pub rdx: u64,
    pub rsi: u64,
    pub rdi: u64,
    pub rsp: u64,
    pub rbp: u64,
    pub r8: u64,
    pub r9: u64,
    pub r10: u64,
    pub r11: u64,
    pub r12: u64,
    pub r13: u64,
    pub r14: u64,
    pub r15: u64,
    pub rip: u64,
    pub rflags: u64,
}
impl ::std::clone::Clone for Regs {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Regs {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct LapicState {
    pub regs: [::libc::c_char; 1024usize],
}
impl ::std::clone::Clone for LapicState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for LapicState {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Segment {
    pub base: u64,
    pub limit: u32,
    pub selector: u16,
    pub _type: u8,
    pub present: u8,
    pub dpl: u8,
    pub db: u8,
    pub s: u8,
    pub l: u8,
    pub g: u8,
    pub avl: u8,
    pub unusable: u8,
    pub padding: u8,
}
impl ::std::clone::Clone for Segment {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Segment {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Dtable {
    pub base: u64,
    pub limit: u16,
    pub padding: [u16; 3usize],
}
impl ::std::clone::Clone for Dtable {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Dtable {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Sregs {
    pub cs: Segment,
    pub ds: Segment,
    pub es: Segment,
    pub fs: Segment,
    pub gs: Segment,
    pub ss: Segment,
    pub tr: Segment,
    pub ldt: Segment,
    pub gdt: Dtable,
    pub idt: Dtable,
    pub cr0: u64,
    pub cr2: u64,
    pub cr3: u64,
    pub cr4: u64,
    pub cr8: u64,
    pub efer: u64,
    pub apic_base: u64,
    pub interrupt_bitmap: [u64; 4usize],
}
impl ::std::clone::Clone for Sregs {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Sregs {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Fpu {
    pub fpr: [[u8; 16usize]; 8usize],
    pub fcw: u16,
    pub fsw: u16,
    pub ftwx: u8,
    pub pad1: u8,
    pub last_opcode: u16,
    pub last_ip: u64,
    pub last_dp: u64,
    pub xmm: [[u8; 16usize]; 16usize],
    pub mxcsr: u32,
    pub pad2: u32,
}
impl ::std::clone::Clone for Fpu {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Fpu {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct MsrEntry {
    pub index: u32,
    pub reserved: u32,
    pub data: u64,
}
impl ::std::clone::Clone for MsrEntry {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for MsrEntry {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Msrs {
    pub nmsrs: u32,
    pub pad: u32,
    pub entries: [MsrEntry; 0usize],
}
impl ::std::clone::Clone for Msrs {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Msrs {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct MsrList {
    pub nmsrs: u32,
    pub indices: [u32; 0usize],
}
impl ::std::clone::Clone for MsrList {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for MsrList {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct CpuidEntry {
    pub function: u32,
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub padding: u32,
}
impl ::std::clone::Clone for CpuidEntry {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for CpuidEntry {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Cpuid {
    pub nent: u32,
    pub padding: u32,
    pub entries: [CpuidEntry; 0usize],
}
impl ::std::clone::Clone for Cpuid {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Cpuid {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct CpuidEntry2 {
    pub function: u32,
    pub index: u32,
    pub flags: u32,
    pub eax: u32,
    pub ebx: u32,
    pub ecx: u32,
    pub edx: u32,
    pub padding: [u32; 3usize],
}
impl ::std::clone::Clone for CpuidEntry2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for CpuidEntry2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
pub struct Cpuid2 {
    pub nent: u32,
    padding: u32,
    entries: [CpuidEntry2],
}

pub struct CpuidHandle {
    cpuid: *mut Cpuid2,
}

impl CpuidHandle {
    pub fn new(nent: u32) -> CpuidHandle {
        unsafe {
            let ptr = calloc(1,
                             8 +
                             nent as usize * mem::size_of::<CpuidEntry2>());
            assert!(!ptr.is_null());
            let ptr: *mut Cpuid2 = mem::transmute((ptr, nent as usize));
            (*ptr).nent = nent;
            CpuidHandle { cpuid: ptr }
        }
    }
}

impl Deref for CpuidHandle {
    type Target = Cpuid2;

    fn deref<'a>(&'a self) -> &'a Cpuid2 {
        unsafe { &*self.cpuid }
    }
}

impl DerefMut for CpuidHandle {
    fn deref_mut<'a>(&'a mut self) -> &'a mut Cpuid2 {
        unsafe { &mut *self.cpuid }
    }
}

impl Drop for CpuidHandle {
    fn drop(&mut self) {
        unsafe {
            let (ptr, _): (*mut c_void, usize) = mem::transmute(self.cpuid);
            free(ptr);
        }
    }
}

impl Cpuid2 {
    pub fn entries(&self) -> &[CpuidEntry2] {
        unsafe {
            let first = self.entries.get_unchecked(0);
            slice::from_raw_parts(first as *const _, self.nent as usize)
        }
    }
    pub fn entries_mut(&mut self) -> &mut [CpuidEntry2] {
        unsafe {
            let first = self.entries.get_unchecked_mut(0);
            slice::from_raw_parts_mut(first as *mut _, self.nent as usize)
        }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct PitChannelState {
    pub count: u32,
    pub latched_count: u16,
    pub count_latched: u8,
    pub status_latched: u8,
    pub status: u8,
    pub read_state: u8,
    pub write_state: u8,
    pub write_latch: u8,
    pub rw_mode: u8,
    pub mode: u8,
    pub bcd: u8,
    pub gate: u8,
    pub count_load_time: i64,
}
impl ::std::clone::Clone for PitChannelState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for PitChannelState {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct DebugExitArch {
    pub exception: u32,
    pub pad: u32,
    pub pc: u64,
    pub dr6: u64,
    pub dr7: u64,
}
impl ::std::clone::Clone for DebugExitArch {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for DebugExitArch {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct GuestDebugArch {
    pub debugreg: [u64; 8usize],
}
impl ::std::clone::Clone for GuestDebugArch {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for GuestDebugArch {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct PitState {
    pub channels: [PitChannelState; 3usize],
}
impl ::std::clone::Clone for PitState {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for PitState {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct PitState2 {
    pub channels: [PitChannelState; 3usize],
    pub flags: u32,
    pub reserved: [u32; 9usize],
}
impl ::std::clone::Clone for PitState2 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for PitState2 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct ReinjectControl {
    pub pit_reinject: u8,
    pub reserved: [u8; 31usize],
}
impl ::std::clone::Clone for ReinjectControl {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for ReinjectControl {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct VcpuEvents {
    pub exception: StructUnnamed5,
    pub interrupt: StructUnnamed6,
    pub nmi: StructUnnamed7,
    pub sipi_vector: u32,
    pub flags: u32,
    pub reserved: [u32; 10usize],
}
impl ::std::clone::Clone for VcpuEvents {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for VcpuEvents {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct StructUnnamed5 {
    pub injected: u8,
    pub nr: u8,
    pub has_error_code: u8,
    pub pad: u8,
    pub error_code: u32,
}
impl ::std::clone::Clone for StructUnnamed5 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for StructUnnamed5 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct StructUnnamed6 {
    pub injected: u8,
    pub nr: u8,
    pub soft: u8,
    pub shadow: u8,
}
impl ::std::clone::Clone for StructUnnamed6 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for StructUnnamed6 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct StructUnnamed7 {
    pub injected: u8,
    pub pending: u8,
    pub masked: u8,
    pub pad: u8,
}
impl ::std::clone::Clone for StructUnnamed7 {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for StructUnnamed7 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Debugregs {
    pub db: [u64; 4usize],
    pub dr6: u64,
    pub dr7: u64,
    pub flags: u64,
    pub reserved: [u64; 9usize],
}
impl ::std::clone::Clone for Debugregs {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Debugregs {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Xsave {
    pub region: [u32; 1024usize],
}
impl ::std::clone::Clone for Xsave {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Xsave {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Xcr {
    pub xcr: u32,
    pub reserved: u32,
    pub value: u64,
}
impl ::std::clone::Clone for Xcr {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Xcr {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Xcrs {
    pub nr_xcrs: u32,
    pub flags: u32,
    pub xcrs: [Xcr; 16usize],
    pub padding: [u64; 16usize],
}
impl ::std::clone::Clone for Xcrs {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Xcrs {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct SyncRegs;
impl ::std::clone::Clone for SyncRegs {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for SyncRegs {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
