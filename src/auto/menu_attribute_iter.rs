// This file was generated by gir (https://github.com/gtk-rs/gir @ 8b9d0bb)
// from gir-files (https://github.com/gtk-rs/gir-files @ 77d1f70)
// DO NOT EDIT

use ffi;
use glib;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct MenuAttributeIter(Object<ffi::GMenuAttributeIter, ffi::GMenuAttributeIterClass>);

    match fn {
        get_type => || ffi::g_menu_attribute_iter_get_type(),
    }
}

pub trait MenuAttributeIterExt {
    fn get_name(&self) -> Option<String>;

    fn get_next(&self) -> Option<(String, glib::Variant)>;

    fn get_value(&self) -> Option<glib::Variant>;

    fn next(&self) -> bool;
}

impl<O: IsA<MenuAttributeIter>> MenuAttributeIterExt for O {
    fn get_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::g_menu_attribute_iter_get_name(self.to_glib_none().0))
        }
    }

    fn get_next(&self) -> Option<(String, glib::Variant)> {
        unsafe {
            let mut out_name = ptr::null();
            let mut value = ptr::null_mut();
            let ret = from_glib(ffi::g_menu_attribute_iter_get_next(self.to_glib_none().0, &mut out_name, &mut value));
            if ret { Some((from_glib_none(out_name), from_glib_full(value))) } else { None }
        }
    }

    fn get_value(&self) -> Option<glib::Variant> {
        unsafe {
            from_glib_full(ffi::g_menu_attribute_iter_get_value(self.to_glib_none().0))
        }
    }

    fn next(&self) -> bool {
        unsafe {
            from_glib(ffi::g_menu_attribute_iter_next(self.to_glib_none().0))
        }
    }
}
