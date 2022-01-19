#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    non_snake_case
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

use crate::api::*;
use flutter_rust_bridge::*;

// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_avatar_url(port_: i64, h: *mut wire_Client) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "avatar_url",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_h = h.wire2api();
            move |task_callback| avatar_url(api_h)
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_logged_in(port_: i64, h: *mut wire_Client) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "logged_in",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_h = h.wire2api();
            move |task_callback| logged_in(api_h)
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_homeserver(port_: i64, h: *mut wire_Client) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "homeserver",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_h = h.wire2api();
            move |task_callback| homeserver(api_h)
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_new_client(port_: i64, url: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_client",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_url = url.wire2api();
            move |task_callback| new_client(api_url)
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_echo(port_: i64, url: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "echo",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_url = url.wire2api();
            move |task_callback| echo(api_url)
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_init(port_: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "init",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| init(),
    )
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_Client {
    field0: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: wire enums

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_client() -> *mut wire_Client {
    support::new_leak_box_ptr(wire_Client::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_uint_8_list(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        if self.is_null() {
            None
        } else {
            Some(self.wire2api())
        }
    }
}

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<Client> for *mut wire_Client {
    fn wire2api(self) -> Client {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        (*wrap).wire2api().into()
    }
}

impl Wire2Api<Client> for wire_Client {
    fn wire2api(self) -> Client {
        Client(self.field0.wire2api())
    }
}

impl Wire2Api<u32> for u32 {
    fn wire2api(self) -> u32 {
        self
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_Client {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
        }
    }
}

// Section: impl IntoDart

impl support::IntoDart for Client {
    fn into_dart(self) -> support::DartCObject {
        vec![self.0.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Client {}

// Section: executor
support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}