// This file was generated by gir (5a68ad0) from gir-files (469db10)
// DO NOT EDIT

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
use std;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct TlsCertificate(Object<ffi::GTlsCertificate, ffi::GTlsCertificateClass>);

    match fn {
        get_type => || ffi::g_tls_certificate_get_type(),
    }
}

impl TlsCertificate {
    pub fn new_from_file<P: AsRef<std::path::Path>>(file: P) -> Result<TlsCertificate, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_new_from_file(file.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn new_from_files<P: AsRef<std::path::Path>, Q: AsRef<std::path::Path>>(cert_file: P, key_file: Q) -> Result<TlsCertificate, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_new_from_files(cert_file.as_ref().to_glib_none().0, key_file.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn new_from_pem(data: &str) -> Result<TlsCertificate, Error> {
        let length = data.len() as isize;
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_new_from_pem(data.to_glib_none().0, length, &mut error);
            if error.is_null() { Ok(from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }

    pub fn list_new_from_file<P: AsRef<std::path::Path>>(file: P) -> Result<Vec<TlsCertificate>, Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_certificate_list_new_from_file(file.as_ref().to_glib_none().0, &mut error);
            if error.is_null() { Ok(FromGlibPtrContainer::from_glib_full(ret)) } else { Err(from_glib_full(error)) }
        }
    }
}

pub trait TlsCertificateExt {
    fn get_issuer(&self) -> Option<TlsCertificate>;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn is_same(&self, cert_two: &TlsCertificate) -> bool;

    //fn verify<'a, 'b, P: IsA</*Ignored*/SocketConnectable> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b TlsCertificate>>>(&self, identity: Q, trusted_ca: R) -> TlsCertificateFlags;

    //fn get_property_certificate(&self) -> /*Ignored*/Option<glib::ByteArray>;

    fn get_property_certificate_pem(&self) -> Option<String>;

    fn connect_property_certificate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_certificate_pem_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_issuer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_private_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_private_key_pem_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<TlsCertificate> + IsA<glib::object::Object>> TlsCertificateExt for O {
    fn get_issuer(&self) -> Option<TlsCertificate> {
        unsafe {
            from_glib_none(ffi::g_tls_certificate_get_issuer(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn is_same(&self, cert_two: &TlsCertificate) -> bool {
        unsafe {
            from_glib(ffi::g_tls_certificate_is_same(self.to_glib_none().0, cert_two.to_glib_none().0))
        }
    }

    //fn verify<'a, 'b, P: IsA</*Ignored*/SocketConnectable> + 'a, Q: Into<Option<&'a P>>, R: Into<Option<&'b TlsCertificate>>>(&self, identity: Q, trusted_ca: R) -> TlsCertificateFlags {
    //    unsafe { TODO: call ffi::g_tls_certificate_verify() }
    //}

    //fn get_property_certificate(&self) -> /*Ignored*/Option<glib::ByteArray> {
    //    unsafe {
    //        let mut value = Value::from_type(</*Unknown type*/ as StaticType>::static_type());
    //        gobject_ffi::g_object_get_property(self.to_glib_none().0, "certificate".to_glib_none().0, value.to_glib_none_mut().0);
    //        value.get()
    //    }
    //}

    fn get_property_certificate_pem(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "certificate-pem".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn connect_property_certificate_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::certificate",
                transmute(notify_certificate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_certificate_pem_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::certificate-pem",
                transmute(notify_certificate_pem_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_issuer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::issuer",
                transmute(notify_issuer_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_private_key_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::private-key",
                transmute(notify_private_key_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_private_key_pem_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::private-key-pem",
                transmute(notify_private_key_pem_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_certificate_trampoline<P>(this: *mut ffi::GTlsCertificate, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsCertificate> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsCertificate::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_certificate_pem_trampoline<P>(this: *mut ffi::GTlsCertificate, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsCertificate> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsCertificate::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_issuer_trampoline<P>(this: *mut ffi::GTlsCertificate, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsCertificate> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsCertificate::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_private_key_trampoline<P>(this: *mut ffi::GTlsCertificate, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsCertificate> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsCertificate::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_private_key_pem_trampoline<P>(this: *mut ffi::GTlsCertificate, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<TlsCertificate> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&TlsCertificate::from_glib_borrow(this).downcast_unchecked())
}
