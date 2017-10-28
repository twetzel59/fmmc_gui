//! FMMC GUI is a toy project. Don't use it. I won't finish it, I can
//! *almost* promise!

#![deny(missing_docs)]

extern crate sfml;

pub use application::AppWindow;
pub use widget::WidgetExt;

pub mod application;
pub mod widget;
