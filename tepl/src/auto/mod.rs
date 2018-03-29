// This file was generated by gir (https://github.com/gtk-rs/gir @ 7f5a2b5)
// from gir-files (https://github.com/gtk-rs/gir-files @ 2b56823)
// DO NOT EDIT

#[cfg(any(feature = "v3_0", feature = "dox"))]
mod abstract_factory;
#[cfg(any(feature = "v3_0", feature = "dox"))]
pub use self::abstract_factory::AbstractFactory;
#[cfg(any(feature = "v3_0", feature = "dox"))]
pub use self::abstract_factory::AbstractFactoryExt;

#[cfg(any(feature = "v2_0", feature = "dox"))]
mod application;
#[cfg(any(feature = "v2_0", feature = "dox"))]
pub use self::application::Application;
#[cfg(any(feature = "v2_0", feature = "dox"))]
pub use self::application::ApplicationExt;

#[cfg(any(feature = "v3_0", feature = "dox"))]
mod application_window;
#[cfg(any(feature = "v3_0", feature = "dox"))]
pub use self::application_window::ApplicationWindow;
#[cfg(any(feature = "v3_0", feature = "dox"))]
pub use self::application_window::ApplicationWindowExt;

mod buffer;
pub use self::buffer::Buffer;
pub use self::buffer::BufferExt;

mod file;
pub use self::file::File;
pub use self::file::FileExt;

mod file_loader;
pub use self::file_loader::FileLoader;
pub use self::file_loader::FileLoaderExt;

mod file_metadata;
pub use self::file_metadata::FileMetadata;
pub use self::file_metadata::FileMetadataExt;

mod file_saver;
pub use self::file_saver::FileSaver;
pub use self::file_saver::FileSaverExt;

mod fold_region;
pub use self::fold_region::FoldRegion;
pub use self::fold_region::FoldRegionExt;

mod gutter_renderer_folds;
pub use self::gutter_renderer_folds::GutterRendererFolds;
pub use self::gutter_renderer_folds::GutterRendererFoldsExt;

mod info_bar;
pub use self::info_bar::InfoBar;
pub use self::info_bar::InfoBarExt;

mod notebook;
pub use self::notebook::Notebook;

#[cfg(any(feature = "v3_0", feature = "dox"))]
mod tab;
#[cfg(any(feature = "v3_0", feature = "dox"))]
pub use self::tab::Tab;
#[cfg(any(feature = "v3_0", feature = "dox"))]
pub use self::tab::TabExt;

#[cfg(any(feature = "v3_0", feature = "dox"))]
mod tab_group;
#[cfg(any(feature = "v3_0", feature = "dox"))]
pub use self::tab_group::TabGroup;
#[cfg(any(feature = "v3_0", feature = "dox"))]
pub use self::tab_group::TabGroupExt;

#[cfg(any(feature = "v3_0", feature = "dox"))]
mod tab_label;
#[cfg(any(feature = "v3_0", feature = "dox"))]
pub use self::tab_label::TabLabel;
#[cfg(any(feature = "v3_0", feature = "dox"))]
pub use self::tab_label::TabLabelExt;

mod view;
pub use self::view::View;
pub use self::view::ViewExt;

#[cfg(any(feature = "v2_0", feature = "dox"))]
mod encoding;
#[cfg(any(feature = "v2_0", feature = "dox"))]
pub use self::encoding::Encoding;

mod enums;
pub use self::enums::CompressionType;
pub use self::enums::NewlineType;

#[doc(hidden)]
pub mod traits {
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub use super::AbstractFactoryExt;
    #[cfg(any(feature = "v2_0", feature = "dox"))]
    pub use super::ApplicationExt;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub use super::ApplicationWindowExt;
    pub use super::BufferExt;
    pub use super::FileExt;
    pub use super::FileLoaderExt;
    pub use super::FileMetadataExt;
    pub use super::FileSaverExt;
    pub use super::FoldRegionExt;
    pub use super::GutterRendererFoldsExt;
    pub use super::InfoBarExt;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub use super::TabExt;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub use super::TabGroupExt;
    #[cfg(any(feature = "v3_0", feature = "dox"))]
    pub use super::TabLabelExt;
    pub use super::ViewExt;
}
