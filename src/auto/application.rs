// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use ActionGroup;
use ActionMap;
use ApplicationCommandLine;
use ApplicationFlags;
use Cancellable;
use Error;
use File;
#[cfg(any(feature = "v2_40", feature = "dox"))]
use Notification;
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
use libc;
use std::boxed::Box as Box_;
use std::mem;
use std::mem::transmute;
use std::ptr;

glib_wrapper! {
    pub struct Application(Object<ffi::GApplication, ffi::GApplicationClass>): ActionGroup, ActionMap;

    match fn {
        get_type => || ffi::g_application_get_type(),
    }
}

impl Application {
    pub fn new<'a, P: Into<Option<&'a str>>>(application_id: P, flags: ApplicationFlags) -> Application {
        let application_id = application_id.into();
        let application_id = application_id.to_glib_none();
        unsafe {
            from_glib_full(ffi::g_application_new(application_id.0, flags.to_glib()))
        }
    }

    pub fn get_default() -> Option<Application> {
        unsafe {
            from_glib_none(ffi::g_application_get_default())
        }
    }

    pub fn id_is_valid(application_id: &str) -> bool {
        unsafe {
            from_glib(ffi::g_application_id_is_valid(application_id.to_glib_none().0))
        }
    }
}

pub trait ApplicationExt {
    fn activate(&self);

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn add_main_option<'a, P: Into<Option<&'a str>>>(&self, long_name: &str, short_name: glib::Char, flags: glib::OptionFlags, arg: glib::OptionArg, description: &str, arg_description: P);

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn add_main_option_entries(&self, entries: /*Ignored*/&[&glib::OptionEntry]);

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn add_option_group(&self, group: /*Ignored*/&glib::OptionGroup);

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn bind_busy_property<P: IsA<glib::Object>>(&self, object: &P, property: &str);

    fn get_application_id(&self) -> Option<String>;

    //#[cfg(any(feature = "v2_34", feature = "dox"))]
    //fn get_dbus_connection(&self) -> /*Ignored*/Option<DBusConnection>;

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_dbus_object_path(&self) -> Option<String>;

    fn get_flags(&self) -> ApplicationFlags;

    fn get_inactivity_timeout(&self) -> u32;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn get_is_busy(&self) -> bool;

    fn get_is_registered(&self) -> bool;

    fn get_is_remote(&self) -> bool;

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn get_resource_base_path(&self) -> Option<String>;

    fn hold(&self);

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn mark_busy(&self);

    fn open(&self, files: &[File], hint: &str);

    fn quit(&self);

    fn register<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error>;

    fn release(&self);

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn send_notification<'a, P: Into<Option<&'a str>>>(&self, id: P, notification: &Notification);

    #[deprecated]
    fn set_action_group<'a, P: IsA<ActionGroup> + 'a, Q: Into<Option<&'a P>>>(&self, action_group: Q);

    fn set_application_id<'a, P: Into<Option<&'a str>>>(&self, application_id: P);

    fn set_default(&self);

    fn set_flags(&self, flags: ApplicationFlags);

    fn set_inactivity_timeout(&self, inactivity_timeout: u32);

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn set_option_context_description<'a, P: Into<Option<&'a str>>>(&self, description: P);

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn set_option_context_parameter_string<'a, P: Into<Option<&'a str>>>(&self, parameter_string: P);

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn set_option_context_summary<'a, P: Into<Option<&'a str>>>(&self, summary: P);

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn set_resource_base_path<'a, P: Into<Option<&'a str>>>(&self, resource_path: P);

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn unbind_busy_property<P: IsA<glib::Object>>(&self, object: &P, property: &str);

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn unmark_busy(&self);

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn withdraw_notification(&self, id: &str);

    fn get_property_resource_base_path(&self) -> Option<String>;

    fn set_property_resource_base_path(&self, resource_base_path: Option<&str>);

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_command_line<F: Fn(&Self, &ApplicationCommandLine) -> i32 + 'static>(&self, f: F) -> SignalHandlerId;

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn connect_handle_local_options<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId;

