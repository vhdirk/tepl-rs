// This file was generated by gir (https://github.com/gtk-rs/gir @ 7f5a2b5)
// from gir-files (https://github.com/gtk-rs/gir-files @ 2b56823)
// DO NOT EDIT

#[cfg(any(feature = "v2_0", feature = "dox"))]
use ActionInfo;
use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct ActionInfoCentralStore(Object<ffi::AmtkActionInfoCentralStore, ffi::AmtkActionInfoCentralStoreClass>);

    match fn {
        get_type => || ffi::amtk_action_info_central_store_get_type(),
    }
}

impl ActionInfoCentralStore {
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn get_singleton() -> Option<ActionInfoCentralStore> {
        unsafe {
            from_glib_none(ffi::amtk_action_info_central_store_get_singleton())
        }
    }
}

pub trait ActionInfoCentralStoreExt {
    #[cfg(any(feature = "v2_0", feature = "dox"))]
    fn lookup(&self, action_name: &str) -> Option<ActionInfo>;
}

impl<O: IsA<ActionInfoCentralStore>> ActionInfoCentralStoreExt for O {
    #[cfg(any(feature = "v2_0", feature = "dox"))]
    fn lookup(&self, action_name: &str) -> Option<ActionInfo> {
        unsafe {
            from_glib_none(ffi::amtk_action_info_central_store_lookup(self.to_glib_none().0, action_name.to_glib_none().0))
        }
    }
}
