pub use self::button::Button;

pub mod button;

pub trait Widget<'s> {
    fn draw(&self, target: &mut ::sfml::graphics::RenderTarget);
    fn signal_manager(&mut self) -> &mut SignalManager;
    fn contains_point(&self, point: ::sfml::system::Vector2f) -> bool;
}

pub trait WidgetExt<'s>: Widget<'s> {
    fn connect_click<T: FnMut() + 'static>(&mut self, closure: T) {
        self.signal_manager().connect_click(closure);
    }
}

pub struct SignalManager {
    click: Vec<Box<FnMut()>>,
}

impl SignalManager {
    fn new() -> SignalManager {
        SignalManager {
            click: Vec::new(),
        }
    }

    pub fn connect_click<T: FnMut() + 'static>(&mut self, closure: T) {
        self.click.push(Box::new(closure));
    }

    pub fn handle_click(&mut self) {
        for i in &mut self.click {
            i();
        }
    }
}
