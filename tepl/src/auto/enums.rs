// This file was generated by gir (https://github.com/gtk-rs/gir @ 7f5a2b5)
// from gir-files (https://github.com/gtk-rs/gir-files @ 2b56823)
// DO NOT EDIT

use ffi;
use glib::StaticType;
use glib::Type;
use glib::translate::*;
use glib::value::FromValue;
use glib::value::FromValueOptional;
use glib::value::SetValue;
use glib::value::Value;
use gobject_ffi;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum CompressionType {
    None,
    Gzip,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for CompressionType {
    type GlibType = ffi::TeplCompressionType;

    fn to_glib(&self) -> ffi::TeplCompressionType {
        match *self {
            CompressionType::None => ffi::TEPL_COMPRESSION_TYPE_NONE,
            CompressionType::Gzip => ffi::TEPL_COMPRESSION_TYPE_GZIP,
            CompressionType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::TeplCompressionType> for CompressionType {
    fn from_glib(value: ffi::TeplCompressionType) -> Self {
        match value {
            0 => CompressionType::None,
            1 => CompressionType::Gzip,
            value => CompressionType::__Unknown(value),
        }
    }
}

impl StaticType for CompressionType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::tepl_compression_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for CompressionType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for CompressionType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for CompressionType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum NewlineType {
    Lf,
    Cr,
    CrLf,
    #[doc(hidden)]
    __Unknown(i32),
}

#[doc(hidden)]
impl ToGlib for NewlineType {
    type GlibType = ffi::TeplNewlineType;

    fn to_glib(&self) -> ffi::TeplNewlineType {
        match *self {
            NewlineType::Lf => ffi::TEPL_NEWLINE_TYPE_LF,
            NewlineType::Cr => ffi::TEPL_NEWLINE_TYPE_CR,
            NewlineType::CrLf => ffi::TEPL_NEWLINE_TYPE_CR_LF,
            NewlineType::__Unknown(value) => value
        }
    }
}

#[doc(hidden)]
impl FromGlib<ffi::TeplNewlineType> for NewlineType {
    fn from_glib(value: ffi::TeplNewlineType) -> Self {
        match value {
            0 => NewlineType::Lf,
            1 => NewlineType::Cr,
            2 => NewlineType::CrLf,
            value => NewlineType::__Unknown(value),
        }
    }
}

impl StaticType for NewlineType {
    fn static_type() -> Type {
        unsafe { from_glib(ffi::tepl_newline_type_get_type()) }
    }
}

impl<'a> FromValueOptional<'a> for NewlineType {
    unsafe fn from_value_optional(value: &Value) -> Option<Self> {
        Some(FromValue::from_value(value))
    }
}

impl<'a> FromValue<'a> for NewlineType {
    unsafe fn from_value(value: &Value) -> Self {
        from_glib(gobject_ffi::g_value_get_enum(value.to_glib_none().0))
    }
}

impl SetValue for NewlineType {
    unsafe fn set_value(value: &mut Value, this: &Self) {
        gobject_ffi::g_value_set_enum(value.to_glib_none_mut().0, this.to_glib())
    }
}

