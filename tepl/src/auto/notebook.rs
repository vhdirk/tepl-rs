// This file was generated by gir (https://github.com/gtk-rs/gir @ 7f5a2b5)
// from gir-files (https://github.com/gtk-rs/gir-files @ 2b56823)
// DO NOT EDIT

use TabGroup;
use ffi;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct Notebook(Object<ffi::TeplNotebook, ffi::TeplNotebookClass>): TabGroup;

    match fn {
        get_type => || ffi::tepl_notebook_get_type(),
    }
}

impl Notebook {
    //#[cfg(any(feature = "v3_0", feature = "dox"))]
    //pub fn new() -> Notebook {
    //    unsafe { TODO: call ffi::tepl_notebook_new() }
    //}
}

#[cfg(any(feature = "v3_0", feature = "dox"))]
impl Default for Notebook {
    fn default() -> Self {
        Self::new()
    }
}
