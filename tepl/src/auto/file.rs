// This file was generated by gir (https://github.com/gtk-rs/gir @ 7f5a2b5)
// from gir-files (https://github.com/gtk-rs/gir-files @ 2b56823)
// DO NOT EDIT

use CompressionType;
use Encoding;
use FileMetadata;
use NewlineType;
use ffi;
use gio;
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
    pub struct File(Object<ffi::TeplFile, ffi::TeplFileClass>);

    match fn {
        get_type => || ffi::tepl_file_get_type(),
    }
}

impl File {
    pub fn new() -> File {
        unsafe {
            from_glib_full(ffi::tepl_file_new())
        }
    }
}

impl Default for File {
    fn default() -> Self {
        Self::new()
    }
}

pub trait FileExt {
    fn check_file_on_disk(&self);

    fn get_compression_type(&self) -> CompressionType;

    fn get_encoding(&self) -> Option<Encoding>;

    fn get_file_metadata(&self) -> Option<FileMetadata>;

    fn get_location(&self) -> Option<gio::File>;

    fn get_newline_type(&self) -> NewlineType;

    fn get_short_name(&self) -> Option<String>;

    fn is_deleted(&self) -> bool;

    fn is_externally_modified(&self) -> bool;

    fn is_local(&self) -> bool;

    fn is_readonly(&self) -> bool;

    fn set_location<'a, P: IsA<gio::File> + 'a, Q: Into<Option<&'a P>>>(&self, location: Q);

    //fn set_mount_operation_factory<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Unimplemented*/glib::DestroyNotify>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/MountOperationFactory, user_data: P, notify: Q);

    fn get_property_read_only(&self) -> bool;

    fn connect_property_compression_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_encoding_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_location_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_newline_type_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;

    fn connect_property_short_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<File> + IsA<glib::object::Object>> FileExt for O {
    fn check_file_on_disk(&self) {
        unsafe {
            ffi::tepl_file_check_file_on_disk(self.to_glib_none().0);
        }
    }

    fn get_compression_type(&self) -> CompressionType {
        unsafe {
            from_glib(ffi::tepl_file_get_compression_type(self.to_glib_none().0))
        }
    }

    fn get_encoding(&self) -> Option<Encoding> {
        unsafe {
            from_glib_none(ffi::tepl_file_get_encoding(self.to_glib_none().0))
        }
    }

    fn get_file_metadata(&self) -> Option<FileMetadata> {
        unsafe {
            from_glib_none(ffi::tepl_file_get_file_metadata(self.to_glib_none().0))
        }
    }

    fn get_location(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::tepl_file_get_location(self.to_glib_none().0))
        }
    }

    fn get_newline_type(&self) -> NewlineType {
        unsafe {
            from_glib(ffi::tepl_file_get_newline_type(self.to_glib_none().0))
        }
    }

    fn get_short_name(&self) -> Option<String> {
        unsafe {
            from_glib_none(ffi::tepl_file_get_short_name(self.to_glib_none().0))
        }
    }

    fn is_deleted(&self) -> bool {
        unsafe {
            from_glib(ffi::tepl_file_is_deleted(self.to_glib_none().0))
        }
    }

    fn is_externally_modified(&self) -> bool {
        unsafe {
            from_glib(ffi::tepl_file_is_externally_modified(self.to_glib_none().0))
        }
    }

    fn is_local(&self) -> bool {
        unsafe {
            from_glib(ffi::tepl_file_is_local(self.to_glib_none().0))
        }
    }

    fn is_readonly(&self) -> bool {
        unsafe {
            from_glib(ffi::tepl_file_is_readonly(self.to_glib_none().0))
        }
    }

    fn set_location<'a, P: IsA<gio::File> + 'a, Q: Into<Option<&'a P>>>(&self, location: Q) {
        let location = location.into();
        let location = location.to_glib_none();
        unsafe {
            ffi::tepl_file_set_location(self.to_glib_none().0, location.0);
        }
    }

    //fn set_mount_operation_factory<'a, P: Into<Option</*Unimplemented*/Fundamental: Pointer>>, Q: Into<Option<&'a /*Unimplemented*/glib::DestroyNotify>>>(&self, callback: /*Unknown conversion*//*Unimplemented*/MountOperationFactory, user_data: P, notify: Q) {
    //    unsafe { TODO: call ffi::tepl_file_set_mount_operation_factory() }
    //}

    fn get_property_read_only(&self) -> bool {
        unsafe {
            let mut value = Value::from_type(<bool as StaticType>::static_type());
            gobject_ffi::g_object_get_property(self.to_glib_none().0, "read-only".to_glib_none().0, value.to_glib_none_mut().0);
            value.get().unwrap()
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

    fn connect_property_read_only_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::read-only",
                transmute(notify_read_only_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }

    fn connect_property_short_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe {
            let f: Box_<Box_<Fn(&Self) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "notify::short-name",
                transmute(notify_short_name_trampoline::<Self> as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

unsafe extern "C" fn notify_compression_type_trampoline<P>(this: *mut ffi::TeplFile, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<File> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&File::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_encoding_trampoline<P>(this: *mut ffi::TeplFile, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<File> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&File::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_location_trampoline<P>(this: *mut ffi::TeplFile, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<File> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&File::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_newline_type_trampoline<P>(this: *mut ffi::TeplFile, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<File> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&File::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_read_only_trampoline<P>(this: *mut ffi::TeplFile, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<File> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&File::from_glib_borrow(this).downcast_unchecked())
}

unsafe extern "C" fn notify_short_name_trampoline<P>(this: *mut ffi::TeplFile, _param_spec: glib_ffi::gpointer, f: glib_ffi::gpointer)
where P: IsA<File> {
    callback_guard!();
    let f: &&(Fn(&P) + 'static) = transmute(f);
    f(&File::from_glib_borrow(this).downcast_unchecked())
}