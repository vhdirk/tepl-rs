// This file was generated by gir (https://github.com/gtk-rs/gir @ 7f5a2b5)
// from gir-files (https://github.com/gtk-rs/gir-files @ 2b56823)
// DO NOT EDIT

#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]

extern crate libc;
#[macro_use] extern crate bitflags;
extern crate glib_sys as glib;
extern crate gobject_sys as gobject;
extern crate gio_sys as gio;
extern crate gtk_sys as gtk;

#[allow(unused_imports)]
use libc::{c_int, c_char, c_uchar, c_float, c_uint, c_double,
    c_short, c_ushort, c_long, c_ulong,
    c_void, size_t, ssize_t, intptr_t, uintptr_t, time_t, FILE};

#[allow(unused_imports)]
use glib::{gboolean, gconstpointer, gpointer, GType, Volatile};

// Enums
pub type TeplCompressionType = c_int;
pub const TEPL_COMPRESSION_TYPE_NONE: TeplCompressionType = 0;
pub const TEPL_COMPRESSION_TYPE_GZIP: TeplCompressionType = 1;

pub type TeplFileLoaderError = c_int;
pub const TEPL_FILE_LOADER_ERROR_TOO_BIG: TeplFileLoaderError = 0;
pub const TEPL_FILE_LOADER_ERROR_ENCODING_AUTO_DETECTION_FAILED: TeplFileLoaderError = 1;

pub type TeplFileSaverError = c_int;
pub const TEPL_FILE_SAVER_ERROR_INVALID_CHARS: TeplFileSaverError = 0;
pub const TEPL_FILE_SAVER_ERROR_EXTERNALLY_MODIFIED: TeplFileSaverError = 1;

pub type TeplNewlineType = c_int;
pub const TEPL_NEWLINE_TYPE_LF: TeplNewlineType = 0;
pub const TEPL_NEWLINE_TYPE_CR: TeplNewlineType = 1;
pub const TEPL_NEWLINE_TYPE_CR_LF: TeplNewlineType = 2;

pub type TeplSelectionType = c_int;
pub const TEPL_SELECTION_TYPE_NO_SELECTION: TeplSelectionType = 0;
pub const TEPL_SELECTION_TYPE_ON_SAME_LINE: TeplSelectionType = 1;
pub const TEPL_SELECTION_TYPE_MULTIPLE_LINES: TeplSelectionType = 2;

// Flags
bitflags! {
    #[repr(C)]
    pub struct TeplFileSaverFlags: c_uint {
        const NONE = 0;
        const IGNORE_INVALID_CHARS = 1;
        const IGNORE_MODIFICATION_TIME = 2;
        const CREATE_BACKUP = 4;
    }
}
pub const TEPL_FILE_SAVER_FLAGS_NONE: TeplFileSaverFlags = TeplFileSaverFlags::NONE;
pub const TEPL_FILE_SAVER_FLAGS_IGNORE_INVALID_CHARS: TeplFileSaverFlags = TeplFileSaverFlags::IGNORE_INVALID_CHARS;
pub const TEPL_FILE_SAVER_FLAGS_IGNORE_MODIFICATION_TIME: TeplFileSaverFlags = TeplFileSaverFlags::IGNORE_MODIFICATION_TIME;
pub const TEPL_FILE_SAVER_FLAGS_CREATE_BACKUP: TeplFileSaverFlags = TeplFileSaverFlags::CREATE_BACKUP;

bitflags! {
    #[repr(C)]
    pub struct TeplGutterRendererFoldsState: c_uint {
        const NONE = 0;
        const START_FOLDED = 1;
        const START_OPENED = 2;
        const CONTINUE = 4;
        const END = 8;
    }
}
pub const TEPL_GUTTER_RENDERER_FOLDS_STATE_NONE: TeplGutterRendererFoldsState = TeplGutterRendererFoldsState::NONE;
pub const TEPL_GUTTER_RENDERER_FOLDS_STATE_START_FOLDED: TeplGutterRendererFoldsState = TeplGutterRendererFoldsState::START_FOLDED;
pub const TEPL_GUTTER_RENDERER_FOLDS_STATE_START_OPENED: TeplGutterRendererFoldsState = TeplGutterRendererFoldsState::START_OPENED;
pub const TEPL_GUTTER_RENDERER_FOLDS_STATE_CONTINUE: TeplGutterRendererFoldsState = TeplGutterRendererFoldsState::CONTINUE;
pub const TEPL_GUTTER_RENDERER_FOLDS_STATE_END: TeplGutterRendererFoldsState = TeplGutterRendererFoldsState::END;

