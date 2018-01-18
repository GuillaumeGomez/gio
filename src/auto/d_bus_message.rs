// This file was generated by gir (5a68ad0) from gir-files (469db10)
// DO NOT EDIT

use DBusCapabilityFlags;
use DBusMessageByteOrder;
use DBusMessageFlags;
use DBusMessageHeaderField;
use DBusMessageType;
use Error;
use ffi;
use glib;
use glib::StaticType;
use glib::Value;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct DBusMessage(Object<ffi::GDBusMessage>);

    match fn {
        get_type => || ffi::g_dbus_message_get_type(),
    }
}

impl DBusMessage {
    pub fn new() -> DBusMessage {
        unsafe {
            from_glib_full(ffi::g_dbus_message_new())
        }
    }

    pub fn new_from_blob(blob: &[u8], capabilities: DBusCapabilityFlags) -> Result<DBusMessage, Error> {
        let blob_len = blob.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_message_new_from_blob(blob.to_glib_none().0, blob_len, capabilities.to_glib(), &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn new_method_call<'a, 'b, P: Into<Option<&'a str>>, Q: Into<Option<&'b str>>>(name: P, path: &str, interface_: Q, method: &str) -> DBusMessage {
        let name = name.into();
        let name = name.to_glib_none();
        let interface_ = interface_.into();
        let interface_ = interface_.to_glib_none();
        unsafe {
            from_glib_full(ffi::g_dbus_message_new_method_call(name.0, path.to_glib_none().0, interface_.0, method.to_glib_none().0))
        }
    }

    pub fn new_signal(path: &str, interface_: &str, signal: &str) -> DBusMessage {
        unsafe {
            from_glib_full(ffi::g_dbus_message_new_signal(path.to_glib_none().0, interface_.to_glib_none().0, signal.to_glib_none().0))
        }
    }

    pub fn bytes_needed(blob: &[u8]) -> Result<isize, Error> {
        let blob_len = blob.len() as usize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_message_bytes_needed(blob.to_glib_none().0, blob_len, &mut error);
            if error.is_null() { Ok(ret) } else { Err(from_glib_full(error)) }
        }
    }
}

impl Default for DBusMessage {
    fn default() -> Self {
        Self::new()
    }
}

pub trait DBusMessageExt {
    fn copy(&self) -> Result<DBusMessage, Error>;

    fn get_arg0(&self) -> Option<String>;

    fn get_body(&self) -> Option<glib::Variant>;

    fn get_byte_order(&self) -> DBusMessageByteOrder;

    fn get_destination(&self) -> Option<String>;

    fn get_error_name(&self) -> Option<String>;

    fn get_flags(&self) -> DBusMessageFlags;

    fn get_header(&self, header_field: DBusMessageHeaderField) -> Option<glib::Variant>;

    fn get_interface(&self) -> Option<String>;

    fn get_locked(&self) -> bool;

    fn get_member(&self) -> Option<String>;

    fn get_message_type(&self) -> DBusMessageType;

    fn get_num_unix_fds(&self) -> u32;

    fn get_path(&self) -> Option<String>;

    fn get_reply_serial(&self) -> u32;

    fn get_sender(&self) -> Option<String>;

    fn get_serial(&self) -> u32;

    fn get_signature(&self) -> Option<String>;

    //fn get_unix_fd_list(&self) -> /*Ignored*/Option<UnixFDList>;

    fn lock(&self);

    //fn new_method_error(&self, error_name: &str, error_message_format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<DBusMessage>;

    fn new_method_error_literal(&self, error_name: &str, error_message: &str) -> Option<DBusMessage>;

    //fn new_method_error_valist(&self, error_name: &str, error_message_format: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<DBusMessage>;

    fn new_method_reply(&self) -> Option<DBusMessage>;

    fn print(&self, indent: u32) -> Option<String>;

    fn set_body(&self, body: &glib::Variant);

    fn set_byte_order(&self, byte_order: DBusMessageByteOrder);

    fn set_destination(&self, value: &str);

    fn set_error_name(&self, value: &str);

    fn set_flags(&self, flags: DBusMessageFlags);

    fn set_header<'a, P: Into<Option<&'a glib::Variant>>>(&self, header_field: DBusMessageHeaderField, value: P);

