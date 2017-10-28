//! This module contains traits and structures for all UI widgets.

pub use self::button::Button;

pub mod button;

/// A trait used mostly internally for all widgets.
pub trait Widget<'s> {
    /// Draw the widget to a SFML `RenderTarget`.
    fn draw(&self, target: &mut ::sfml::graphics::RenderTarget);

    /// Return this widget's per-instance `SignalManager`.
    fn signal_manager(&mut self) -> &mut SignalManager;

    /// Check if a widget contains an absolute pixel coordinate from
    /// the top left corner of the app. Needed for click handling.
    fn contains_point(&self, point: ::sfml::system::Vector2f) -> bool;
}

/// The main public-facing shared widget functionality.
pub trait WidgetExt<'s>: Widget<'s> {
    /// Connects a signal dispatched when the widget is clicked.
    fn connect_click<T: FnMut() + 'static>(&mut self, closure: T) {
        self.signal_manager().connect_click(closure);
    }
}

/// Use this to implement a widget's signal system.
pub struct SignalManager {
    click: Vec<Box<FnMut()>>,
}

impl SignalManager {
    /// Creates a `SignalManager` with no attached signals.
    pub fn new() -> SignalManager {
        SignalManager {
            click: Vec::new(),
        }
    }

    /// Connects a signal expressed as a closure.
    pub fn connect_click<T: FnMut() + 'static>(&mut self, closure: T) {
        self.click.push(Box::new(closure));
    }

    /// Dispatches click callbacks.
    pub fn handle_click(&mut self) {
        for i in &mut self.click {
            i();
        }
    }
}
