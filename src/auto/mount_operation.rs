// This file was generated by gir (5a68ad0) from gir-files (469db10)
// DO NOT EDIT

use AskPasswordFlags;
use MountOperationResult;
use PasswordSave;
use ffi;
use glib;
use glib::object::Downcast;
use glib::object::IsA;
use glib::signal::SignalHandlerId;
use glib::signal::connect;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct MountOperation(Object<ffi::GMountOperation, ffi::GMountOperationClass>);

    match fn {
        get_type => || ffi::g_mount_operation_get_type(),
    }
}

impl MountOperation {
    pub fn new() -> MountOperation {
        unsafe {
            from_glib_full(ffi::g_mount_operation_new())
        }
    }
}

impl Default for MountOperation {
    fn default() -> Self {
        Self::new()
    }
}

pub trait MountOperationExt {
    fn get_anonymous(&self) -> bool;

    fn get_choice(&self) -> i32;

    fn get_domain(&self) -> Option<String>;

    fn get_password(&self) -> Option<String>;

    fn get_password_save(&self) -> PasswordSave;

    fn get_username(&self) -> Option<String>;

    fn reply(&self, result: MountOperationResult);

    fn set_anonymous(&self, anonymous: bool);

    fn set_choice(&self, choice: i32);

    fn set_domain(&self, domain: &str);

    fn set_password(&self, password: &str);

    fn set_password_save(&self, save: PasswordSave);

    fn set_username(&self, username: &str);

    fn connect_aborted<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_ask_password<F: Fn(&Self, &str, &str, &str, AskPasswordFlags) + 'static>(&self, f: F) -> SignalHandlerId;

    //fn connect_ask_question<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_reply<F: Fn(&Self, MountOperationResult) + 'static>(&self, f: F) -> SignalHandlerId;

    //fn connect_show_processes<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn connect_show_unmount_progress<F: Fn(&Self, &str, i64, i64) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_anonymous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_choice_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_domain_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_password_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_password_save_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_username_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MountOperation> + IsA<glib::object::Object>> MountOperationExt for O {
    fn get_anonymous(&self) -> bool {
        unsafe {
            from_glib(ffi::g_mount_operation_get_anonymous(self.to_glib_none().0))
        }
    }

    fn get_choice(&self) -> i32 {
        unsafe {
            ffi::g_mount_operation_get_choice(self.to_glib_none().0)
        }
    }