    fn set_interface(&self, value: &str);

    fn set_member(&self, value: &str);

    fn set_message_type(&self, type_: DBusMessageType);

    fn set_num_unix_fds(&self, value: u32);

    fn set_path(&self, value: &str);

    fn set_reply_serial(&self, value: u32);

    fn set_sender(&self, value: &str);

    fn set_serial(&self, serial: u32);

    fn set_signature(&self, value: &str);

    //fn set_unix_fd_list<'a, P: Into<Option<&'a /*Ignored*/UnixFDList>>>(&self, fd_list: P);

    fn to_blob(&self, capabilities: DBusCapabilityFlags) -> Result<Vec<u8>, Error>;

    fn to_gerror(&self) -> Result<(), Error>;

    fn get_property_locked(&self) -> bool;

    fn connect_property_locked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<DBusMessage> + IsA<glib::object::Object>> DBusMessageExt for O {
    fn copy(&self) -> Result<DBusMessage, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_message_copy(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_arg0(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_dbus_message_get_arg0(self.to_glib_none().0))
        }
    }

    fn get_body(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_none(ffi::g_dbus_message_get_body(self.to_glib_none().0))
        }
    }

    fn get_byte_order(&self) -> DBusMessageByteOrder {
        unsafe {
            from_glib(ffi::g_dbus_message_get_byte_order(self.to_glib_none().0))
        }
    }