// Callbacks
pub type TeplMountOperationFactory = Option<unsafe extern "C" fn(*mut TeplFile, gpointer) -> *mut gio::GMountOperation>;

// Records
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplAbstractFactoryClass {
    pub parent_class: gobject::GObjectClass,
    pub create_tab: Option<unsafe extern "C" fn(*mut TeplAbstractFactory) -> *mut TeplTab>,
    pub create_tab_label: Option<unsafe extern "C" fn(*mut TeplAbstractFactory, *mut TeplTab) -> *mut gtk::GtkWidget>,
    pub padding: [gpointer; 12],
}

impl ::std::fmt::Debug for TeplAbstractFactoryClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplAbstractFactoryClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("create_tab", &self.create_tab)
         .field("create_tab_label", &self.create_tab_label)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplApplicationClass {
    pub parent_class: gobject::GObjectClass,
    pub padding: [gpointer; 12],
}

impl ::std::fmt::Debug for TeplApplicationClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplApplicationClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("padding", &self.padding)
         .finish()
    }
}

#[repr(C)]
pub struct TeplApplicationPrivate(c_void);

impl ::std::fmt::Debug for TeplApplicationPrivate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplApplicationPrivate @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplApplicationWindowClass {
    pub parent_class: gobject::GObjectClass,
    pub padding: [gpointer; 12],
}

impl ::std::fmt::Debug for TeplApplicationWindowClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplApplicationWindowClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("padding", &self.padding)
         .finish()
    }
}

#[repr(C)]
pub struct TeplApplicationWindowPrivate(c_void);

impl ::std::fmt::Debug for TeplApplicationWindowPrivate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplApplicationWindowPrivate @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplBufferClass {
    pub parent_class: gtk_source::GtkSourceBufferClass,
    pub tepl_cursor_moved: Option<unsafe extern "C" fn(*mut TeplBuffer)>,
    pub padding: [gpointer; 12],
}

impl ::std::fmt::Debug for TeplBufferClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplBufferClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("tepl_cursor_moved", &self.tepl_cursor_moved)
         .field("padding", &self.padding)
         .finish()
    }
}

#[repr(C)]
pub struct TeplEncoding(c_void);

impl ::std::fmt::Debug for TeplEncoding {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplEncoding @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplFileClass {
    pub parent_class: gobject::GObjectClass,
    pub padding: [gpointer; 12],
}

impl ::std::fmt::Debug for TeplFileClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplFileClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("padding", &self.padding)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplFileLoaderClass {
    pub parent_class: gobject::GObjectClass,
    pub padding: [gpointer; 12],
}

impl ::std::fmt::Debug for TeplFileLoaderClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplFileLoaderClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("padding", &self.padding)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplFileMetadataClass {
    pub parent_class: gobject::GObjectClass,
    pub padding: [gpointer; 12],
}

impl ::std::fmt::Debug for TeplFileMetadataClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplFileMetadataClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("padding", &self.padding)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplFileSaverClass {
    pub parent_class: gobject::GObjectClass,
    pub padding: [gpointer; 10],
}

impl ::std::fmt::Debug for TeplFileSaverClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplFileSaverClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("padding", &self.padding)
         .finish()
    }
}

#[repr(C)]
pub struct TeplFileSaverPrivate(c_void);

impl ::std::fmt::Debug for TeplFileSaverPrivate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplFileSaverPrivate @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplFoldRegionClass {
    pub parent_class: gobject::GObjectClass,
    pub padding: [gpointer; 12],
}

