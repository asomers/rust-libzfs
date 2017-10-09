/* automatically generated by rust-bindgen */

pub mod boolean {
    pub type Type = ::std::os::raw::c_uint;
    pub const B_FALSE: Type = 0;
    pub const B_TRUE: Type = 1;
}
pub use self::boolean::Type as boolean_t;
pub type uchar_t = ::std::os::raw::c_uchar;
pub type uint_t = ::std::os::raw::c_uint;
pub type hrtime_t = ::std::os::raw::c_longlong;
pub mod data_type_t {
    pub type Type = ::std::os::raw::c_uint;
    pub const DATA_TYPE_UNKNOWN: Type = 0;
    pub const DATA_TYPE_BOOLEAN: Type = 1;
    pub const DATA_TYPE_BYTE: Type = 2;
    pub const DATA_TYPE_INT16: Type = 3;
    pub const DATA_TYPE_UINT16: Type = 4;
    pub const DATA_TYPE_INT32: Type = 5;
    pub const DATA_TYPE_UINT32: Type = 6;
    pub const DATA_TYPE_INT64: Type = 7;
    pub const DATA_TYPE_UINT64: Type = 8;
    pub const DATA_TYPE_STRING: Type = 9;
    pub const DATA_TYPE_BYTE_ARRAY: Type = 10;
    pub const DATA_TYPE_INT16_ARRAY: Type = 11;
    pub const DATA_TYPE_UINT16_ARRAY: Type = 12;
    pub const DATA_TYPE_INT32_ARRAY: Type = 13;
    pub const DATA_TYPE_UINT32_ARRAY: Type = 14;
    pub const DATA_TYPE_INT64_ARRAY: Type = 15;
    pub const DATA_TYPE_UINT64_ARRAY: Type = 16;
    pub const DATA_TYPE_STRING_ARRAY: Type = 17;
    pub const DATA_TYPE_HRTIME: Type = 18;
    pub const DATA_TYPE_NVLIST: Type = 19;
    pub const DATA_TYPE_NVLIST_ARRAY: Type = 20;
    pub const DATA_TYPE_BOOLEAN_VALUE: Type = 21;
    pub const DATA_TYPE_INT8: Type = 22;
    pub const DATA_TYPE_UINT8: Type = 23;
    pub const DATA_TYPE_BOOLEAN_ARRAY: Type = 24;
    pub const DATA_TYPE_INT8_ARRAY: Type = 25;
    pub const DATA_TYPE_UINT8_ARRAY: Type = 26;
    pub const DATA_TYPE_DOUBLE: Type = 27;
}
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nvpair {
    pub nvp_size: i32,
    pub nvp_name_sz: i16,
    pub nvp_reserve: i16,
    pub nvp_value_elem: i32,
    pub nvp_type: data_type_t::Type,
}
#[test]
fn bindgen_test_layout_nvpair() {
    assert_eq!(::std::mem::size_of::<nvpair>() , 16usize , concat ! (
               "Size of: " , stringify ! ( nvpair ) ));
    assert_eq! (::std::mem::align_of::<nvpair>() , 4usize , concat ! (
                "Alignment of " , stringify ! ( nvpair ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nvpair ) ) . nvp_size as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( nvpair ) , "::" ,
                stringify ! ( nvp_size ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nvpair ) ) . nvp_name_sz as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( nvpair ) , "::" ,
                stringify ! ( nvp_name_sz ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nvpair ) ) . nvp_reserve as * const _ as
                usize } , 6usize , concat ! (
                "Alignment of field: " , stringify ! ( nvpair ) , "::" ,
                stringify ! ( nvp_reserve ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nvpair ) ) . nvp_value_elem as * const _
                as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( nvpair ) , "::" ,
                stringify ! ( nvp_value_elem ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nvpair ) ) . nvp_type as * const _ as
                usize } , 12usize , concat ! (
                "Alignment of field: " , stringify ! ( nvpair ) , "::" ,
                stringify ! ( nvp_type ) ));
}
impl Clone for nvpair {
    fn clone(&self) -> Self { *self }
}
pub type nvpair_t = nvpair;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nvlist {
    pub nvl_version: i32,
    pub nvl_nvflag: u32,
    pub nvl_priv: u64,
    pub nvl_flag: u32,
    pub nvl_pad: i32,
}
#[test]
fn bindgen_test_layout_nvlist() {
    assert_eq!(::std::mem::size_of::<nvlist>() , 24usize , concat ! (
               "Size of: " , stringify ! ( nvlist ) ));
    assert_eq! (::std::mem::align_of::<nvlist>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( nvlist ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nvlist ) ) . nvl_version as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( nvlist ) , "::" ,
                stringify ! ( nvl_version ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nvlist ) ) . nvl_nvflag as * const _ as
                usize } , 4usize , concat ! (
                "Alignment of field: " , stringify ! ( nvlist ) , "::" ,
                stringify ! ( nvl_nvflag ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nvlist ) ) . nvl_priv as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( nvlist ) , "::" ,
                stringify ! ( nvl_priv ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nvlist ) ) . nvl_flag as * const _ as
                usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( nvlist ) , "::" ,
                stringify ! ( nvl_flag ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nvlist ) ) . nvl_pad as * const _ as
                usize } , 20usize , concat ! (
                "Alignment of field: " , stringify ! ( nvlist ) , "::" ,
                stringify ! ( nvl_pad ) ));
}
impl Clone for nvlist {
    fn clone(&self) -> Self { *self }
}
pub type nvlist_t = nvlist;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nv_alloc_ops {
    pub nv_ao_init: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                   *mut nv_alloc_t,
                                                               arg2:
                                                                   *mut __va_list_tag)
                                              -> ::std::os::raw::c_int>,
    pub nv_ao_fini: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                   *mut nv_alloc_t)>,
    pub nv_ao_alloc: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut nv_alloc_t,
                                                                arg2: usize)
                                               ->
                                                   *mut ::std::os::raw::c_void>,
    pub nv_ao_free: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                   *mut nv_alloc_t,
                                                               arg2:
                                                                   *mut ::std::os::raw::c_void,
                                                               arg3: usize)>,
    pub nv_ao_reset: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                    *mut nv_alloc_t)>,
}
#[test]
fn bindgen_test_layout_nv_alloc_ops() {
    assert_eq!(::std::mem::size_of::<nv_alloc_ops>() , 40usize , concat ! (
               "Size of: " , stringify ! ( nv_alloc_ops ) ));
    assert_eq! (::std::mem::align_of::<nv_alloc_ops>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( nv_alloc_ops ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nv_alloc_ops ) ) . nv_ao_init as * const
                _ as usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( nv_alloc_ops ) , "::" ,
                stringify ! ( nv_ao_init ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nv_alloc_ops ) ) . nv_ao_fini as * const
                _ as usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( nv_alloc_ops ) , "::" ,
                stringify ! ( nv_ao_fini ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nv_alloc_ops ) ) . nv_ao_alloc as * const
                _ as usize } , 16usize , concat ! (
                "Alignment of field: " , stringify ! ( nv_alloc_ops ) , "::" ,
                stringify ! ( nv_ao_alloc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nv_alloc_ops ) ) . nv_ao_free as * const
                _ as usize } , 24usize , concat ! (
                "Alignment of field: " , stringify ! ( nv_alloc_ops ) , "::" ,
                stringify ! ( nv_ao_free ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nv_alloc_ops ) ) . nv_ao_reset as * const
                _ as usize } , 32usize , concat ! (
                "Alignment of field: " , stringify ! ( nv_alloc_ops ) , "::" ,
                stringify ! ( nv_ao_reset ) ));
}
impl Clone for nv_alloc_ops {
    fn clone(&self) -> Self { *self }
}
pub type nv_alloc_ops_t = nv_alloc_ops;
#[repr(C)]
#[derive(Debug, Copy)]
pub struct nv_alloc {
    pub nva_ops: *const nv_alloc_ops_t,
    pub nva_arg: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_nv_alloc() {
    assert_eq!(::std::mem::size_of::<nv_alloc>() , 16usize , concat ! (
               "Size of: " , stringify ! ( nv_alloc ) ));
    assert_eq! (::std::mem::align_of::<nv_alloc>() , 8usize , concat ! (
                "Alignment of " , stringify ! ( nv_alloc ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nv_alloc ) ) . nva_ops as * const _ as
                usize } , 0usize , concat ! (
                "Alignment of field: " , stringify ! ( nv_alloc ) , "::" ,
                stringify ! ( nva_ops ) ));
    assert_eq! (unsafe {
                & ( * ( 0 as * const nv_alloc ) ) . nva_arg as * const _ as
                usize } , 8usize , concat ! (
                "Alignment of field: " , stringify ! ( nv_alloc ) , "::" ,
                stringify ! ( nva_arg ) ));
}
impl Clone for nv_alloc {
    fn clone(&self) -> Self { *self }
}
pub type nv_alloc_t = nv_alloc;
extern "C" {
    pub fn nvlist_alloc(arg1: *mut *mut nvlist_t, arg2: uint_t,
                        arg3: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_free(arg1: *mut nvlist_t);
}
extern "C" {
    pub fn nvlist_size(arg1: *mut nvlist_t, arg2: *mut usize,
                       arg3: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_pack(arg1: *mut nvlist_t,
                       arg2: *mut *mut ::std::os::raw::c_char,
                       arg3: *mut usize, arg4: ::std::os::raw::c_int,
                       arg5: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_unpack(arg1: *mut ::std::os::raw::c_char, arg2: usize,
                         arg3: *mut *mut nvlist_t,
                         arg4: ::std::os::raw::c_int)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_dup(arg1: *mut nvlist_t, arg2: *mut *mut nvlist_t,
                      arg3: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_merge(arg1: *mut nvlist_t, arg2: *mut nvlist_t,
                        arg3: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_nvflag(arg1: *mut nvlist_t) -> uint_t;
}
extern "C" {
    pub fn nvlist_xalloc(arg1: *mut *mut nvlist_t, arg2: uint_t,
                         arg3: *mut nv_alloc_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_xpack(arg1: *mut nvlist_t,
                        arg2: *mut *mut ::std::os::raw::c_char,
                        arg3: *mut usize, arg4: ::std::os::raw::c_int,
                        arg5: *mut nv_alloc_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_xunpack(arg1: *mut ::std::os::raw::c_char, arg2: usize,
                          arg3: *mut *mut nvlist_t, arg4: *mut nv_alloc_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_xdup(arg1: *mut nvlist_t, arg2: *mut *mut nvlist_t,
                       arg3: *mut nv_alloc_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_nv_alloc(arg1: *mut nvlist_t) -> *mut nv_alloc_t;
}
extern "C" {
    pub fn nvlist_add_nvpair(arg1: *mut nvlist_t, arg2: *mut nvpair_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_boolean(arg1: *mut nvlist_t,
                              arg2: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_boolean_value(arg1: *mut nvlist_t,
                                    arg2: *const ::std::os::raw::c_char,
                                    arg3: boolean_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_byte(arg1: *mut nvlist_t,
                           arg2: *const ::std::os::raw::c_char, arg3: uchar_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_int8(arg1: *mut nvlist_t,
                           arg2: *const ::std::os::raw::c_char, arg3: i8)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_uint8(arg1: *mut nvlist_t,
                            arg2: *const ::std::os::raw::c_char, arg3: u8)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_int16(arg1: *mut nvlist_t,
                            arg2: *const ::std::os::raw::c_char, arg3: i16)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_uint16(arg1: *mut nvlist_t,
                             arg2: *const ::std::os::raw::c_char, arg3: u16)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_int32(arg1: *mut nvlist_t,
                            arg2: *const ::std::os::raw::c_char, arg3: i32)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_uint32(arg1: *mut nvlist_t,
                             arg2: *const ::std::os::raw::c_char, arg3: u32)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_int64(arg1: *mut nvlist_t,
                            arg2: *const ::std::os::raw::c_char, arg3: i64)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_uint64(arg1: *mut nvlist_t,
                             arg2: *const ::std::os::raw::c_char, arg3: u64)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_string(arg1: *mut nvlist_t,
                             arg2: *const ::std::os::raw::c_char,
                             arg3: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_nvlist(arg1: *mut nvlist_t,
                             arg2: *const ::std::os::raw::c_char,
                             arg3: *mut nvlist_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_boolean_array(arg1: *mut nvlist_t,
                                    arg2: *const ::std::os::raw::c_char,
                                    arg3: *mut boolean_t, arg4: uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_byte_array(arg1: *mut nvlist_t,
                                 arg2: *const ::std::os::raw::c_char,
                                 arg3: *mut uchar_t, arg4: uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_int8_array(arg1: *mut nvlist_t,
                                 arg2: *const ::std::os::raw::c_char,
                                 arg3: *mut i8, arg4: uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_uint8_array(arg1: *mut nvlist_t,
                                  arg2: *const ::std::os::raw::c_char,
                                  arg3: *mut u8, arg4: uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_int16_array(arg1: *mut nvlist_t,
                                  arg2: *const ::std::os::raw::c_char,
                                  arg3: *mut i16, arg4: uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_uint16_array(arg1: *mut nvlist_t,
                                   arg2: *const ::std::os::raw::c_char,
                                   arg3: *mut u16, arg4: uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_int32_array(arg1: *mut nvlist_t,
                                  arg2: *const ::std::os::raw::c_char,
                                  arg3: *mut i32, arg4: uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_uint32_array(arg1: *mut nvlist_t,
                                   arg2: *const ::std::os::raw::c_char,
                                   arg3: *mut u32, arg4: uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_int64_array(arg1: *mut nvlist_t,
                                  arg2: *const ::std::os::raw::c_char,
                                  arg3: *mut i64, arg4: uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_uint64_array(arg1: *mut nvlist_t,
                                   arg2: *const ::std::os::raw::c_char,
                                   arg3: *mut u64, arg4: uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_string_array(arg1: *mut nvlist_t,
                                   arg2: *const ::std::os::raw::c_char,
                                   arg3: *const *const ::std::os::raw::c_char,
                                   arg4: uint_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_nvlist_array(arg1: *mut nvlist_t,
                                   arg2: *const ::std::os::raw::c_char,
                                   arg3: *mut *mut nvlist_t, arg4: uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_hrtime(arg1: *mut nvlist_t,
                             arg2: *const ::std::os::raw::c_char,
                             arg3: hrtime_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_add_double(arg1: *mut nvlist_t,
                             arg2: *const ::std::os::raw::c_char, arg3: f64)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_remove(arg1: *mut nvlist_t,
                         arg2: *const ::std::os::raw::c_char,
                         arg3: data_type_t::Type) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_remove_all(arg1: *mut nvlist_t,
                             arg2: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_remove_nvpair(arg1: *mut nvlist_t, arg2: *mut nvpair_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_boolean(arg1: *mut nvlist_t,
                                 arg2: *const ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_boolean_value(arg1: *mut nvlist_t,
                                       arg2: *const ::std::os::raw::c_char,
                                       arg3: *mut boolean_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_byte(arg1: *mut nvlist_t,
                              arg2: *const ::std::os::raw::c_char,
                              arg3: *mut uchar_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_int8(arg1: *mut nvlist_t,
                              arg2: *const ::std::os::raw::c_char,
                              arg3: *mut i8) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_uint8(arg1: *mut nvlist_t,
                               arg2: *const ::std::os::raw::c_char,
                               arg3: *mut u8) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_int16(arg1: *mut nvlist_t,
                               arg2: *const ::std::os::raw::c_char,
                               arg3: *mut i16) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_uint16(arg1: *mut nvlist_t,
                                arg2: *const ::std::os::raw::c_char,
                                arg3: *mut u16) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_int32(arg1: *mut nvlist_t,
                               arg2: *const ::std::os::raw::c_char,
                               arg3: *mut i32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_uint32(arg1: *mut nvlist_t,
                                arg2: *const ::std::os::raw::c_char,
                                arg3: *mut u32) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_int64(arg1: *mut nvlist_t,
                               arg2: *const ::std::os::raw::c_char,
                               arg3: *mut i64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_uint64(arg1: *mut nvlist_t,
                                arg2: *const ::std::os::raw::c_char,
                                arg3: *mut u64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_string(arg1: *mut nvlist_t,
                                arg2: *const ::std::os::raw::c_char,
                                arg3: *mut *mut ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_nvlist(arg1: *mut nvlist_t,
                                arg2: *const ::std::os::raw::c_char,
                                arg3: *mut *mut nvlist_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_boolean_array(arg1: *mut nvlist_t,
                                       arg2: *const ::std::os::raw::c_char,
                                       arg3: *mut *mut boolean_t,
                                       arg4: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_byte_array(arg1: *mut nvlist_t,
                                    arg2: *const ::std::os::raw::c_char,
                                    arg3: *mut *mut uchar_t,
                                    arg4: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_int8_array(arg1: *mut nvlist_t,
                                    arg2: *const ::std::os::raw::c_char,
                                    arg3: *mut *mut i8, arg4: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_uint8_array(arg1: *mut nvlist_t,
                                     arg2: *const ::std::os::raw::c_char,
                                     arg3: *mut *mut u8, arg4: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_int16_array(arg1: *mut nvlist_t,
                                     arg2: *const ::std::os::raw::c_char,
                                     arg3: *mut *mut i16, arg4: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_uint16_array(arg1: *mut nvlist_t,
                                      arg2: *const ::std::os::raw::c_char,
                                      arg3: *mut *mut u16, arg4: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_int32_array(arg1: *mut nvlist_t,
                                     arg2: *const ::std::os::raw::c_char,
                                     arg3: *mut *mut i32, arg4: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_uint32_array(arg1: *mut nvlist_t,
                                      arg2: *const ::std::os::raw::c_char,
                                      arg3: *mut *mut u32, arg4: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_int64_array(arg1: *mut nvlist_t,
                                     arg2: *const ::std::os::raw::c_char,
                                     arg3: *mut *mut i64, arg4: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_uint64_array(arg1: *mut nvlist_t,
                                      arg2: *const ::std::os::raw::c_char,
                                      arg3: *mut *mut u64, arg4: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_string_array(arg1: *mut nvlist_t,
                                      arg2: *const ::std::os::raw::c_char,
                                      arg3:
                                          *mut *mut *mut ::std::os::raw::c_char,
                                      arg4: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_nvlist_array(arg1: *mut nvlist_t,
                                      arg2: *const ::std::os::raw::c_char,
                                      arg3: *mut *mut *mut nvlist_t,
                                      arg4: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_hrtime(arg1: *mut nvlist_t,
                                arg2: *const ::std::os::raw::c_char,
                                arg3: *mut hrtime_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_pairs(arg1: *mut nvlist_t,
                               arg2: ::std::os::raw::c_int, ...)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_double(arg1: *mut nvlist_t,
                                arg2: *const ::std::os::raw::c_char,
                                arg3: *mut f64) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_nvpair(arg1: *mut nvlist_t,
                                arg2: *const ::std::os::raw::c_char,
                                arg3: *mut *mut nvpair_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_lookup_nvpair_embedded_index(arg1: *mut nvlist_t,
                                               arg2:
                                                   *const ::std::os::raw::c_char,
                                               arg3: *mut *mut nvpair_t,
                                               arg4:
                                                   *mut ::std::os::raw::c_int,
                                               arg5:
                                                   *mut *mut ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvlist_exists(arg1: *mut nvlist_t,
                         arg2: *const ::std::os::raw::c_char) -> boolean_t;
}
extern "C" {
    pub fn nvlist_empty(arg1: *mut nvlist_t) -> boolean_t;
}
extern "C" {
    pub fn nvlist_next_nvpair(arg1: *mut nvlist_t, arg2: *mut nvpair_t)
     -> *mut nvpair_t;
}
extern "C" {
    pub fn nvlist_prev_nvpair(arg1: *mut nvlist_t, arg2: *mut nvpair_t)
     -> *mut nvpair_t;
}
extern "C" {
    pub fn nvpair_name(arg1: *mut nvpair_t) -> *mut ::std::os::raw::c_char;
}
extern "C" {
    pub fn nvpair_type(arg1: *mut nvpair_t) -> data_type_t::Type;
}
extern "C" {
    pub fn nvpair_type_is_array(arg1: *mut nvpair_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_boolean_value(arg1: *mut nvpair_t,
                                      arg2: *mut boolean_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_byte(arg1: *mut nvpair_t, arg2: *mut uchar_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_int8(arg1: *mut nvpair_t, arg2: *mut i8)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_uint8(arg1: *mut nvpair_t, arg2: *mut u8)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_int16(arg1: *mut nvpair_t, arg2: *mut i16)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_uint16(arg1: *mut nvpair_t, arg2: *mut u16)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_int32(arg1: *mut nvpair_t, arg2: *mut i32)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_uint32(arg1: *mut nvpair_t, arg2: *mut u32)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_int64(arg1: *mut nvpair_t, arg2: *mut i64)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_uint64(arg1: *mut nvpair_t, arg2: *mut u64)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_string(arg1: *mut nvpair_t,
                               arg2: *mut *mut ::std::os::raw::c_char)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_nvlist(arg1: *mut nvpair_t, arg2: *mut *mut nvlist_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_boolean_array(arg1: *mut nvpair_t,
                                      arg2: *mut *mut boolean_t,
                                      arg3: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_byte_array(arg1: *mut nvpair_t,
                                   arg2: *mut *mut uchar_t, arg3: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_int8_array(arg1: *mut nvpair_t, arg2: *mut *mut i8,
                                   arg3: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_uint8_array(arg1: *mut nvpair_t, arg2: *mut *mut u8,
                                    arg3: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_int16_array(arg1: *mut nvpair_t, arg2: *mut *mut i16,
                                    arg3: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_uint16_array(arg1: *mut nvpair_t, arg2: *mut *mut u16,
                                     arg3: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_int32_array(arg1: *mut nvpair_t, arg2: *mut *mut i32,
                                    arg3: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_uint32_array(arg1: *mut nvpair_t, arg2: *mut *mut u32,
                                     arg3: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_int64_array(arg1: *mut nvpair_t, arg2: *mut *mut i64,
                                    arg3: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_uint64_array(arg1: *mut nvpair_t, arg2: *mut *mut u64,
                                     arg3: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_string_array(arg1: *mut nvpair_t,
                                     arg2:
                                         *mut *mut *mut ::std::os::raw::c_char,
                                     arg3: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_nvlist_array(arg1: *mut nvpair_t,
                                     arg2: *mut *mut *mut nvlist_t,
                                     arg3: *mut uint_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_hrtime(arg1: *mut nvpair_t, arg2: *mut hrtime_t)
     -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn nvpair_value_double(arg1: *mut nvpair_t, arg2: *mut f64)
     -> ::std::os::raw::c_int;
}
