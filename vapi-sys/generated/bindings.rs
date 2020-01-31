/* automatically generated by rust-bindgen */

pub type __uint32_t = ::std::os::raw::c_uint;
pub type __uint64_t = ::std::os::raw::c_ulong;
pub type __off_t = ::std::os::raw::c_long;
pub type __off64_t = ::std::os::raw::c_long;
pub type FILE = _IO_FILE;
pub type _IO_lock_t = ::std::os::raw::c_void;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout__IO_marker() {
    assert_eq!(
        ::std::mem::size_of::<_IO_marker>(),
        24usize,
        concat!("Size of: ", stringify!(_IO_marker))
    );
    assert_eq!(
        ::std::mem::align_of::<_IO_marker>(),
        8usize,
        concat!("Alignment of ", stringify!(_IO_marker))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_marker>()))._next as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_marker),
            "::",
            stringify!(_next)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_marker>()))._sbuf as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_marker),
            "::",
            stringify!(_sbuf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_marker>()))._pos as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_marker),
            "::",
            stringify!(_pos)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _IO_FILE {
    pub _flags: ::std::os::raw::c_int,
    pub _IO_read_ptr: *mut ::std::os::raw::c_char,
    pub _IO_read_end: *mut ::std::os::raw::c_char,
    pub _IO_read_base: *mut ::std::os::raw::c_char,
    pub _IO_write_base: *mut ::std::os::raw::c_char,
    pub _IO_write_ptr: *mut ::std::os::raw::c_char,
    pub _IO_write_end: *mut ::std::os::raw::c_char,
    pub _IO_buf_base: *mut ::std::os::raw::c_char,
    pub _IO_buf_end: *mut ::std::os::raw::c_char,
    pub _IO_save_base: *mut ::std::os::raw::c_char,
    pub _IO_backup_base: *mut ::std::os::raw::c_char,
    pub _IO_save_end: *mut ::std::os::raw::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: ::std::os::raw::c_int,
    pub _flags2: ::std::os::raw::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: ::std::os::raw::c_ushort,
    pub _vtable_offset: ::std::os::raw::c_schar,
    pub _shortbuf: [::std::os::raw::c_char; 1usize],
    pub _lock: *mut _IO_lock_t,
    pub _offset: __off64_t,
    pub __pad1: *mut ::std::os::raw::c_void,
    pub __pad2: *mut ::std::os::raw::c_void,
    pub __pad3: *mut ::std::os::raw::c_void,
    pub __pad4: *mut ::std::os::raw::c_void,
    pub __pad5: usize,
    pub _mode: ::std::os::raw::c_int,
    pub _unused2: [::std::os::raw::c_char; 20usize],
}
#[test]
fn bindgen_test_layout__IO_FILE() {
    assert_eq!(
        ::std::mem::size_of::<_IO_FILE>(),
        216usize,
        concat!("Size of: ", stringify!(_IO_FILE))
    );
    assert_eq!(
        ::std::mem::align_of::<_IO_FILE>(),
        8usize,
        concat!("Alignment of ", stringify!(_IO_FILE))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._flags as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_flags)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_ptr as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_end as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_read_base as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_read_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_base as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_ptr as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_write_end as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_write_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_buf_base as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_buf_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_buf_end as *const _ as usize },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_buf_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_save_base as *const _ as usize },
        72usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_save_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_backup_base as *const _ as usize },
        80usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_backup_base)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._IO_save_end as *const _ as usize },
        88usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_IO_save_end)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._markers as *const _ as usize },
        96usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_markers)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._chain as *const _ as usize },
        104usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_chain)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._fileno as *const _ as usize },
        112usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_fileno)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._flags2 as *const _ as usize },
        116usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_flags2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._old_offset as *const _ as usize },
        120usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_old_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._cur_column as *const _ as usize },
        128usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_cur_column)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._vtable_offset as *const _ as usize },
        130usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_vtable_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._shortbuf as *const _ as usize },
        131usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_shortbuf)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._lock as *const _ as usize },
        136usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_lock)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._offset as *const _ as usize },
        144usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_offset)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad1 as *const _ as usize },
        152usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad1)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad2 as *const _ as usize },
        160usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad3 as *const _ as usize },
        168usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad3)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad4 as *const _ as usize },
        176usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad4)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>())).__pad5 as *const _ as usize },
        184usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(__pad5)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._mode as *const _ as usize },
        192usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<_IO_FILE>()))._unused2 as *const _ as usize },
        196usize,
        concat!(
            "Offset of field: ",
            stringify!(_IO_FILE),
            "::",
            stringify!(_unused2)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vsm {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vsc {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VSC_level_desc {
    pub name: *const ::std::os::raw::c_char,
    pub label: *const ::std::os::raw::c_char,
    pub sdesc: *const ::std::os::raw::c_char,
    pub ldesc: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_VSC_level_desc() {
    assert_eq!(
        ::std::mem::size_of::<VSC_level_desc>(),
        32usize,
        concat!("Size of: ", stringify!(VSC_level_desc))
    );
    assert_eq!(
        ::std::mem::align_of::<VSC_level_desc>(),
        8usize,
        concat!("Alignment of ", stringify!(VSC_level_desc))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSC_level_desc>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VSC_level_desc),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSC_level_desc>())).label as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VSC_level_desc),
            "::",
            stringify!(label)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSC_level_desc>())).sdesc as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VSC_level_desc),
            "::",
            stringify!(sdesc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSC_level_desc>())).ldesc as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VSC_level_desc),
            "::",
            stringify!(ldesc)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VSC_point {
    pub ptr: *const u64,
    pub name: *const ::std::os::raw::c_char,
    pub ctype: *const ::std::os::raw::c_char,
    pub semantics: ::std::os::raw::c_int,
    pub format: ::std::os::raw::c_int,
    pub level: *const VSC_level_desc,
    pub sdesc: *const ::std::os::raw::c_char,
    pub ldesc: *const ::std::os::raw::c_char,
    pub priv_: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_VSC_point() {
    assert_eq!(
        ::std::mem::size_of::<VSC_point>(),
        64usize,
        concat!("Size of: ", stringify!(VSC_point))
    );
    assert_eq!(
        ::std::mem::align_of::<VSC_point>(),
        8usize,
        concat!("Alignment of ", stringify!(VSC_point))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSC_point>())).ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VSC_point),
            "::",
            stringify!(ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSC_point>())).name as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VSC_point),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSC_point>())).ctype as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VSC_point),
            "::",
            stringify!(ctype)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSC_point>())).semantics as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VSC_point),
            "::",
            stringify!(semantics)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSC_point>())).format as *const _ as usize },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(VSC_point),
            "::",
            stringify!(format)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSC_point>())).level as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(VSC_point),
            "::",
            stringify!(level)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSC_point>())).sdesc as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(VSC_point),
            "::",
            stringify!(sdesc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSC_point>())).ldesc as *const _ as usize },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(VSC_point),
            "::",
            stringify!(ldesc)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSC_point>())).priv_ as *const _ as usize },
        56usize,
        concat!(
            "Offset of field: ",
            stringify!(VSC_point),
            "::",
            stringify!(priv_)
        )
    );
}
pub type VSC_new_f = ::std::option::Option<
    unsafe extern "C" fn(
        priv_: *mut ::std::os::raw::c_void,
        pt: *const VSC_point,
    ) -> *mut ::std::os::raw::c_void,