impl ::std::fmt::Debug for TeplFoldRegionClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplFoldRegionClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("padding", &self.padding)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplGutterRendererFoldsClass {
    pub parent_class: gtk_source::GtkSourceGutterRendererClass,
    pub padding: [gpointer; 12],
}

impl ::std::fmt::Debug for TeplGutterRendererFoldsClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplGutterRendererFoldsClass @ {:?}", self as *const _))
         .field("parent_class", &self.parent_class)
         .field("padding", &self.padding)
         .finish()
    }
}

#[repr(C)]
pub struct TeplInfoBarClass {
    _truncated_record_marker: c_void,
    // /*Ignored*/field parent_class has incomplete type
}

impl ::std::fmt::Debug for TeplInfoBarClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplInfoBarClass @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
pub struct TeplNotebookClass {
    _truncated_record_marker: c_void,
    // /*Ignored*/field parent_class has incomplete type
}

impl ::std::fmt::Debug for TeplNotebookClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplNotebookClass @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
pub struct TeplNotebookPrivate(c_void);

impl ::std::fmt::Debug for TeplNotebookPrivate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplNotebookPrivate @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
pub struct TeplTabClass {
    _truncated_record_marker: c_void,
    // /*Ignored*/field parent_class has incomplete type
}

impl ::std::fmt::Debug for TeplTabClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplTabClass @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplTabGroupInterface {
    pub parent_interface: gobject::GTypeInterface,
    pub get_tabs: Option<unsafe extern "C" fn(*mut TeplTabGroup) -> *mut glib::GList>,
    pub get_active_tab: Option<unsafe extern "C" fn(*mut TeplTabGroup) -> *mut TeplTab>,
    pub set_active_tab: Option<unsafe extern "C" fn(*mut TeplTabGroup, *mut TeplTab)>,
    pub append_tab_vfunc: Option<unsafe extern "C" fn(*mut TeplTabGroup, *mut TeplTab)>,
}

impl ::std::fmt::Debug for TeplTabGroupInterface {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplTabGroupInterface @ {:?}", self as *const _))
         .field("parent_interface", &self.parent_interface)
         .field("get_tabs", &self.get_tabs)
         .field("get_active_tab", &self.get_active_tab)
         .field("set_active_tab", &self.set_active_tab)
         .field("append_tab_vfunc", &self.append_tab_vfunc)
         .finish()
    }
}

#[repr(C)]
pub struct TeplTabLabelClass {
    _truncated_record_marker: c_void,
    // /*Ignored*/field parent_class has incomplete type
}

impl ::std::fmt::Debug for TeplTabLabelClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplTabLabelClass @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
pub struct TeplTabLabelPrivate(c_void);

impl ::std::fmt::Debug for TeplTabLabelPrivate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplTabLabelPrivate @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
pub struct TeplTabPrivate(c_void);

impl ::std::fmt::Debug for TeplTabPrivate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplTabPrivate @ {:?}", self as *const _))
         .finish()
    }
}

#[repr(C)]
pub struct TeplViewClass {
    _truncated_record_marker: c_void,
    // /*Ignored*/field parent_class has incomplete type
}

impl ::std::fmt::Debug for TeplViewClass {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplViewClass @ {:?}", self as *const _))
         .finish()
    }
}

// Classes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplAbstractFactory {
    pub parent: gobject::GObject,
}