    fn get_domain(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mount_operation_get_domain(self.to_glib_none().0))
        }
    }

    fn get_password(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mount_operation_get_password(self.to_glib_none().0))
        }
    }

    fn get_password_save(&self) -> PasswordSave {
        unsafe {
            from_glib(ffi::g_mount_operation_get_password_save(self.to_glib_none().0))
        }
    }

    fn get_username(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_mount_operation_get_username(self.to_glib_none().0))
        }
    }

    fn reply(&self, result: MountOperationResult) {
        unsafe {
            ffi::g_mount_operation_reply(self.to_glib_none().0, result.to_glib());
        }
    }

    fn set_anonymous(&self, anonymous: bool) {
        unsafe {
            ffi::g_mount_operation_set_anonymous(self.to_glib_none().0, anonymous.to_glib());
        }
    }

    fn set_choice(&self, choice: i32) {
        unsafe {
            ffi::g_mount_operation_set_choice(self.to_glib_none().0, choice);
        }
    }

    fn set_domain(&self, domain: &str) {
        unsafe {
            ffi::g_mount_operation_set_domain(self.to_glib_none().0, domain.to_glib_none().0);
        }
    }

    fn set_password(&self, password: &str) {
        unsafe {
            ffi::g_mount_operation_set_password(self.to_glib_none().0, password.to_glib_none().0);
        }
    }

    fn set_password_save(&self, save: PasswordSave) {
        unsafe {
            ffi::g_mount_operation_set_password_save(self.to_glib_none().0, save.to_glib());
        }
    }

    fn set_username(&self, username: &str) {
        unsafe {
            ffi::g_mount_operation_set_username(self.to_glib_none().0, username.to_glib_none().0);
        }
    }

    fn connect_aborted<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "aborted",
                transmute(aborted_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_ask_password<F: Fn(&Self, &str, &str, &str, AskPasswordFlags) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str, &str, &str, AskPasswordFlags) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "ask-password",
                transmute(ask_password_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //fn connect_ask_question<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype choices: *.CArray TypeId { ns_id: 0, id: 28 }
    //}

    fn connect_reply<F: Fn(&Self, MountOperationResult) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, MountOperationResult) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "reply",
                transmute(reply_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //fn connect_show_processes<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Empty ctype processes: *.Array TypeId { ns_id: 2, id: 3 }
    //    Empty ctype choices: *.CArray TypeId { ns_id: 0, id: 28 }
    //}

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn connect_show_unmount_progress<F: Fn(&Self, &str, i64, i64) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &str, i64, i64) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "show-unmount-progress",
                transmute(show_unmount_progress_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_anonymous_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::anonymous",
                transmute(notify_anonymous_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_choice_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::choice",
                transmute(notify_choice_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_domain_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::domain",
                transmute(notify_domain_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_password_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::password",
                transmute(notify_password_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_password_save_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::password-save",
                transmute(notify_password_save_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_username_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::username",
                transmute(notify_username_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn aborted_trampoline<P>(this: *mut ffi::GMountOperation, f: glib_ffi::gpointer)
where P: IsA<MountOperation> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MountOperation::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn ask_password_trampoline<P>(this: *mut ffi::GMountOperation, message: *mut libc::c_char, default_user: *mut libc::c_char, default_domain: *mut libc::c_char, flags: ffi::GAskPasswordFlags, f: glib_ffi::gpointer)
where P: IsA<MountOperation> {
    callback_guard!();
    let f: &&(Fn(&P, &str, &str, &str, AskPasswordFlags) + 'static) = transmute(f);
    f(&MountOperation::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(message), &String::from_glib_none(default_user), &String::from_glib_none(default_domain), from_glib(flags))
}

unsafe extern "C" fn reply_trampoline<P>(this: *mut ffi::GMountOperation, result: ffi::GMountOperationResult, f: glib_ffi::gpointer)
where P: IsA<MountOperation> {
    callback_guard!();
    let f: &&(Fn(&P, MountOperationResult) + 'static) = transmute(f);
    f(&MountOperation::from_glib_borrow(this).downcast_unchecked(), from_glib(result))
}

#[cfg(any(feature = "v2_34", feature = "dox"))]
unsafe extern "C" fn show_unmount_progress_trampoline<P>(this: *mut ffi::GMountOperation, message: *mut libc::c_char, time_left: i64, bytes_left: i64, f: glib_ffi::gpointer)
where P: IsA<MountOperation> {
    callback_guard!();
    let f: &&(Fn(&P, &str, i64, i64) + 'static) = transmute(f);
    f(&MountOperation::from_glib_borrow(this).downcast_unchecked(), &String::from_glib_none(message), time_left, bytes_left)
}

unsafe extern "C" fn notify_anonymous_trampoline<P>(this: *mut ffi::GMountOperation, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MountOperation> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MountOperation::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_choice_trampoline<P>(this: *mut ffi::GMountOperation, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MountOperation> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MountOperation::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_domain_trampoline<P>(this: *mut ffi::GMountOperation, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MountOperation> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MountOperation::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_password_trampoline<P>(this: *mut ffi::GMountOperation, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MountOperation> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MountOperation::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_password_save_trampoline<P>(this: *mut ffi::GMountOperation, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MountOperation> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MountOperation::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_username_trampoline<P>(this: *mut ffi::GMountOperation, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<MountOperation> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&MountOperation::from_glib_borrow(this).downcast_unchecked())
}
