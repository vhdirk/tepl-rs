// This file was generated by gir (https://github.com/gtk-rs/gir @ 7f5a2b5)
// from gir-files (https://github.com/gtk-rs/gir-files @ 2b56823)
// DO NOT EDIT

use Buffer;
use CompressionType;
use Encoding;
use File;
use NewlineType;
use ffi;
use gio;
use glib;
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
    pub struct FileSaver(Object<ffi::TeplFileSaver, ffi::TeplFileSaverClass>);

    match fn {
        get_type => || ffi::tepl_file_saver_get_type(),
    }
}

impl FileSaver {
    pub fn new(buffer: &Buffer, file: &File) -> FileSaver {
        unsafe {
            from_glib_full(ffi::tepl_file_saver_new(buffer.to_glib_none().0, file.to_glib_none().0))
        }
    }

    pub fn new_with_target<P: IsA<gio::File>>(buffer: &Buffer, file: &File, target_location: &P) -> FileSaver {
        unsafe {
            from_glib_full(ffi::tepl_file_saver_new_with_target(buffer.to_glib_none().0, file.to_glib_none().0, target_location.to_glib_none().0))
        }
    }
}

pub trait FileSaverExt {
    fn get_buffer(&self) -> Option<Buffer>;

    fn get_compression_type(&self) -> CompressionType;

    fn get_encoding(&self) -> Option<Encoding>;

    fn get_file(&self) -> Option<File>;

    //fn get_flags(&self) -> /*Ignored*/FileSaverFlags;

    fn get_location(&self) -> Option<gio::File>;

    fn get_newline_type(&self) -> NewlineType;

    //fn save_async<'a, 'b, 'c, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::FileProgressCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>, S: Into<Option<&'c /*Unimplemented*/glib::DestroyNotify>>, T: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: P, progress_callback: Q, progress_callback_data: R, progress_callback_notify: S, callback: T);

    fn set_compression_type(&self, compression_type: CompressionType);

    fn set_encoding<'a, P: Into<Option<&'a Encoding>>>(&self, encoding: P);

    //fn set_flags(&self, flags: /*Ignored*/FileSaverFlags);

    fn set_newline_type(&self, newline_type: NewlineType);

    fn connect_property_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_compression_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<FileSaver> + IsA<glib::object::Object>> FileSaverExt for O {
    fn get_buffer(&self) -> Option<Buffer> {
        unsafe {
            from_glib_none(ffi::tepl_file_saver_get_buffer(self.to_glib_none().0))
        }
    }

    fn get_compression_type(&self) -> CompressionType {
        unsafe {
            from_glib(ffi::tepl_file_saver_get_compression_type(self.to_glib_none().0))
        }
    }

    fn get_encoding(&self) -> Option<Encoding> {
        unsafe {
            from_glib_none(ffi::tepl_file_saver_get_encoding(self.to_glib_none().0))
        }
    }

    fn get_file(&self) -> Option<File> {
        unsafe {
            from_glib_none(ffi::tepl_file_saver_get_file(self.to_glib_none().0))
        }
    }

    //fn get_flags(&self) -> /*Ignored*/FileSaverFlags {
    //    unsafe { TODO: call ffi::tepl_file_saver_get_flags() }
    //}

    fn get_location(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::tepl_file_saver_get_location(self.to_glib_none().0))
        }
    }

    fn get_newline_type(&self) -> NewlineType {
        unsafe {
            from_glib(ffi::tepl_file_saver_get_newline_type(self.to_glib_none().0))
        }
    }

    //fn save_async<'a, 'b, 'c, P: Into<Option<&'a /*Ignored*/gio::Cancellable>>, Q: Into<Option<&'b /*Ignored*/gio::FileProgressCallback>>, R: Into<Option</*Unimplemented*/Fundamental: Pointer>>, S: Into<Option<&'c /*Unimplemented*/glib::DestroyNotify>>, T: FnOnce(Result<(), Error>) + Send + 'static>(&self, io_priority: glib::Priority, cancellable: P, progress_callback: Q, progress_callback_data: R, progress_callback_notify: S, callback: T) {
    //    unsafe { TODO: call ffi::tepl_file_saver_save_async() }
    //}

    fn set_compression_type(&self, compression_type: CompressionType) {
        unsafe {
            ffi::tepl_file_saver_set_compression_type(self.to_glib_none().0, compression_type.to_glib());
        }
    }

    fn set_encoding<'a, P: Into<Option<&'a Encoding>>>(&self, encoding: P) {
        let encoding = encoding.into();
        let encoding = encoding.to_glib_none();
        unsafe {
            ffi::tepl_file_saver_set_encoding(self.to_glib_none().0, encoding.0);
        }
    }

    //fn set_flags(&self, flags: /*Ignored*/FileSaverFlags) {
    //    unsafe { TODO: call ffi::tepl_file_saver_set_flags() }
    //}

    fn set_newline_type(&self, newline_type: NewlineType) {
        unsafe {
            ffi::tepl_file_saver_set_newline_type(self.to_glib_none().0, newline_type.to_glib());
        }
    }

    fn connect_property_buffer_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::buffer",
                transmute(notify_buffer_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_compression_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::compression-type",
                transmute(notify_compression_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::encoding",
                transmute(notify_encoding_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::file",
                transmute(notify_file_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_flags_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::flags",
                transmute(notify_flags_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::location",
                transmute(notify_location_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::newline-type",
                transmute(notify_newline_type_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_buffer_trampoline<P>(this: *mut ffi::TeplFileSaver, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileSaver> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileSaver::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_compression_type_trampoline<P>(this: *mut ffi::TeplFileSaver, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileSaver> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileSaver::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_encoding_trampoline<P>(this: *mut ffi::TeplFileSaver, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileSaver> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileSaver::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_file_trampoline<P>(this: *mut ffi::TeplFileSaver, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileSaver> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileSaver::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_flags_trampoline<P>(this: *mut ffi::TeplFileSaver, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileSaver> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileSaver::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_location_trampoline<P>(this: *mut ffi::TeplFileSaver, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileSaver> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileSaver::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_newline_type_trampoline<P>(this: *mut ffi::TeplFileSaver, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<FileSaver> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&FileSaver::from_glib_borrow(this).downcast_unchecked())
}