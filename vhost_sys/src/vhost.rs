// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Default)]
pub struct __IncompleteArrayField<T>(::std::marker::PhantomData<T>);
impl<T> __IncompleteArrayField<T> {
    #[inline]
    pub fn new() -> Self {
        __IncompleteArrayField(::std::marker::PhantomData)
    }
    #[inline]
    pub unsafe fn as_ptr(&self) -> *const T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_mut_ptr(&mut self) -> *mut T {
        ::std::mem::transmute(self)
    }
    #[inline]
    pub unsafe fn as_slice(&self, len: usize) -> &[T] {
        ::std::slice::from_raw_parts(self.as_ptr(), len)
    }
    #[inline]
    pub unsafe fn as_mut_slice(&mut self, len: usize) -> &mut [T] {
        ::std::slice::from_raw_parts_mut(self.as_mut_ptr(), len)
    }
}
impl<T> ::std::fmt::Debug for __IncompleteArrayField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__IncompleteArrayField")
    }
}
impl<T> ::std::clone::Clone for __IncompleteArrayField<T> {
    #[inline]
    fn clone(&self) -> Self {
        Self::new()
    }
}
impl<T> ::std::marker::Copy for __IncompleteArrayField<T> {}
pub const __BITS_PER_LONG: ::std::os::raw::c_uint = 64;
pub const BITS_PER_LONG: ::std::os::raw::c_uint = 32;
pub const BITS_PER_LONG_LONG: ::std::os::raw::c_uint = 64;
pub const __FD_SETSIZE: ::std::os::raw::c_uint = 1024;
pub const VHOST_VRING_F_LOG: ::std::os::raw::c_uint = 0;
pub const VHOST_ACCESS_RO: ::std::os::raw::c_uint = 1;
pub const VHOST_ACCESS_WO: ::std::os::raw::c_uint = 2;
pub const VHOST_ACCESS_RW: ::std::os::raw::c_uint = 3;
pub const VHOST_IOTLB_MISS: ::std::os::raw::c_uint = 1;
pub const VHOST_IOTLB_UPDATE: ::std::os::raw::c_uint = 2;
pub const VHOST_IOTLB_INVALIDATE: ::std::os::raw::c_uint = 3;
pub const VHOST_IOTLB_ACCESS_FAIL: ::std::os::raw::c_uint = 4;
pub const VHOST_IOTLB_MSG: ::std::os::raw::c_uint = 1;
pub const VHOST_PAGE_SIZE: ::std::os::raw::c_uint = 4096;
pub const VHOST_VIRTIO: ::std::os::raw::c_uint = 175;
pub const VHOST_VRING_LITTLE_ENDIAN: ::std::os::raw::c_uint = 0;
pub const VHOST_VRING_BIG_ENDIAN: ::std::os::raw::c_uint = 1;
pub const VHOST_F_LOG_ALL: ::std::os::raw::c_uint = 26;
pub const VHOST_NET_F_VIRTIO_NET_HDR: ::std::os::raw::c_uint = 27;
pub const VHOST_SCSI_ABI_VERSION: ::std::os::raw::c_uint = 1;
pub type __s8 = ::std::os::raw::c_schar;
pub type __u8 = ::std::os::raw::c_uchar;
pub type __s16 = ::std::os::raw::c_short;
pub type __u16 = ::std::os::raw::c_ushort;
pub type __s32 = ::std::os::raw::c_int;
pub type __u32 = ::std::os::raw::c_uint;
pub type __s64 = ::std::os::raw::c_longlong;
pub type __u64 = ::std::os::raw::c_ulonglong;
pub const false_: _bindgen_ty_1 = 0;
pub const true_: _bindgen_ty_1 = 1;
pub type _bindgen_ty_1 = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct __kernel_fd_set {
    pub fds_bits: [::std::os::raw::c_ulong; 16usize],
}
#[test]
fn bindgen_test_layout___kernel_fd_set() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fd_set>(),
        128usize,
        concat!("Size of: ", stringify!(__kernel_fd_set))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fd_set>(),
        8usize,
        concat!("Alignment of ", stringify!(__kernel_fd_set))
    );
}
pub type __kernel_sighandler_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: ::std::os::raw::c_int)>;
pub type __kernel_key_t = ::std::os::raw::c_int;
pub type __kernel_mqd_t = ::std::os::raw::c_int;
pub type __kernel_old_uid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_gid_t = ::std::os::raw::c_ushort;
pub type __kernel_old_dev_t = ::std::os::raw::c_ulong;
pub type __kernel_long_t = ::std::os::raw::c_long;
pub type __kernel_ulong_t = ::std::os::raw::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = ::std::os::raw::c_uint;
pub type __kernel_pid_t = ::std::os::raw::c_int;
pub type __kernel_ipc_pid_t = ::std::os::raw::c_int;
pub type __kernel_uid_t = ::std::os::raw::c_uint;
pub type __kernel_gid_t = ::std::os::raw::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = ::std::os::raw::c_int;
pub type __kernel_uid32_t = ::std::os::raw::c_uint;
pub type __kernel_gid32_t = ::std::os::raw::c_uint;
pub type __kernel_size_t = __kernel_ulong_t;
pub type __kernel_ssize_t = __kernel_long_t;
pub type __kernel_ptrdiff_t = __kernel_long_t;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct __kernel_fsid_t {
    pub val: [::std::os::raw::c_int; 2usize],
}
#[test]
fn bindgen_test_layout___kernel_fsid_t() {
    assert_eq!(
        ::std::mem::size_of::<__kernel_fsid_t>(),
        8usize,
        concat!("Size of: ", stringify!(__kernel_fsid_t))
    );
    assert_eq!(
        ::std::mem::align_of::<__kernel_fsid_t>(),
        4usize,
        concat!("Alignment of ", stringify!(__kernel_fsid_t))
    );
}
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = ::std::os::raw::c_longlong;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = ::std::os::raw::c_int;
pub type __kernel_clockid_t = ::std::os::raw::c_int;
pub type __kernel_caddr_t = *mut ::std::os::raw::c_char;
pub type __kernel_uid16_t = ::std::os::raw::c_ushort;
pub type __kernel_gid16_t = ::std::os::raw::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __kernel_dev_t = __u32;
pub type fd_set = __kernel_fd_set;
pub type dev_t = __kernel_dev_t;
pub type ino_t = __kernel_ino_t;
pub type mode_t = __kernel_mode_t;
pub type umode_t = ::std::os::raw::c_ushort;
pub type nlink_t = __u32;
pub type off_t = __kernel_off_t;
pub type pid_t = __kernel_pid_t;
pub type daddr_t = __kernel_daddr_t;
pub type key_t = __kernel_key_t;
pub type suseconds_t = __kernel_suseconds_t;
pub type timer_t = __kernel_timer_t;
pub type clockid_t = __kernel_clockid_t;
pub type mqd_t = __kernel_mqd_t;
pub type bool_ = bool;
pub type uid_t = __kernel_uid32_t;
pub type gid_t = __kernel_gid32_t;
pub type uid16_t = __kernel_uid16_t;
pub type gid16_t = __kernel_gid16_t;
pub type loff_t = __kernel_loff_t;
pub type time_t = __kernel_time_t;
pub type clock_t = __kernel_clock_t;
pub type caddr_t = __kernel_caddr_t;
pub type u_char = ::std::os::raw::c_uchar;
pub type u_short = ::std::os::raw::c_ushort;
pub type u_int = ::std::os::raw::c_uint;
pub type u_long = ::std::os::raw::c_ulong;
pub type unchar = ::std::os::raw::c_uchar;
pub type ushort = ::std::os::raw::c_ushort;
pub type uint = ::std::os::raw::c_uint;
pub type ulong = ::std::os::raw::c_ulong;
pub type u_int8_t = __u8;
pub type u_int16_t = __u16;
pub type u_int32_t = __u32;
pub type u_int64_t = __u64;
pub type sector_t = ::std::os::raw::c_ulong;
pub type blkcnt_t = ::std::os::raw::c_ulong;
pub type dma_addr_t = u32;
pub type gfp_t = ::std::os::raw::c_uint;
pub type fmode_t = ::std::os::raw::c_uint;
pub type phys_addr_t = u32;
pub type resource_size_t = phys_addr_t;
pub type irq_hw_number_t = ::std::os::raw::c_ulong;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct atomic_t {
    pub counter: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_atomic_t() {
    assert_eq!(
        ::std::mem::size_of::<atomic_t>(),
        4usize,
        concat!("Size of: ", stringify!(atomic_t))
    );
    assert_eq!(
        ::std::mem::align_of::<atomic_t>(),
        4usize,
        concat!("Alignment of ", stringify!(atomic_t))
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}
#[test]
fn bindgen_test_layout_list_head() {
    assert_eq!(
        ::std::mem::size_of::<list_head>(),
        16usize,
        concat!("Size of: ", stringify!(list_head))
    );
    assert_eq!(
        ::std::mem::align_of::<list_head>(),
        8usize,
        concat!("Alignment of ", stringify!(list_head))
    );
}
impl Default for list_head {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hlist_head {
    pub first: *mut hlist_node,
}
#[test]
fn bindgen_test_layout_hlist_head() {
    assert_eq!(
        ::std::mem::size_of::<hlist_head>(),
        8usize,
        concat!("Size of: ", stringify!(hlist_head))
    );
    assert_eq!(
        ::std::mem::align_of::<hlist_head>(),
        8usize,
        concat!("Alignment of ", stringify!(hlist_head))
    );
}
impl Default for hlist_head {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct hlist_node {
    pub next: *mut hlist_node,
    pub pprev: *mut *mut hlist_node,
}
#[test]
fn bindgen_test_layout_hlist_node() {
    assert_eq!(
        ::std::mem::size_of::<hlist_node>(),
        16usize,
        concat!("Size of: ", stringify!(hlist_node))
    );
    assert_eq!(
        ::std::mem::align_of::<hlist_node>(),
        8usize,
        concat!("Alignment of ", stringify!(hlist_node))
    );
}
impl Default for hlist_node {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct ustat {
    pub f_tfree: __kernel_daddr_t,
    pub f_tinode: __kernel_ino_t,
    pub f_fname: [::std::os::raw::c_char; 6usize],
    pub f_fpack: [::std::os::raw::c_char; 6usize],
}
#[test]
fn bindgen_test_layout_ustat() {
    assert_eq!(
        ::std::mem::size_of::<ustat>(),
        32usize,
        concat!("Size of: ", stringify!(ustat))
    );
    assert_eq!(
        ::std::mem::align_of::<ustat>(),
        8usize,
        concat!("Alignment of ", stringify!(ustat))
    );
}
/// struct callback_head - callback structure for use with RCU and task_work
/// @next: next update requests in a list
/// @func: actual update function to call after the grace period.
///
/// The struct is aligned to size of pointer. On most architectures it happens
/// naturally due ABI requirements, but some architectures (like CRIS) have
/// weird ABI and we need to ask it explicitly.
///
/// The alignment is required to guarantee that bit 0 of @next will be
/// clear under normal conditions -- as long as we use call_rcu(),
/// call_rcu_bh(), call_rcu_sched(), or call_srcu() to queue callback.
///
/// This guarantee is important for few reasons:
/// - future call_rcu_lazy() will make use of lower bits in the pointer;
/// - the structure shares storage spacer in struct page with @compound_head,
/// which encode PageTail() in bit 0. The guarantee is needed to avoid
/// false-positive PageTail().
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct callback_head {
    pub next: *mut callback_head,
    pub func: ::std::option::Option<unsafe extern "C" fn(head: *mut callback_head)>,
}
#[test]
fn bindgen_test_layout_callback_head() {
    assert_eq!(
        ::std::mem::size_of::<callback_head>(),
        16usize,
        concat!("Size of: ", stringify!(callback_head))
    );
    assert_eq!(
        ::std::mem::align_of::<callback_head>(),
        8usize,
        concat!("Alignment of ", stringify!(callback_head))
    );
}
impl Default for callback_head {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
pub type rcu_callback_t = ::std::option::Option<unsafe extern "C" fn(head: *mut callback_head)>;
pub type call_rcu_func_t =
    ::std::option::Option<unsafe extern "C" fn(head: *mut callback_head, func: rcu_callback_t)>;
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vhost_vring_state {
    pub index: ::std::os::raw::c_uint,
    pub num: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_vhost_vring_state() {
    assert_eq!(
        ::std::mem::size_of::<vhost_vring_state>(),
        8usize,
        concat!("Size of: ", stringify!(vhost_vring_state))
    );
    assert_eq!(
        ::std::mem::align_of::<vhost_vring_state>(),
        4usize,
        concat!("Alignment of ", stringify!(vhost_vring_state))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vhost_vring_file {
    pub index: ::std::os::raw::c_uint,
    pub fd: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout_vhost_vring_file() {
    assert_eq!(
        ::std::mem::size_of::<vhost_vring_file>(),
        8usize,
        concat!("Size of: ", stringify!(vhost_vring_file))
    );
    assert_eq!(
        ::std::mem::align_of::<vhost_vring_file>(),
        4usize,
        concat!("Alignment of ", stringify!(vhost_vring_file))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vhost_vring_addr {
    pub index: ::std::os::raw::c_uint,
    pub flags: ::std::os::raw::c_uint,
    pub desc_user_addr: __u64,
    pub used_user_addr: __u64,
    pub avail_user_addr: __u64,
    pub log_guest_addr: __u64,
}
#[test]
fn bindgen_test_layout_vhost_vring_addr() {
    assert_eq!(
        ::std::mem::size_of::<vhost_vring_addr>(),
        40usize,
        concat!("Size of: ", stringify!(vhost_vring_addr))
    );
    assert_eq!(
        ::std::mem::align_of::<vhost_vring_addr>(),
        8usize,
        concat!("Alignment of ", stringify!(vhost_vring_addr))
    );
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vhost_iotlb_msg {
    pub iova: __u64,
    pub size: __u64,
    pub uaddr: __u64,
    pub perm: __u8,
    pub type_: __u8,
}
#[test]
fn bindgen_test_layout_vhost_iotlb_msg() {
    assert_eq!(
        ::std::mem::size_of::<vhost_iotlb_msg>(),
        32usize,
        concat!("Size of: ", stringify!(vhost_iotlb_msg))
    );
    assert_eq!(
        ::std::mem::align_of::<vhost_iotlb_msg>(),
        8usize,
        concat!("Alignment of ", stringify!(vhost_iotlb_msg))
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_msg {
    pub type_: ::std::os::raw::c_int,
    pub __bindgen_anon_1: vhost_msg__bindgen_ty_1,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union vhost_msg__bindgen_ty_1 {
    pub iotlb: vhost_iotlb_msg,
    pub padding: [__u8; 64usize],
    _bindgen_union_align: [u64; 8usize],
}
#[test]
fn bindgen_test_layout_vhost_msg__bindgen_ty_1() {
    assert_eq!(
        ::std::mem::size_of::<vhost_msg__bindgen_ty_1>(),
        64usize,
        concat!("Size of: ", stringify!(vhost_msg__bindgen_ty_1))
    );
    assert_eq!(
        ::std::mem::align_of::<vhost_msg__bindgen_ty_1>(),
        8usize,
        concat!("Alignment of ", stringify!(vhost_msg__bindgen_ty_1))
    );
}
impl Default for vhost_msg__bindgen_ty_1 {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[test]
fn bindgen_test_layout_vhost_msg() {
    assert_eq!(
        ::std::mem::size_of::<vhost_msg>(),
        72usize,
        concat!("Size of: ", stringify!(vhost_msg))
    );
    assert_eq!(
        ::std::mem::align_of::<vhost_msg>(),
        8usize,
        concat!("Alignment of ", stringify!(vhost_msg))
    );
}
impl Default for vhost_msg {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Debug, Default, Copy, Clone)]
pub struct vhost_memory_region {
    pub guest_phys_addr: __u64,
    pub memory_size: __u64,
    pub userspace_addr: __u64,
    pub flags_padding: __u64,
}
#[test]
fn bindgen_test_layout_vhost_memory_region() {
    assert_eq!(
        ::std::mem::size_of::<vhost_memory_region>(),
        32usize,
        concat!("Size of: ", stringify!(vhost_memory_region))
    );
    assert_eq!(
        ::std::mem::align_of::<vhost_memory_region>(),
        8usize,
        concat!("Alignment of ", stringify!(vhost_memory_region))
    );
}
#[repr(C)]
#[derive(Debug, Default)]
pub struct vhost_memory {
    pub nregions: __u32,
    pub padding: __u32,
    pub regions: __IncompleteArrayField<vhost_memory_region>,
    __force_alignment: [u64; 0],
}
#[test]
fn bindgen_test_layout_vhost_memory() {
    assert_eq!(
        ::std::mem::size_of::<vhost_memory>(),
        8usize,
        concat!("Size of: ", stringify!(vhost_memory))
    );
    assert_eq!(
        ::std::mem::align_of::<vhost_memory>(),
        8usize,
        concat!("Alignment of ", stringify!(vhost_memory))
    );
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vhost_scsi_target {
    pub abi_version: ::std::os::raw::c_int,
    pub vhost_wwpn: [::std::os::raw::c_char; 224usize],
    pub vhost_tpgt: ::std::os::raw::c_ushort,
    pub reserved: ::std::os::raw::c_ushort,
}
#[test]
fn bindgen_test_layout_vhost_scsi_target() {
    assert_eq!(
        ::std::mem::size_of::<vhost_scsi_target>(),
        232usize,
        concat!("Size of: ", stringify!(vhost_scsi_target))
    );
    assert_eq!(
        ::std::mem::align_of::<vhost_scsi_target>(),
        4usize,
        concat!("Alignment of ", stringify!(vhost_scsi_target))
    );
}
impl Default for vhost_scsi_target {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