impl ::std::fmt::Debug for TeplAbstractFactory {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplAbstractFactory @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplApplication {
    pub parent: gobject::GObject,
    pub priv_: *mut TeplApplicationPrivate,
}

impl ::std::fmt::Debug for TeplApplication {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplApplication @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplApplicationWindow {
    pub parent: gobject::GObject,
    pub priv_: *mut TeplApplicationWindowPrivate,
}

impl ::std::fmt::Debug for TeplApplicationWindow {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplApplicationWindow @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplBuffer {
    pub parent_instance: gtk_source::GtkSourceBuffer,
}

impl ::std::fmt::Debug for TeplBuffer {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplBuffer @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplFile {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for TeplFile {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplFile @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplFileLoader {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for TeplFileLoader {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplFileLoader @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplFileMetadata {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for TeplFileMetadata {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplFileMetadata @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplFileSaver {
    pub object: gobject::GObject,
    pub priv_: *mut TeplFileSaverPrivate,
}

impl ::std::fmt::Debug for TeplFileSaver {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplFileSaver @ {:?}", self as *const _))
         .field("object", &self.object)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplFoldRegion {
    pub parent_instance: gobject::GObject,
}

impl ::std::fmt::Debug for TeplFoldRegion {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplFoldRegion @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplGutterRendererFolds {
    pub parent_instance: gtk_source::GtkSourceGutterRenderer,
}

impl ::std::fmt::Debug for TeplGutterRendererFolds {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplGutterRendererFolds @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplInfoBar {
    pub parent_instance: gtk::GtkInfoBar,
}

impl ::std::fmt::Debug for TeplInfoBar {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplInfoBar @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplNotebook {
    pub parent: gtk::GtkNotebook,
    pub priv_: *mut TeplNotebookPrivate,
}

impl ::std::fmt::Debug for TeplNotebook {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplNotebook @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplTab {
    pub parent: gtk::GtkGrid,
    pub priv_: *mut TeplTabPrivate,
}

impl ::std::fmt::Debug for TeplTab {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplTab @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplTabLabel {
    pub parent: gtk::GtkGrid,
    pub priv_: *mut TeplTabLabelPrivate,
}

impl ::std::fmt::Debug for TeplTabLabel {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplTabLabel @ {:?}", self as *const _))
         .field("parent", &self.parent)
         .field("priv_", &self.priv_)
         .finish()
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TeplView {
    pub parent_instance: gtk_source::GtkSourceView,
}

impl ::std::fmt::Debug for TeplView {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        f.debug_struct(&format!("TeplView @ {:?}", self as *const _))
         .field("parent_instance", &self.parent_instance)
         .finish()
    }
}

// Interfaces
#[repr(C)]
pub struct TeplTabGroup(c_void);

impl ::std::fmt::Debug for TeplTabGroup {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        write!(f, "TeplTabGroup @ {:?}", self as *const _)
    }
}


extern "C" {

    //=========================================================================
    // TeplCompressionType
    //=========================================================================
    pub fn tepl_compression_type_get_type() -> GType;

    //=========================================================================
    // TeplFileLoaderError
    //=========================================================================
    pub fn tepl_file_loader_error_get_type() -> GType;
    pub fn tepl_file_loader_error_quark() -> glib::GQuark;

    //=========================================================================
    // TeplFileSaverError
    //=========================================================================
    pub fn tepl_file_saver_error_get_type() -> GType;
    pub fn tepl_file_saver_error_quark() -> glib::GQuark;

    //=========================================================================
    // TeplNewlineType
    //=========================================================================
    pub fn tepl_newline_type_get_type() -> GType;

    //=========================================================================
    // TeplSelectionType
    //=========================================================================
    pub fn tepl_selection_type_get_type() -> GType;

    //=========================================================================
    // TeplFileSaverFlags
    //=========================================================================
    pub fn tepl_file_saver_flags_get_type() -> GType;

    //=========================================================================
    // TeplGutterRendererFoldsState
    //=========================================================================
    pub fn tepl_gutter_renderer_folds_state_get_type() -> GType;

    //=========================================================================
    // TeplEncoding
    //=========================================================================
    pub fn tepl_encoding_get_type() -> GType;
    pub fn tepl_encoding_new(charset: *const c_char) -> *mut TeplEncoding;
    pub fn tepl_encoding_new_from_locale() -> *mut TeplEncoding;
    pub fn tepl_encoding_new_utf8() -> *mut TeplEncoding;
    pub fn tepl_encoding_copy(enc: *const TeplEncoding) -> *mut TeplEncoding;
    pub fn tepl_encoding_equals(enc1: *const TeplEncoding, enc2: *const TeplEncoding) -> gboolean;
    pub fn tepl_encoding_free(enc: *mut TeplEncoding);
    pub fn tepl_encoding_get_charset(enc: *const TeplEncoding) -> *const c_char;
    pub fn tepl_encoding_get_name(enc: *const TeplEncoding) -> *const c_char;
    pub fn tepl_encoding_is_utf8(enc: *const TeplEncoding) -> gboolean;
    pub fn tepl_encoding_to_string(enc: *const TeplEncoding) -> *mut c_char;
    pub fn tepl_encoding_get_all() -> *mut glib::GSList;
    pub fn tepl_encoding_get_default_candidates() -> *mut glib::GSList;

    //=========================================================================
    // TeplAbstractFactory
    //=========================================================================
    pub fn tepl_abstract_factory_get_type() -> GType;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_abstract_factory_get_singleton() -> *mut TeplAbstractFactory;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_abstract_factory_create_tab(factory: *mut TeplAbstractFactory) -> *mut TeplTab;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_abstract_factory_create_tab_label(factory: *mut TeplAbstractFactory, tab: *mut TeplTab) -> *mut gtk::GtkWidget;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_abstract_factory_set_singleton(factory: *mut TeplAbstractFactory);

    //=========================================================================
    // TeplApplication
    //=========================================================================
    pub fn tepl_application_get_type() -> GType;
    pub fn tepl_application_get_default() -> *mut TeplApplication;
    pub fn tepl_application_get_from_gtk_application(gtk_app: *mut gtk::GtkApplication) -> *mut TeplApplication;
    pub fn tepl_application_get_app_action_info_store(tepl_app: *mut TeplApplication) -> *mut amtk::AmtkActionInfoStore;
    pub fn tepl_application_get_application(tepl_app: *mut TeplApplication) -> *mut gtk::GtkApplication;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_application_get_tepl_action_info_store(tepl_app: *mut TeplApplication) -> *mut amtk::AmtkActionInfoStore;
    pub fn tepl_application_open_simple(tepl_app: *mut TeplApplication, file: *mut gio::GFile);

    //=========================================================================
    // TeplApplicationWindow
    //=========================================================================
    pub fn tepl_application_window_get_type() -> GType;
    pub fn tepl_application_window_get_from_gtk_application_window(gtk_window: *mut gtk::GtkApplicationWindow) -> *mut TeplApplicationWindow;
    pub fn tepl_application_window_get_application_window(tepl_window: *mut TeplApplicationWindow) -> *mut gtk::GtkApplicationWindow;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_application_window_set_tab_group(tepl_window: *mut TeplApplicationWindow, tab_group: *mut TeplTabGroup);

    //=========================================================================
    // TeplBuffer
    //=========================================================================
    pub fn tepl_buffer_get_type() -> GType;
    pub fn tepl_buffer_new() -> *mut TeplBuffer;
    pub fn tepl_buffer_get_file(buffer: *mut TeplBuffer) -> *mut TeplFile;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_buffer_get_full_title(buffer: *mut TeplBuffer) -> *mut c_char;
    pub fn tepl_buffer_get_selection_type(buffer: *mut TeplBuffer) -> TeplSelectionType;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_buffer_get_short_title(buffer: *mut TeplBuffer) -> *mut c_char;
    pub fn tepl_buffer_get_style_scheme_id(buffer: *mut TeplBuffer) -> *mut c_char;
    pub fn tepl_buffer_is_untouched(buffer: *mut TeplBuffer) -> gboolean;
    pub fn tepl_buffer_set_style_scheme_id(buffer: *mut TeplBuffer, style_scheme_id: *const c_char);

    //=========================================================================
    // TeplFile
    //=========================================================================
    pub fn tepl_file_get_type() -> GType;
    pub fn tepl_file_new() -> *mut TeplFile;
    pub fn tepl_file_check_file_on_disk(file: *mut TeplFile);
    pub fn tepl_file_get_compression_type(file: *mut TeplFile) -> TeplCompressionType;
    pub fn tepl_file_get_encoding(file: *mut TeplFile) -> *const TeplEncoding;
    pub fn tepl_file_get_file_metadata(file: *mut TeplFile) -> *mut TeplFileMetadata;
    pub fn tepl_file_get_location(file: *mut TeplFile) -> *mut gio::GFile;
    pub fn tepl_file_get_newline_type(file: *mut TeplFile) -> TeplNewlineType;
    pub fn tepl_file_get_short_name(file: *mut TeplFile) -> *const c_char;
    pub fn tepl_file_is_deleted(file: *mut TeplFile) -> gboolean;
    pub fn tepl_file_is_externally_modified(file: *mut TeplFile) -> gboolean;
    pub fn tepl_file_is_local(file: *mut TeplFile) -> gboolean;
    pub fn tepl_file_is_readonly(file: *mut TeplFile) -> gboolean;
    pub fn tepl_file_set_location(file: *mut TeplFile, location: *mut gio::GFile);
    pub fn tepl_file_set_mount_operation_factory(file: *mut TeplFile, callback: TeplMountOperationFactory, user_data: gpointer, notify: glib::GDestroyNotify);

    //=========================================================================
    // TeplFileLoader
    //=========================================================================
    pub fn tepl_file_loader_get_type() -> GType;
    pub fn tepl_file_loader_new(buffer: *mut TeplBuffer, file: *mut TeplFile) -> *mut TeplFileLoader;
    pub fn tepl_file_loader_get_buffer(loader: *mut TeplFileLoader) -> *mut TeplBuffer;
    pub fn tepl_file_loader_get_chunk_size(loader: *mut TeplFileLoader) -> i64;
    pub fn tepl_file_loader_get_encoding(loader: *mut TeplFileLoader) -> *const TeplEncoding;
    pub fn tepl_file_loader_get_file(loader: *mut TeplFileLoader) -> *mut TeplFile;
    pub fn tepl_file_loader_get_location(loader: *mut TeplFileLoader) -> *mut gio::GFile;
    pub fn tepl_file_loader_get_max_size(loader: *mut TeplFileLoader) -> i64;
    pub fn tepl_file_loader_get_newline_type(loader: *mut TeplFileLoader) -> TeplNewlineType;
    pub fn tepl_file_loader_load_async(loader: *mut TeplFileLoader, io_priority: c_int, cancellable: *mut gio::GCancellable, progress_callback: gio::GFileProgressCallback, progress_callback_data: gpointer, progress_callback_notify: glib::GDestroyNotify, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    pub fn tepl_file_loader_load_finish(loader: *mut TeplFileLoader, result: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> gboolean;
    pub fn tepl_file_loader_set_chunk_size(loader: *mut TeplFileLoader, chunk_size: i64);
    pub fn tepl_file_loader_set_max_size(loader: *mut TeplFileLoader, max_size: i64);

    //=========================================================================
    // TeplFileMetadata
    //=========================================================================
    pub fn tepl_file_metadata_get_type() -> GType;
    pub fn tepl_file_metadata_new(file: *mut TeplFile) -> *mut TeplFileMetadata;
    pub fn tepl_file_metadata_get(metadata: *mut TeplFileMetadata, key: *const c_char) -> *mut c_char;
    pub fn tepl_file_metadata_get_file(metadata: *mut TeplFileMetadata) -> *mut TeplFile;
    pub fn tepl_file_metadata_load(metadata: *mut TeplFileMetadata, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> gboolean;
    pub fn tepl_file_metadata_load_async(metadata: *mut TeplFileMetadata, io_priority: c_int, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    pub fn tepl_file_metadata_load_finish(metadata: *mut TeplFileMetadata, result: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> gboolean;
    pub fn tepl_file_metadata_save(metadata: *mut TeplFileMetadata, cancellable: *mut gio::GCancellable, error: *mut *mut glib::GError) -> gboolean;
    pub fn tepl_file_metadata_save_async(metadata: *mut TeplFileMetadata, io_priority: c_int, cancellable: *mut gio::GCancellable, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    pub fn tepl_file_metadata_save_finish(metadata: *mut TeplFileMetadata, result: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> gboolean;
    pub fn tepl_file_metadata_set(metadata: *mut TeplFileMetadata, key: *const c_char, value: *const c_char);

    //=========================================================================
    // TeplFileSaver
    //=========================================================================
    pub fn tepl_file_saver_get_type() -> GType;
    pub fn tepl_file_saver_new(buffer: *mut TeplBuffer, file: *mut TeplFile) -> *mut TeplFileSaver;
    pub fn tepl_file_saver_new_with_target(buffer: *mut TeplBuffer, file: *mut TeplFile, target_location: *mut gio::GFile) -> *mut TeplFileSaver;
    pub fn tepl_file_saver_get_buffer(saver: *mut TeplFileSaver) -> *mut TeplBuffer;
    pub fn tepl_file_saver_get_compression_type(saver: *mut TeplFileSaver) -> TeplCompressionType;
    pub fn tepl_file_saver_get_encoding(saver: *mut TeplFileSaver) -> *const TeplEncoding;
    pub fn tepl_file_saver_get_file(saver: *mut TeplFileSaver) -> *mut TeplFile;
    pub fn tepl_file_saver_get_flags(saver: *mut TeplFileSaver) -> TeplFileSaverFlags;
    pub fn tepl_file_saver_get_location(saver: *mut TeplFileSaver) -> *mut gio::GFile;
    pub fn tepl_file_saver_get_newline_type(saver: *mut TeplFileSaver) -> TeplNewlineType;
    pub fn tepl_file_saver_save_async(saver: *mut TeplFileSaver, io_priority: c_int, cancellable: *mut gio::GCancellable, progress_callback: gio::GFileProgressCallback, progress_callback_data: gpointer, progress_callback_notify: glib::GDestroyNotify, callback: gio::GAsyncReadyCallback, user_data: gpointer);
    pub fn tepl_file_saver_save_finish(saver: *mut TeplFileSaver, result: *mut gio::GAsyncResult, error: *mut *mut glib::GError) -> gboolean;
    pub fn tepl_file_saver_set_compression_type(saver: *mut TeplFileSaver, compression_type: TeplCompressionType);
    pub fn tepl_file_saver_set_encoding(saver: *mut TeplFileSaver, encoding: *const TeplEncoding);
    pub fn tepl_file_saver_set_flags(saver: *mut TeplFileSaver, flags: TeplFileSaverFlags);
    pub fn tepl_file_saver_set_newline_type(saver: *mut TeplFileSaver, newline_type: TeplNewlineType);

    //=========================================================================
    // TeplFoldRegion
    //=========================================================================
    pub fn tepl_fold_region_get_type() -> GType;
    pub fn tepl_fold_region_new(buffer: *mut gtk::GtkTextBuffer, start: *const gtk::GtkTextIter, end: *const gtk::GtkTextIter) -> *mut TeplFoldRegion;
    pub fn tepl_fold_region_get_bounds(fold_region: *mut TeplFoldRegion, start: *mut gtk::GtkTextIter, end: *mut gtk::GtkTextIter) -> gboolean;
    pub fn tepl_fold_region_get_buffer(fold_region: *mut TeplFoldRegion) -> *mut gtk::GtkTextBuffer;
    pub fn tepl_fold_region_get_folded(fold_region: *mut TeplFoldRegion) -> gboolean;
    pub fn tepl_fold_region_set_bounds(fold_region: *mut TeplFoldRegion, start: *const gtk::GtkTextIter, end: *const gtk::GtkTextIter);
    pub fn tepl_fold_region_set_folded(fold_region: *mut TeplFoldRegion, folded: gboolean);

    //=========================================================================
    // TeplGutterRendererFolds
    //=========================================================================
    pub fn tepl_gutter_renderer_folds_get_type() -> GType;
    pub fn tepl_gutter_renderer_folds_new() -> *mut gtk_source::GtkSourceGutterRenderer;
    pub fn tepl_gutter_renderer_folds_set_state(self_: *mut TeplGutterRendererFolds, state: TeplGutterRendererFoldsState);

    //=========================================================================
    // TeplInfoBar
    //=========================================================================
    pub fn tepl_info_bar_get_type() -> GType;
    pub fn tepl_info_bar_new() -> *mut TeplInfoBar;
    pub fn tepl_info_bar_new_simple(msg_type: gtk::GtkMessageType, primary_msg: *const c_char, secondary_msg: *const c_char) -> *mut TeplInfoBar;
    pub fn tepl_info_bar_create_label() -> *mut gtk::GtkLabel;
    pub fn tepl_info_bar_add_close_button(info_bar: *mut TeplInfoBar);
    pub fn tepl_info_bar_add_content_widget(info_bar: *mut TeplInfoBar, content: *mut gtk::GtkWidget);
    pub fn tepl_info_bar_add_icon(info_bar: *mut TeplInfoBar);
    pub fn tepl_info_bar_add_primary_message(info_bar: *mut TeplInfoBar, primary_msg: *const c_char);
    pub fn tepl_info_bar_add_secondary_message(info_bar: *mut TeplInfoBar, secondary_msg: *const c_char);

    //=========================================================================
    // TeplNotebook
    //=========================================================================
    pub fn tepl_notebook_get_type() -> GType;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_notebook_new() -> *mut gtk::GtkWidget;

    //=========================================================================
    // TeplTab
    //=========================================================================
    pub fn tepl_tab_get_type() -> GType;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_tab_new() -> *mut TeplTab;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_tab_new_with_view(view: *mut TeplView) -> *mut TeplTab;
    pub fn tepl_tab_add_info_bar(tab: *mut TeplTab, info_bar: *mut gtk::GtkInfoBar);
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_tab_get_buffer(tab: *mut TeplTab) -> *mut TeplBuffer;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_tab_get_view(tab: *mut TeplTab) -> *mut TeplView;

    //=========================================================================
    // TeplTabLabel
    //=========================================================================
    pub fn tepl_tab_label_get_type() -> GType;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_tab_label_new(tab: *mut TeplTab) -> *mut gtk::GtkWidget;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_tab_label_get_tab(tab_label: *mut TeplTabLabel) -> *mut TeplTab;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_tab_label_update_tooltip(tab_label: *mut TeplTabLabel);

    //=========================================================================
    // TeplView
    //=========================================================================
    pub fn tepl_view_get_type() -> GType;
    pub fn tepl_view_new() -> *mut gtk::GtkWidget;
    pub fn tepl_view_copy_clipboard(view: *mut TeplView);
    pub fn tepl_view_cut_clipboard(view: *mut TeplView);
    pub fn tepl_view_delete_selection(view: *mut TeplView);
    pub fn tepl_view_goto_line(view: *mut TeplView, line: c_int) -> gboolean;
    pub fn tepl_view_goto_line_offset(view: *mut TeplView, line: c_int, line_offset: c_int) -> gboolean;
    pub fn tepl_view_paste_clipboard(view: *mut TeplView);
    pub fn tepl_view_scroll_to_cursor(view: *mut TeplView);
    pub fn tepl_view_select_all(view: *mut TeplView);
    pub fn tepl_view_select_lines(view: *mut TeplView, start_line: c_int, end_line: c_int);

    //=========================================================================
    // TeplTabGroup
    //=========================================================================
    pub fn tepl_tab_group_get_type() -> GType;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_tab_group_append_tab(tab_group: *mut TeplTabGroup, tab: *mut TeplTab, jump_to: gboolean);
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_tab_group_get_active_buffer(tab_group: *mut TeplTabGroup) -> *mut TeplBuffer;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_tab_group_get_active_tab(tab_group: *mut TeplTabGroup) -> *mut TeplTab;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_tab_group_get_active_view(tab_group: *mut TeplTabGroup) -> *mut TeplView;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_tab_group_get_buffers(tab_group: *mut TeplTabGroup) -> *mut glib::GList;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_tab_group_get_tabs(tab_group: *mut TeplTabGroup) -> *mut glib::GList;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_tab_group_get_views(tab_group: *mut TeplTabGroup) -> *mut glib::GList;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_tab_group_set_active_tab(tab_group: *mut TeplTabGroup, tab: *mut TeplTab);

    //=========================================================================
    // Other functions
    //=========================================================================
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_finalize();
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_init();
    pub fn tepl_iter_get_line_indentation(iter: *const gtk::GtkTextIter) -> *mut c_char;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub fn tepl_menu_shell_append_edit_actions(menu_shell: *mut gtk::GtkMenuShell);
    pub fn tepl_metadata_manager_init(metadata_path: *const c_char);
    pub fn tepl_metadata_manager_shutdown();

}
