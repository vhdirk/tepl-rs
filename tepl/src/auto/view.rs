// This file was generated by gir (https://github.com/gtk-rs/gir @ 7f5a2b5)
// from gir-files (https://github.com/gtk-rs/gir-files @ 2b56823)
// DO NOT EDIT

use ffi;
use glib::object::IsA;
use glib::translate::*;
use glib_ffi;
use gobject_ffi;
use std::mem;
use std::ptr;

glib_wrapper! {
    pub struct View(Object<ffi::TeplView, ffi::TeplViewClass>);

    match fn {
        get_type => || ffi::tepl_view_get_type(),
    }
}

impl View {
    //pub fn new() -> View {
    //    unsafe { TODO: call ffi::tepl_view_new() }
    //}
}

impl Default for View {
    fn default() -> Self {
        Self::new()
    }
}

pub trait ViewExt {
    fn copy_clipboard(&self);

    fn cut_clipboard(&self);

    fn delete_selection(&self);

    #[cfg(any(feature = "v2_0", feature = "dox"))]
    fn goto_line(&self, line: i32) -> bool;

    #[cfg(any(feature = "v2_0", feature = "dox"))]
    fn goto_line_offset(&self, line: i32, line_offset: i32) -> bool;

    fn paste_clipboard(&self);

    fn scroll_to_cursor(&self);

    fn select_all(&self);

    #[cfg(any(feature = "v2_0", feature = "dox"))]
    fn select_lines(&self, start_line: i32, end_line: i32);
}

impl<O: IsA<View>> ViewExt for O {
    fn copy_clipboard(&self) {
        unsafe {
            ffi::tepl_view_copy_clipboard(self.to_glib_none().0);
        }
    }

    fn cut_clipboard(&self) {
        unsafe {
            ffi::tepl_view_cut_clipboard(self.to_glib_none().0);
        }
    }

    fn delete_selection(&self) {
        unsafe {
            ffi::tepl_view_delete_selection(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_0", feature = "dox"))]
    fn goto_line(&self, line: i32) -> bool {
        unsafe {
            from_glib(ffi::tepl_view_goto_line(self.to_glib_none().0, line))
        }
    }

    #[cfg(any(feature = "v2_0", feature = "dox"))]
    fn goto_line_offset(&self, line: i32, line_offset: i32) -> bool {
        unsafe {
            from_glib(ffi::tepl_view_goto_line_offset(self.to_glib_none().0, line, line_offset))
        }
    }

    fn paste_clipboard(&self) {
        unsafe {
            ffi::tepl_view_paste_clipboard(self.to_glib_none().0);
        }
    }

    fn scroll_to_cursor(&self) {
        unsafe {
            ffi::tepl_view_scroll_to_cursor(self.to_glib_none().0);
        }
    }

    fn select_all(&self) {
        unsafe {
            ffi::tepl_view_select_all(self.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v2_0", feature = "dox"))]
    fn select_lines(&self, start_line: i32, end_line: i32) {
        unsafe {
            ffi::tepl_view_select_lines(self.to_glib_none().0, start_line, end_line);
        }
    }
}