    fn connect_shutdown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_startup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_action_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_application_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_inactivity_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn connect_property_is_busy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_is_registered_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_is_remote_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_resource_base_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<Application> + IsA<glib::object::Object>> ApplicationExt for O {
    fn activate(&self) {
        unsafe {
            ffi::g_application_activate(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn add_main_option<'a, P: Into<Option<&'a str>>>(&self, long_name: &str, short_name: glib::Char, flags: glib::OptionFlags, arg: glib::OptionArg, description: &str, arg_description: P) {
        let arg_description = arg_description.into();
        let arg_description = arg_description.to_glib_none();
        unsafe {
            ffi::g_application_add_main_option(self.to_glib_none().0, long_name.to_glib_none().0, short_name.to_glib(), flags.to_glib(), arg.to_glib(), description.to_glib_none().0, arg_description.0);
        }
    }

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn add_main_option_entries(&self, entries: /*Ignored*/&[&glib::OptionEntry]) {
    //    unsafe { TODO: call ffi::g_application_add_main_option_entries() }
    //}

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn add_option_group(&self, group: /*Ignored*/&glib::OptionGroup) {
    //    unsafe { TODO: call ffi::g_application_add_option_group() }
    //}

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn bind_busy_property<P: IsA<glib::Object>>(&self, object: &P, property: &str) {
        unsafe {
            ffi::g_application_bind_busy_property(self.to_glib_none().0, object.to_glib_none().0, property.to_glib_none().0);
        }
    }

    fn get_application_id(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_application_get_application_id(self.to_glib_none().0))
        }
    }

    //#[cfg(any(feature = "v2_34", feature = "dox"))]
    //fn get_dbus_connection(&self) -> /*Ignored*/Option<DBusConnection> {
    //    unsafe { TODO: call ffi::g_application_get_dbus_connection() }
    //}

    #[cfg(any(feature = "v2_34", feature = "dox"))]
    fn get_dbus_object_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_application_get_dbus_object_path(self.to_glib_none().0))
        }
    }

    fn get_flags(&self) -> ApplicationFlags {
        unsafe {
            from_glib(ffi::g_application_get_flags(self.to_glib_none().0))
        }
    }

    fn get_inactivity_timeout(&self) -> u32 {
        unsafe {
            ffi::g_application_get_inactivity_timeout(self.to_glib_none().0)
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn get_is_busy(&self) -> bool {
        unsafe {
            from_glib(ffi::g_application_get_is_busy(self.to_glib_none().0))
        }
    }

    fn get_is_registered(&self) -> bool {
        unsafe {
            from_glib(ffi::g_application_get_is_registered(self.to_glib_none().0))
        }
    }

    fn get_is_remote(&self) -> bool {
        unsafe {
            from_glib(ffi::g_application_get_is_remote(self.to_glib_none().0))
        }
    }

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn get_resource_base_path(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_application_get_resource_base_path(self.to_glib_none().0))
        }
    }

    fn hold(&self) {
        unsafe {
            ffi::g_application_hold(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn mark_busy(&self) {
        unsafe {
            ffi::g_application_mark_busy(self.to_glib_none().0);
        }
    }

    fn open(&self, files: &[File], hint: &str) {
        let n_files = files.len() as i32;
        unsafe {
            ffi::g_application_open(self.to_glib_none().0, files.to_glib_none().0, n_files, hint.to_glib_none().0);
        }
    }

    fn quit(&self) {
        unsafe {
            ffi::g_application_quit(self.to_glib_none().0);
        }
    }

    fn register<'a, P: Into<Option<&'a Cancellable>>>(&self, cancellable: P) -> Result<(), Error> {
        let cancellable = cancellable.into();
        let cancellable = cancellable.to_glib_none();
        unsafe {
            let mut error = ptr::null_mut();
            let _ = ffi::g_application_register(self.to_glib_none().0, cancellable.0, &mut error);
            if error.is_null() { Ok(()) } else { Err(from_glib_full(error)) }
        }
    }

    fn release(&self) {
        unsafe {
            ffi::g_application_release(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn send_notification<'a, P: Into<Option<&'a str>>>(&self, id: P, notification: &Notification) {
        let id = id.into();
        let id = id.to_glib_none();
        unsafe {
            ffi::g_application_send_notification(self.to_glib_none().0, id.0, notification.to_glib_none().0);
        }
    }

    fn set_action_group<'a, P: IsA<ActionGroup> + 'a, Q: Into<Option<&'a P>>>(&self, action_group: Q) {
        let action_group = action_group.into();
        let action_group = action_group.to_glib_none();
        unsafe {
            ffi::g_application_set_action_group(self.to_glib_none().0, action_group.0);
        }
    }

    fn set_application_id<'a, P: Into<Option<&'a str>>>(&self, application_id: P) {
        let application_id = application_id.into();
        let application_id = application_id.to_glib_none();
        unsafe {
            ffi::g_application_set_application_id(self.to_glib_none().0, application_id.0);
        }
    }

    fn set_default(&self) {
        unsafe {
            ffi::g_application_set_default(self.to_glib_none().0);
        }
    }

    fn set_flags(&self, flags: ApplicationFlags) {
        unsafe {
            ffi::g_application_set_flags(self.to_glib_none().0, flags.to_glib());
        }
    }

    fn set_inactivity_timeout(&self, inactivity_timeout: u32) {
        unsafe {
            ffi::g_application_set_inactivity_timeout(self.to_glib_none().0, inactivity_timeout);
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn set_option_context_description<'a, P: Into<Option<&'a str>>>(&self, description: P) {
        let description = description.into();
        let description = description.to_glib_none();
        unsafe {
            ffi::g_application_set_option_context_description(self.to_glib_none().0, description.0);
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn set_option_context_parameter_string<'a, P: Into<Option<&'a str>>>(&self, parameter_string: P) {
        let parameter_string = parameter_string.into();
        let parameter_string = parameter_string.to_glib_none();
        unsafe {
            ffi::g_application_set_option_context_parameter_string(self.to_glib_none().0, parameter_string.0);
        }
    }

    #[cfg(any(feature = "v2_56", feature = "dox"))]
    fn set_option_context_summary<'a, P: Into<Option<&'a str>>>(&self, summary: P) {
        let summary = summary.into();
        let summary = summary.to_glib_none();
        unsafe {
            ffi::g_application_set_option_context_summary(self.to_glib_none().0, summary.0);
        }
    }

    #[cfg(any(feature = "v2_42", feature = "dox"))]
    fn set_resource_base_path<'a, P: Into<Option<&'a str>>>(&self, resource_path: P) {
        let resource_path = resource_path.into();
        let resource_path = resource_path.to_glib_none();
        unsafe {
            ffi::g_application_set_resource_base_path(self.to_glib_none().0, resource_path.0);
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn unbind_busy_property<P: IsA<glib::Object>>(&self, object: &P, property: &str) {
        unsafe {
            ffi::g_application_unbind_busy_property(self.to_glib_none().0, object.to_glib_none().0, property.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_38", feature = "dox"))]
    fn unmark_busy(&self) {
        unsafe {
            ffi::g_application_unmark_busy(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_40", feature = "dox"))]
    fn withdraw_notification(&self, id: &str) {
        unsafe {
            ffi::g_application_withdraw_notification(self.to_glib_none().0, id.to_glib_none().0);
        }
    }

    fn get_property_resource_base_path(&self) -> Option<String> {
        unsafe {
            let mut value = Value::from_type(<String as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "resource-base-path".to_glib_none().0, value.to_glib_none_mut().0);
            value.get()
        }
    }

    fn set_property_resource_base_path(&self, resource_base_path: Option<&str>) {
        unsafe {
            gobject_ffi::g_object_set_property(self.to_glib_none().0, "resource-base-path".to_glib_none().0, Value::from(resource_base_path).to_glib_none().0);
        }
    }

    fn connect_activate<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "activate",
                transmute(activate_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_command_line<F: Fn(&Self, &ApplicationCommandLine) -> i32 + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self, &ApplicationCommandLine) -> i32 + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "command-line",
                transmute(command_line_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    //#[cfg(any(feature = "v2_40", feature = "dox"))]
    //fn connect_handle_local_options<Unsupported or ignored types>(&self, f: F) -> SignalHandlerId {
    //    Ignored options: GLib.VariantDict
    //}

    fn connect_shutdown<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "shutdown",
                transmute(shutdown_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_startup<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "startup",
                transmute(startup_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_action_group_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::action-group",
                transmute(notify_action_group_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_application_id_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::application-id",
                transmute(notify_application_id_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::flags",
                transmute(notify_flags_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_inactivity_timeout_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::inactivity-timeout",
                transmute(notify_inactivity_timeout_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(any(feature = "v2_44", feature = "dox"))]
    fn connect_property_is_busy_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::is-busy",
                transmute(notify_is_busy_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_is_registered_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::is-registered",
                transmute(notify_is_registered_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_is_remote_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::is-remote",
                transmute(notify_is_remote_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_resource_base_path_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::resource-base-path",
                transmute(notify_resource_base_path_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn activate_trampoline<P>(this: *mut ffi::GApplication, f: glib_ffi::gpointer)
where P: IsA<Application> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn command_line_trampoline<P>(this: *mut ffi::GApplication, command_line: *mut ffi::GApplicationCommandLine, f: glib_ffi::gpointer) -> libc::c_int
where P: IsA<Application> {
    let f: &&(Fn(&P, &ApplicationCommandLine) -> i32 + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked(), &from_glib_borrow(command_line))
}

unsafe extern "C" fn shutdown_trampoline<P>(this: *mut ffi::GApplication, f: glib_ffi::gpointer)
where P: IsA<Application> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn startup_trampoline<P>(this: *mut ffi::GApplication, f: glib_ffi::gpointer)
where P: IsA<Application> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_action_group_trampoline<P>(this: *mut ffi::GApplication, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Application> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_application_id_trampoline<P>(this: *mut ffi::GApplication, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Application> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_flags_trampoline<P>(this: *mut ffi::GApplication, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Application> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_inactivity_timeout_trampoline<P>(this: *mut ffi::GApplication, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Application> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked())
}

#[cfg(any(feature = "v2_44", feature = "dox"))]
unsafe extern "C" fn notify_is_busy_trampoline<P>(this: *mut ffi::GApplication, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Application> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_is_registered_trampoline<P>(this: *mut ffi::GApplication, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Application> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_is_remote_trampoline<P>(this: *mut ffi::GApplication, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Application> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_resource_base_path_trampoline<P>(this: *mut ffi::GApplication, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<Application> {
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&Application::from_glib_borrow(this).downcast_unchecked())
}
