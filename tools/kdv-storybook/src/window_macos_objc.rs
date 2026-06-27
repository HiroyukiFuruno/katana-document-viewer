use std::ffi::{c_char, c_void};

pub(crate) type Id = *mut c_void;
pub(crate) type Sel = *mut c_void;

#[link(name = "AppKit", kind = "framework")]
unsafe extern "C" {}

#[link(name = "objc")]
unsafe extern "C" {
    fn objc_getClass(name: *const c_char) -> Id;
    fn sel_registerName(name: *const c_char) -> Sel;
    #[link_name = "objc_msgSend"]
    fn objc_msg_send();
}

pub(crate) fn get_class(name: &[u8]) -> Id {
    unsafe { objc_getClass(name.as_ptr().cast::<c_char>()) }
}

pub(crate) fn selector(name: &[u8]) -> Sel {
    unsafe { sel_registerName(name.as_ptr().cast::<c_char>()) }
}

pub(crate) fn msg_send_id(receiver: Id, selector: Sel) -> Id {
    unsafe {
        let send: unsafe extern "C" fn(Id, Sel) -> Id =
            std::mem::transmute(objc_msg_send as *const ());
        send(receiver, selector)
    }
}