    fn get_destination(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_dbus_message_get_destination(self.to_glib_none().0))
        }
    }

    fn get_error_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_dbus_message_get_error_name(self.to_glib_none().0))
        }
    }

    fn get_flags(&self) -> DBusMessageFlags {
        unsafe {
            from_glib(ffi::g_dbus_message_get_flags(self.to_glib_none().0))
        }
    }

    fn get_header(&self, header_field: DBusMessageHeaderField) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_dbus_message_get_header(self.to_glib_none().0, header_field.to_glib()))
        }
    }

    fn get_interface(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_dbus_message_get_interface(self.to_glib_none().0))
        }
    }

    fn get_locked(&self) -> bool {
        unsafe {
            from_glib(ffi::g_dbus_message_get_locked(self.to_glib_none().0))
        }
    }

    fn get_member(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_dbus_message_get_member(self.to_glib_none().0))
        }
    }

    fn get_message_type(&self) -> DBusMessageType {
        unsafe {
            from_glib(ffi::g_dbus_message_get_message_type(self.to_glib_none().0))
        }
    }

    fn get_num_unix_fds(&self) -> u32 {
        unsafe {
            ffi::g_dbus_message_get_num_unix_fds(self.to_glib_none().0)
        }
    }

    fn get_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_dbus_message_get_path(self.to_glib_none().0))
        }
    }

    fn get_reply_serial(&self) -> u32 {
        unsafe {
            ffi::g_dbus_message_get_reply_serial(self.to_glib_none().0)
        }
    }

    fn get_sender(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_dbus_message_get_sender(self.to_glib_none().0))
        }
    }

    fn get_serial(&self) -> u32 {
        unsafe {
            ffi::g_dbus_message_get_serial(self.to_glib_none().0)
        }
    }

    fn get_signature(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_dbus_message_get_signature(self.to_glib_none().0))
        }
    }

    //fn get_unix_fd_list(&self) -> /*Ignored*/Option<UnixFDList> {
    //    unsafe { TODO: call ffi::g_dbus_message_get_unix_fd_list() }
    //}

    fn lock(&self) {
        unsafe {
            ffi::g_dbus_message_lock(self.to_glib_none().0);
        }
    }

    //fn new_method_error(&self, error_name: &str, error_message_format: &str, : /*Unknown conversion*//*Unimplemented*/Fundamental: VarArgs) -> Option<DBusMessage> {
    //    unsafe { TODO: call ffi::g_dbus_message_new_method_error() }
    //}

    fn new_method_error_literal(&self, error_name: &str, error_message: &str) -> Option<DBusMessage> {
        unsafe {
            from_glib_full(ffi::g_dbus_message_new_method_error_literal(self.to_glib_none().0, error_name.to_glib_none().0, error_message.to_glib_none().0))
        }
    }

    //fn new_method_error_valist(&self, error_name: &str, error_message_format: &str, var_args: /*Unknown conversion*//*Unimplemented*/Unsupported) -> Option<DBusMessage> {
    //    unsafe { TODO: call ffi::g_dbus_message_new_method_error_valist() }
    //}

    fn new_method_reply(&self) -> Option<DBusMessage> {
        unsafe {
            from_glib_full(ffi::g_dbus_message_new_method_reply(self.to_glib_none().0))
        }
    }

    fn print(&self, indent: u32) -> Option<String> {
        unsafe {
            from_glib_full(ffi::g_dbus_message_print(self.to_glib_none().0, indent))
        }
    }

    fn set_body(&self, body: &glib::Variant) {
        unsafe {
            ffi::g_dbus_message_set_body(self.to_glib_none().0, body.to_glib_none().0);
        }
    }

    fn set_byte_order(&self, byte_order: DBusMessageByteOrder) {
        unsafe {
            ffi::g_dbus_message_set_byte_order(self.to_glib_none().0, byte_order.to_glib());
        }
    }

    fn set_destination(&self, value: &str) {
        unsafe {
            ffi::g_dbus_message_set_destination(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_error_name(&self, value: &str) {
        unsafe {
            ffi::g_dbus_message_set_error_name(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_flags(&self, flags: DBusMessageFlags) {
        unsafe {
            ffi::g_dbus_message_set_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    fn set_header<'a, P: Into<Option<&'a glib::Variant>>>(&self, header_field: DBusMessageHeaderField, value: P) {
        let value = value.into();
        let value = value.to_glib_none();
        unsafe {
            ffi::g_dbus_message_set_header(self.to_glib_none().0, header_field.to_glib(), value.0);
        }
    }

    fn set_interface(&self, value: &str) {
        unsafe {
            ffi::g_dbus_message_set_interface(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_member(&self, value: &str) {
        unsafe {
            ffi::g_dbus_message_set_member(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_message_type(&self, type_: DBusMessageType) {
        unsafe {
            ffi::g_dbus_message_set_message_type(self.to_glib_none().0, type_.to_glib());
        }
    }

    fn set_num_unix_fds(&self, value: u32) {
        unsafe {
            ffi::g_dbus_message_set_num_unix_fds(self.to_glib_none().0, value);
        }
    }

    fn set_path(&self, value: &str) {
        unsafe {
            ffi::g_dbus_message_set_path(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_reply_serial(&self, value: u32) {
        unsafe {
            ffi::g_dbus_message_set_reply_serial(self.to_glib_none().0, value);
        }
    }

    fn set_sender(&self, value: &str) {
        unsafe {
            ffi::g_dbus_message_set_sender(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    fn set_serial(&self, serial: u32) {
        unsafe {
            ffi::g_dbus_message_set_serial(self.to_glib_none().0, serial);
        }
    }

    fn set_signature(&self, value: &str) {
        unsafe {
            ffi::g_dbus_message_set_signature(self.to_glib_none().0, value.to_glib_none().0);
        }
    }

    //fn set_unix_fd_list<'a, P: Into<Option<&'a /*Ignored*/UnixFDList>>>(&self, fd_list: P) {
    //    unsafe { TODO: call ffi::g_dbus_message_set_unix_fd_list() }
    //}

    fn to_blob(&self, capabilities: DBusCapabilityFlags) -> Result<Vec<u8>, Error> {
        unsafe {
            let mut out_size = mem::uninitialized();
            let mut error = ptr::null_mut();
            let ret = ffi::g_dbus_message_to_blob(self.to_glib_none().0, &mut out_size, capabilities.to_glib(), &mut error);
            if error.is_null() { Ok(FromGlibContainer::from_glib_full_num(ret, out_size as usize)) } else { Err(from_glib_full(error)) }
        }
    }

    fn to_gerror(&self) -> Result<(), Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_dbus_message_to_gerror(self.to_glib_none().0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn get_property_locked(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "locked".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
        }
    }

    fn connect_property_locked_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::locked",
                transmute(notify_locked_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_locked_trampoline<P>(this: *mut ffi::GDBusMessage, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<DBusMessage> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&DBusMessage::from_glib_borrow(this).downcast_unchecked())
}
