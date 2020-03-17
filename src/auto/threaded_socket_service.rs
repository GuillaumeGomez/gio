// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use gio_sys;
use glib;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use glib::StaticType;
use glib::Value;
use glib_sys;
use gobject_sys;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;
use SocketConnection;
use SocketListener;
use SocketService;

glib_wrapper! {
    pub struct ThreadedSocketService(Object<gio_sys::GThreadedSocketService, gio_sys::GThreadedSocketServiceClass, ThreadedSocketServiceClass>) @extends SocketService, SocketListener;

    match fn {
        get_type => || gio_sys::g_threaded_socket_service_get_type(),
    }
}

impl ThreadedSocketService {
    pub fn new(max_threads: i32) -> ThreadedSocketService {
        unsafe {
            SocketService::from_glib_full(gio_sys::g_threaded_socket_service_new(max_threads))
                .unsafe_cast()
        }
    }
}

pub const NONE_THREADED_SOCKET_SERVICE: Option<&ThreadedSocketService> = None;

pub trait ThreadedSocketServiceExt: 'static {
    fn get_property_max_threads(&self) -> i32;

    fn connect_run<F: Fn(&Self, &SocketConnection, &glib::Object) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId;
}

impl<O: IsA<ThreadedSocketService>> ThreadedSocketServiceExt for O {
    fn get_property_max_threads(&self) -> i32 {
        unsafe {
            let mut value = Value::from_type(<i32 as StaticType>::static_type());
            gobject_sys::g_object_get_property(
                self.to_glib_none().0 as *mut gobject_sys::GObject,
                b"max-threads\0".as_ptr() as *const _,
                value.to_glib_none_mut().0,
            );
            value
                .get()
                .expect("Return Value for property `max-threads` getter")
                .unwrap()
        }
    }

    fn connect_run<F: Fn(&Self, &SocketConnection, &glib::Object) -> bool + 'static>(
        &self,
        f: F,
    ) -> SignalHandlerId {
        unsafe extern "C" fn run_trampoline<
            P,
            F: Fn(&P, &SocketConnection, &glib::Object) -> bool + 'static,
        >(
            this: *mut gio_sys::GThreadedSocketService,
            connection: *mut gio_sys::GSocketConnection,
            source_object: *mut gobject_sys::GObject,
            f: glib_sys::gpointer,
        ) -> glib_sys::gboolean
        where
            P: IsA<ThreadedSocketService>,
        {
            let f: &F = &*(f as *const F);
            f(
                &ThreadedSocketService::from_glib_borrow(this).unsafe_cast_ref(),
                &from_glib_borrow(connection),
                &from_glib_borrow(source_object),
            )
            .to_glib()
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"run\0".as_ptr() as *const _,
                Some(transmute(run_trampoline::<Self, F> as usize)),
                Box_::into_raw(f),
            )
        }
    }
}

impl fmt::Display for ThreadedSocketService {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ThreadedSocketService")
    }
}
