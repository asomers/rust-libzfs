/* automatically generated by rust-bindgen */

pub type uchar_t = ::std::os::raw::c_uchar;
pub type uint_t = ::std::os::raw::c_uint;
extern "C" {
    pub fn libzfs_core_init() -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn libzfs_core_fini();
}
pub mod lzc_dataset_type {
    pub type Type = ::std::os::raw::c_uint;
    pub const LZC_DATSET_TYPE_ZFS: Type = 2;
    pub const LZC_DATSET_TYPE_ZVOL: Type = 3;
}
extern "C" {
    pub fn lzc_snapshot(arg1: *mut nvlist_t, arg2: *mut nvlist_t,
                        arg3: *mut *mut nvlist_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_create(arg1: *const ::std::os::raw::c_char,
                      arg2: lzc_dataset_type::Type, arg3: *mut nvlist_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_clone(arg1: *const ::std::os::raw::c_char,
                     arg2: *const ::std::os::raw::c_char, arg3: *mut nvlist_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_promote(arg1: *const ::std::os::raw::c_char,
                       arg2: *mut ::std::os::raw::c_char,
                       arg3: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_destroy_snaps(arg1: *mut nvlist_t, arg2: boolean_t,
                             arg3: *mut *mut nvlist_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_bookmark(arg1: *mut nvlist_t, arg2: *mut *mut nvlist_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_get_bookmarks(arg1: *const ::std::os::raw::c_char,
                             arg2: *mut nvlist_t, arg3: *mut *mut nvlist_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_destroy_bookmarks(arg1: *mut nvlist_t,
                                 arg2: *mut *mut nvlist_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_snaprange_space(arg1: *const ::std::os::raw::c_char,
                               arg2: *const ::std::os::raw::c_char,
                               arg3: *mut u64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_hold(arg1: *mut nvlist_t, arg2: ::std::os::raw::c_int,
                    arg3: *mut *mut nvlist_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_release(arg1: *mut nvlist_t, arg2: *mut *mut nvlist_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_get_holds(arg1: *const ::std::os::raw::c_char,
                         arg2: *mut *mut nvlist_t) -> ::std::os::raw::c_int;
}
pub mod lzc_send_flags {
    pub type Type = ::std::os::raw::c_uint;
    pub const LZC_SEND_FLAG_EMBED_DATA: Type = 1;
    pub const LZC_SEND_FLAG_LARGE_BLOCK: Type = 2;
    pub const LZC_SEND_FLAG_COMPRESS: Type = 4;
}
extern "C" {
    pub fn lzc_send(arg1: *const ::std::os::raw::c_char,
                    arg2: *const ::std::os::raw::c_char,
                    arg3: ::std::os::raw::c_int, arg4: lzc_send_flags::Type)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_send_resume(arg1: *const ::std::os::raw::c_char,
                           arg2: *const ::std::os::raw::c_char,
                           arg3: ::std::os::raw::c_int,
                           arg4: lzc_send_flags::Type, arg5: u64, arg6: u64)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_send_space(arg1: *const ::std::os::raw::c_char,
                          arg2: *const ::std::os::raw::c_char,
                          arg3: lzc_send_flags::Type, arg4: *mut u64)
     -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct dmu_replay_record {
    _unused: [u8; 0],
}
extern "C" {
    pub fn lzc_receive(arg1: *const ::std::os::raw::c_char,
                       arg2: *mut nvlist_t,
                       arg3: *const ::std::os::raw::c_char, arg4: boolean_t,
                       arg5: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_receive_resumable(arg1: *const ::std::os::raw::c_char,
                                 arg2: *mut nvlist_t,
                                 arg3: *const ::std::os::raw::c_char,
                                 arg4: boolean_t, arg5: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_receive_with_header(arg1: *const ::std::os::raw::c_char,
                                   arg2: *mut nvlist_t,
                                   arg3: *const ::std::os::raw::c_char,
                                   arg4: boolean_t, arg5: boolean_t,
                                   arg6: ::std::os::raw::c_int,
                                   arg7: *const dmu_replay_record)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_receive_one(arg1: *const ::std::os::raw::c_char,
                           arg2: *mut nvlist_t,
                           arg3: *const ::std::os::raw::c_char,
                           arg4: boolean_t, arg5: boolean_t,
                           arg6: ::std::os::raw::c_int,
                           arg7: *const dmu_replay_record,
                           arg8: ::std::os::raw::c_int, arg9: *mut u64,
                           arg10: *mut u64, arg11: *mut u64,
                           arg12: *mut *mut nvlist_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_receive_with_cmdprops(arg1: *const ::std::os::raw::c_char,
                                     arg2: *mut nvlist_t, arg3: *mut nvlist_t,
                                     arg4: *const ::std::os::raw::c_char,
                                     arg5: boolean_t, arg6: boolean_t,
                                     arg7: ::std::os::raw::c_int,
                                     arg8: *const dmu_replay_record,
                                     arg9: ::std::os::raw::c_int,
                                     arg10: *mut u64, arg11: *mut u64,
                                     arg12: *mut u64,
                                     arg13: *mut *mut nvlist_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_exists(arg1: *const ::std::os::raw::c_char) -> boolean_t;
}
extern "C" {
    pub fn lzc_rollback(arg1: *const ::std::os::raw::c_char,
                        arg2: *mut ::std::os::raw::c_char,
                        arg3: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_rollback_to(arg1: *const ::std::os::raw::c_char,
                           arg2: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn lzc_sync(arg1: *const ::std::os::raw::c_char, arg2: *mut nvlist_t,
                    arg3: *mut *mut nvlist_t) -> ::std::os::raw::c_int;
}