>;
pub type VSC_destroy_f = ::std::option::Option<
    unsafe extern "C" fn(priv_: *mut ::std::os::raw::c_void, pt: *const VSC_point),
>;
pub type VSC_iter_f = ::std::option::Option<
    unsafe extern "C" fn(
        priv_: *mut ::std::os::raw::c_void,
        pt: *const VSC_point,
    ) -> ::std::os::raw::c_int,
>;
extern "C" {
    pub fn VSC_New() -> *mut vsc;
}
extern "C" {
    pub fn VSC_Destroy(arg1: *mut *mut vsc, arg2: *mut vsm);
}
extern "C" {
    pub fn VSC_Arg(
        arg1: *mut vsc,
        arg: ::std::os::raw::c_char,
        opt: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSC_State(
        arg1: *mut vsc,
        arg2: VSC_new_f,
        arg3: VSC_destroy_f,
        arg4: *mut ::std::os::raw::c_void,
    );
}
extern "C" {
    pub fn VSC_Iter(
        arg1: *mut vsc,
        arg2: *mut vsm,
        arg3: VSC_iter_f,
        priv_: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSC_ChangeLevel(
        arg1: *const VSC_level_desc,
        arg2: ::std::os::raw::c_int,
    ) -> *const VSC_level_desc;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vsm_fantom {
    pub priv_: usize,
    pub priv2: usize,
    pub b: *mut ::std::os::raw::c_void,
    pub e: *mut ::std::os::raw::c_void,
    pub class: *mut ::std::os::raw::c_char,
    pub ident: *mut ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_vsm_fantom() {
    assert_eq!(
        ::std::mem::size_of::<vsm_fantom>(),
        48usize,
        concat!("Size of: ", stringify!(vsm_fantom))
    );
    assert_eq!(
        ::std::mem::align_of::<vsm_fantom>(),
        8usize,
        concat!("Alignment of ", stringify!(vsm_fantom))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vsm_fantom>())).priv_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vsm_fantom),
            "::",
            stringify!(priv_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vsm_fantom>())).priv2 as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(vsm_fantom),
            "::",
            stringify!(priv2)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vsm_fantom>())).b as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(vsm_fantom),
            "::",
            stringify!(b)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vsm_fantom>())).e as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(vsm_fantom),
            "::",
            stringify!(e)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vsm_fantom>())).class as *const _ as usize },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(vsm_fantom),
            "::",
            stringify!(class)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vsm_fantom>())).ident as *const _ as usize },
        40usize,
        concat!(
            "Offset of field: ",
            stringify!(vsm_fantom),
            "::",
            stringify!(ident)
        )
    );
}
extern "C" {
    pub fn VSM_New() -> *mut vsm;
}
extern "C" {
    pub fn VSM_Destroy(vd: *mut *mut vsm);
}
extern "C" {
    pub fn VSM_Error(vd: *const vsm) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn VSM_ResetError(vd: *mut vsm);
}
extern "C" {
    pub fn VSM_Arg(
        arg1: *mut vsm,
        flag: ::std::os::raw::c_char,
        arg: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSM_Attach(arg1: *mut vsm, progress: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSM_Status(arg1: *mut vsm) -> ::std::os::raw::c_uint;
}
extern "C" {
    pub fn VSM__iter0(arg1: *const vsm, vf: *mut vsm_fantom);
}
extern "C" {
    pub fn VSM__itern(arg1: *mut vsm, vf: *mut vsm_fantom) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSM_Map(arg1: *mut vsm, vf: *mut vsm_fantom) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSM_Unmap(arg1: *mut vsm, vf: *mut vsm_fantom) -> ::std::os::raw::c_int;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct vsm_valid {
    pub name: *const ::std::os::raw::c_char,
}
#[test]
fn bindgen_test_layout_vsm_valid() {
    assert_eq!(
        ::std::mem::size_of::<vsm_valid>(),
        8usize,
        concat!("Size of: ", stringify!(vsm_valid))
    );
    assert_eq!(
        ::std::mem::align_of::<vsm_valid>(),
        8usize,
        concat!("Alignment of ", stringify!(vsm_valid))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<vsm_valid>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(vsm_valid),
            "::",
            stringify!(name)
        )
    );
}
extern "C" {
    pub fn VSM_StillValid(arg1: *const vsm, vf: *const vsm_fantom) -> *const vsm_valid;
}
extern "C" {
    pub fn VSM_Get(
        arg1: *mut vsm,
        vf: *mut vsm_fantom,
        class: *const ::std::os::raw::c_char,
        ident: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSM_Dup(
        arg1: *mut vsm,
        class: *const ::std::os::raw::c_char,
        ident: *const ::std::os::raw::c_char,
    ) -> *mut ::std::os::raw::c_char;
}
pub const VSL_tag_e_SLT__Bogus: VSL_tag_e = 0;
pub const VSL_tag_e_SLT_Debug: VSL_tag_e = 1;
pub const VSL_tag_e_SLT_Error: VSL_tag_e = 2;
pub const VSL_tag_e_SLT_CLI: VSL_tag_e = 3;
pub const VSL_tag_e_SLT_SessOpen: VSL_tag_e = 4;
pub const VSL_tag_e_SLT_SessClose: VSL_tag_e = 5;
pub const VSL_tag_e_SLT_BackendOpen: VSL_tag_e = 6;
pub const VSL_tag_e_SLT_BackendReuse: VSL_tag_e = 7;
pub const VSL_tag_e_SLT_BackendClose: VSL_tag_e = 8;
pub const VSL_tag_e_SLT_HttpGarbage: VSL_tag_e = 9;
pub const VSL_tag_e_SLT_Proxy: VSL_tag_e = 10;
pub const VSL_tag_e_SLT_ProxyGarbage: VSL_tag_e = 11;
pub const VSL_tag_e_SLT_Backend: VSL_tag_e = 12;
pub const VSL_tag_e_SLT_Length: VSL_tag_e = 13;
pub const VSL_tag_e_SLT_FetchError: VSL_tag_e = 14;
pub const VSL_tag_e_SLT_ReqMethod: VSL_tag_e = 15;
pub const VSL_tag_e_SLT_ReqURL: VSL_tag_e = 16;
pub const VSL_tag_e_SLT_ReqProtocol: VSL_tag_e = 17;
pub const VSL_tag_e_SLT_ReqStatus: VSL_tag_e = 18;
pub const VSL_tag_e_SLT_ReqReason: VSL_tag_e = 19;
pub const VSL_tag_e_SLT_ReqHeader: VSL_tag_e = 20;
pub const VSL_tag_e_SLT_ReqUnset: VSL_tag_e = 21;
pub const VSL_tag_e_SLT_ReqLost: VSL_tag_e = 22;
pub const VSL_tag_e_SLT_RespMethod: VSL_tag_e = 23;
pub const VSL_tag_e_SLT_RespURL: VSL_tag_e = 24;
pub const VSL_tag_e_SLT_RespProtocol: VSL_tag_e = 25;
pub const VSL_tag_e_SLT_RespStatus: VSL_tag_e = 26;
pub const VSL_tag_e_SLT_RespReason: VSL_tag_e = 27;
pub const VSL_tag_e_SLT_RespHeader: VSL_tag_e = 28;
pub const VSL_tag_e_SLT_RespUnset: VSL_tag_e = 29;
pub const VSL_tag_e_SLT_RespLost: VSL_tag_e = 30;
pub const VSL_tag_e_SLT_BereqMethod: VSL_tag_e = 31;
pub const VSL_tag_e_SLT_BereqURL: VSL_tag_e = 32;
pub const VSL_tag_e_SLT_BereqProtocol: VSL_tag_e = 33;
pub const VSL_tag_e_SLT_BereqStatus: VSL_tag_e = 34;
pub const VSL_tag_e_SLT_BereqReason: VSL_tag_e = 35;
pub const VSL_tag_e_SLT_BereqHeader: VSL_tag_e = 36;
pub const VSL_tag_e_SLT_BereqUnset: VSL_tag_e = 37;
pub const VSL_tag_e_SLT_BereqLost: VSL_tag_e = 38;
pub const VSL_tag_e_SLT_BerespMethod: VSL_tag_e = 39;
pub const VSL_tag_e_SLT_BerespURL: VSL_tag_e = 40;
pub const VSL_tag_e_SLT_BerespProtocol: VSL_tag_e = 41;
pub const VSL_tag_e_SLT_BerespStatus: VSL_tag_e = 42;
pub const VSL_tag_e_SLT_BerespReason: VSL_tag_e = 43;
pub const VSL_tag_e_SLT_BerespHeader: VSL_tag_e = 44;
pub const VSL_tag_e_SLT_BerespUnset: VSL_tag_e = 45;
pub const VSL_tag_e_SLT_BerespLost: VSL_tag_e = 46;
pub const VSL_tag_e_SLT_ObjMethod: VSL_tag_e = 47;
pub const VSL_tag_e_SLT_ObjURL: VSL_tag_e = 48;
pub const VSL_tag_e_SLT_ObjProtocol: VSL_tag_e = 49;
pub const VSL_tag_e_SLT_ObjStatus: VSL_tag_e = 50;
pub const VSL_tag_e_SLT_ObjReason: VSL_tag_e = 51;
pub const VSL_tag_e_SLT_ObjHeader: VSL_tag_e = 52;
pub const VSL_tag_e_SLT_ObjUnset: VSL_tag_e = 53;
pub const VSL_tag_e_SLT_ObjLost: VSL_tag_e = 54;
pub const VSL_tag_e_SLT_BogoHeader: VSL_tag_e = 55;
pub const VSL_tag_e_SLT_LostHeader: VSL_tag_e = 56;
pub const VSL_tag_e_SLT_TTL: VSL_tag_e = 57;
pub const VSL_tag_e_SLT_Fetch_Body: VSL_tag_e = 58;
pub const VSL_tag_e_SLT_VCL_acl: VSL_tag_e = 59;
pub const VSL_tag_e_SLT_VCL_call: VSL_tag_e = 60;
pub const VSL_tag_e_SLT_VCL_trace: VSL_tag_e = 61;
pub const VSL_tag_e_SLT_VCL_return: VSL_tag_e = 62;
pub const VSL_tag_e_SLT_ReqStart: VSL_tag_e = 63;
pub const VSL_tag_e_SLT_Hit: VSL_tag_e = 64;
pub const VSL_tag_e_SLT_HitPass: VSL_tag_e = 65;
pub const VSL_tag_e_SLT_ExpBan: VSL_tag_e = 66;
pub const VSL_tag_e_SLT_ExpKill: VSL_tag_e = 67;
pub const VSL_tag_e_SLT_WorkThread: VSL_tag_e = 68;
pub const VSL_tag_e_SLT_ESI_xmlerror: VSL_tag_e = 69;
pub const VSL_tag_e_SLT_Hash: VSL_tag_e = 70;
pub const VSL_tag_e_SLT_Backend_health: VSL_tag_e = 71;
pub const VSL_tag_e_SLT_VCL_Log: VSL_tag_e = 72;
pub const VSL_tag_e_SLT_VCL_Error: VSL_tag_e = 73;
pub const VSL_tag_e_SLT_Gzip: VSL_tag_e = 74;
pub const VSL_tag_e_SLT_Link: VSL_tag_e = 75;
pub const VSL_tag_e_SLT_Begin: VSL_tag_e = 76;
pub const VSL_tag_e_SLT_End: VSL_tag_e = 77;
pub const VSL_tag_e_SLT_VSL: VSL_tag_e = 78;
pub const VSL_tag_e_SLT_Storage: VSL_tag_e = 79;
pub const VSL_tag_e_SLT_Timestamp: VSL_tag_e = 80;
pub const VSL_tag_e_SLT_ReqAcct: VSL_tag_e = 81;
pub const VSL_tag_e_SLT_PipeAcct: VSL_tag_e = 82;
pub const VSL_tag_e_SLT_BereqAcct: VSL_tag_e = 83;
pub const VSL_tag_e_SLT_VfpAcct: VSL_tag_e = 84;
pub const VSL_tag_e_SLT_Witness: VSL_tag_e = 85;
pub const VSL_tag_e_SLT_BackendStart: VSL_tag_e = 86;
pub const VSL_tag_e_SLT_H2RxHdr: VSL_tag_e = 87;
pub const VSL_tag_e_SLT_H2RxBody: VSL_tag_e = 88;
pub const VSL_tag_e_SLT_H2TxHdr: VSL_tag_e = 89;
pub const VSL_tag_e_SLT_H2TxBody: VSL_tag_e = 90;
pub const VSL_tag_e_SLT_HitMiss: VSL_tag_e = 91;
pub const VSL_tag_e_SLT_Filters: VSL_tag_e = 92;
pub const VSL_tag_e_SLT_SessError: VSL_tag_e = 93;
pub const VSL_tag_e_SLT_VCL_use: VSL_tag_e = 94;
pub const VSL_tag_e_SLT__Reserved: VSL_tag_e = 254;
pub const VSL_tag_e_SLT__Batch: VSL_tag_e = 255;
pub type VSL_tag_e = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VSL_data {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VSLQ {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VSLC_ptr {
    pub ptr: *const u32,
    pub priv_: ::std::os::raw::c_uint,
}
#[test]
fn bindgen_test_layout_VSLC_ptr() {
    assert_eq!(
        ::std::mem::size_of::<VSLC_ptr>(),
        16usize,
        concat!("Size of: ", stringify!(VSLC_ptr))
    );
    assert_eq!(
        ::std::mem::align_of::<VSLC_ptr>(),
        8usize,
        concat!("Alignment of ", stringify!(VSLC_ptr))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSLC_ptr>())).ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VSLC_ptr),
            "::",
            stringify!(ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSLC_ptr>())).priv_ as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VSLC_ptr),
            "::",
            stringify!(priv_)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VSL_cursor {
    pub rec: VSLC_ptr,
    pub priv_tbl: *const ::std::os::raw::c_void,
    pub priv_data: *mut ::std::os::raw::c_void,
}
#[test]
fn bindgen_test_layout_VSL_cursor() {
    assert_eq!(
        ::std::mem::size_of::<VSL_cursor>(),
        32usize,
        concat!("Size of: ", stringify!(VSL_cursor))
    );
    assert_eq!(
        ::std::mem::align_of::<VSL_cursor>(),
        8usize,
        concat!("Alignment of ", stringify!(VSL_cursor))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSL_cursor>())).rec as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VSL_cursor),
            "::",
            stringify!(rec)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSL_cursor>())).priv_tbl as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VSL_cursor),
            "::",
            stringify!(priv_tbl)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSL_cursor>())).priv_data as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VSL_cursor),
            "::",
            stringify!(priv_data)
        )
    );
}
pub const VSL_transaction_e_VSL_t_unknown: VSL_transaction_e = 0;
pub const VSL_transaction_e_VSL_t_sess: VSL_transaction_e = 1;
pub const VSL_transaction_e_VSL_t_req: VSL_transaction_e = 2;
pub const VSL_transaction_e_VSL_t_bereq: VSL_transaction_e = 3;
pub const VSL_transaction_e_VSL_t_raw: VSL_transaction_e = 4;
pub const VSL_transaction_e_VSL_t__MAX: VSL_transaction_e = 5;
pub type VSL_transaction_e = u32;
pub const VSL_reason_e_VSL_r_unknown: VSL_reason_e = 0;
pub const VSL_reason_e_VSL_r_http_1: VSL_reason_e = 1;
pub const VSL_reason_e_VSL_r_rxreq: VSL_reason_e = 2;
pub const VSL_reason_e_VSL_r_esi: VSL_reason_e = 3;
pub const VSL_reason_e_VSL_r_restart: VSL_reason_e = 4;
pub const VSL_reason_e_VSL_r_pass: VSL_reason_e = 5;
pub const VSL_reason_e_VSL_r_fetch: VSL_reason_e = 6;
pub const VSL_reason_e_VSL_r_bgfetch: VSL_reason_e = 7;
pub const VSL_reason_e_VSL_r_pipe: VSL_reason_e = 8;
pub const VSL_reason_e_VSL_r__MAX: VSL_reason_e = 9;
pub type VSL_reason_e = u32;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VSL_transaction {
    pub level: ::std::os::raw::c_uint,
    pub vxid: u32,
    pub vxid_parent: u32,
    pub type_: VSL_transaction_e,
    pub reason: VSL_reason_e,
    pub c: *mut VSL_cursor,
}
#[test]
fn bindgen_test_layout_VSL_transaction() {
    assert_eq!(
        ::std::mem::size_of::<VSL_transaction>(),
        32usize,
        concat!("Size of: ", stringify!(VSL_transaction))
    );
    assert_eq!(
        ::std::mem::align_of::<VSL_transaction>(),
        8usize,
        concat!("Alignment of ", stringify!(VSL_transaction))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSL_transaction>())).level as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(VSL_transaction),
            "::",
            stringify!(level)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSL_transaction>())).vxid as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(VSL_transaction),
            "::",
            stringify!(vxid)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSL_transaction>())).vxid_parent as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(VSL_transaction),
            "::",
            stringify!(vxid_parent)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSL_transaction>())).type_ as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(VSL_transaction),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSL_transaction>())).reason as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(VSL_transaction),
            "::",
            stringify!(reason)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<VSL_transaction>())).c as *const _ as usize },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(VSL_transaction),
            "::",
            stringify!(c)
        )
    );
}
pub const VSL_grouping_e_VSL_g_raw: VSL_grouping_e = 0;
pub const VSL_grouping_e_VSL_g_vxid: VSL_grouping_e = 1;
pub const VSL_grouping_e_VSL_g_request: VSL_grouping_e = 2;
pub const VSL_grouping_e_VSL_g_session: VSL_grouping_e = 3;
pub const VSL_grouping_e_VSL_g__MAX: VSL_grouping_e = 4;
pub type VSL_grouping_e = u32;
pub type VSLQ_dispatch_f = ::std::option::Option<
    unsafe extern "C" fn(
        vsl: *mut VSL_data,
        trans: *const *mut VSL_transaction,
        priv_: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int,
>;
pub type VSL_tagfind_f = ::std::option::Option<
    unsafe extern "C" fn(tag: ::std::os::raw::c_int, priv_: *mut ::std::os::raw::c_void),
>;
extern "C" {
    pub fn VSL_Name2Tag(
        name: *const ::std::os::raw::c_char,
        l: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSL_Glob2Tags(
        glob: *const ::std::os::raw::c_char,
        l: ::std::os::raw::c_int,
        func: VSL_tagfind_f,
        priv_: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSL_List2Tags(
        list: *const ::std::os::raw::c_char,
        l: ::std::os::raw::c_int,
        func: VSL_tagfind_f,
        priv_: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSLQ_Name2Grouping(
        name: *const ::std::os::raw::c_char,
        l: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSL_New() -> *mut VSL_data;
}
extern "C" {
    pub fn VSL_Arg(
        vsl: *mut VSL_data,
        opt: ::std::os::raw::c_int,
        arg: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSL_Delete(vsl: *mut VSL_data);
}
extern "C" {
    pub fn VSL_Error(vsl: *const VSL_data) -> *const ::std::os::raw::c_char;
}
extern "C" {
    pub fn VSL_ResetError(vsl: *mut VSL_data);
}
extern "C" {
    pub fn VSL_CursorVSM(
        vsl: *mut VSL_data,
        vsm: *mut vsm,
        options: ::std::os::raw::c_uint,
    ) -> *mut VSL_cursor;
}
extern "C" {
    pub fn VSL_CursorFile(
        vsl: *mut VSL_data,
        name: *const ::std::os::raw::c_char,
        options: ::std::os::raw::c_uint,
    ) -> *mut VSL_cursor;
}
extern "C" {
    pub fn VSL_DeleteCursor(c: *const VSL_cursor);
}
extern "C" {
    pub fn VSL_ResetCursor(c: *const VSL_cursor) -> vsl_status;
}
pub const vsl_check_vsl_check_e_notsupp: vsl_check = -1;
pub const vsl_check_vsl_check_e_inval: vsl_check = 0;
pub const vsl_check_vsl_check_warn: vsl_check = 1;
pub const vsl_check_vsl_check_valid: vsl_check = 2;
pub type vsl_check = i32;
extern "C" {
    pub fn VSL_Check(c: *const VSL_cursor, ptr: *const VSLC_ptr) -> vsl_check;
}
pub const vsl_status_vsl_e_write: vsl_status = -5;
pub const vsl_status_vsl_e_io: vsl_status = -4;
pub const vsl_status_vsl_e_overrun: vsl_status = -3;
pub const vsl_status_vsl_e_abandon: vsl_status = -2;
pub const vsl_status_vsl_e_eof: vsl_status = -1;
pub const vsl_status_vsl_end: vsl_status = 0;
pub const vsl_status_vsl_more: vsl_status = 1;
pub type vsl_status = i32;
extern "C" {
    pub fn VSL_Next(c: *const VSL_cursor) -> vsl_status;
}
extern "C" {
    pub fn VSL_Match(vsl: *mut VSL_data, c: *const VSL_cursor) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSL_Print(
        vsl: *const VSL_data,
        c: *const VSL_cursor,
        fo: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSL_PrintTerse(
        vsl: *const VSL_data,
        c: *const VSL_cursor,
        fo: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSL_PrintAll(
        vsl: *mut VSL_data,
        c: *const VSL_cursor,
        fo: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSL_PrintTransactions(
        vsl: *mut VSL_data,
        trans: *const *mut VSL_transaction,
        priv_: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSL_WriteOpen(
        vsl: *mut VSL_data,
        name: *const ::std::os::raw::c_char,
        append: ::std::os::raw::c_int,
        unbuffered: ::std::os::raw::c_int,
    ) -> *mut FILE;
}
extern "C" {
    pub fn VSL_Write(
        vsl: *const VSL_data,
        c: *const VSL_cursor,
        fo: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSL_WriteAll(
        vsl: *mut VSL_data,
        c: *const VSL_cursor,
        fo: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSL_WriteTransactions(
        vsl: *mut VSL_data,
        trans: *const *mut VSL_transaction,
        priv_: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSLQ_New(
        vsl: *mut VSL_data,
        cp: *mut *mut VSL_cursor,
        grouping: VSL_grouping_e,
        query: *const ::std::os::raw::c_char,
    ) -> *mut VSLQ;
}
extern "C" {
    pub fn VSLQ_Delete(pvslq: *mut *mut VSLQ);
}
extern "C" {
    pub fn VSLQ_SetCursor(vslq: *mut VSLQ, cp: *mut *mut VSL_cursor);
}
extern "C" {
    pub fn VSLQ_Dispatch(
        vslq: *mut VSLQ,
        func: VSLQ_dispatch_f,
        priv_: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn VSLQ_Flush(
        vslq: *mut VSLQ,
        func: VSLQ_dispatch_f,
        priv_: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
