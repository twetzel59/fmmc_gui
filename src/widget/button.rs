use sfml::graphics::{Color, RectangleShape, RenderTarget, Shape, Transformable};
use sfml::system::Vector2f;
use super::{SignalManager, Widget, WidgetExt};

pub struct Button<'s> {
    rect: RectangleShape<'s>,
    signals: SignalManager,
}

impl<'s> Button<'s> {
    pub fn new() -> Button<'s> {
        let mut rect = RectangleShape::with_size(Vector2f::new(100., 50.));
        rect.set_position((100., 75.));
        rect.set_fill_color(&Color::rgb(50, 100, 210));

        Button {
            rect,
            signals: SignalManager::new(),
        }
    }
}

impl<'s> Widget<'s> for Button<'s> {
    fn draw(&self, target: &mut RenderTarget) {
        target.draw(&self.rect);
    }

    fn signal_manager(&mut self) -> &mut SignalManager {
        &mut self.signals
    }

    fn contains_point(&self, point: Vector2f) -> bool {
        self.rect.global_bounds().contains(point)
    }
}

impl<'s> WidgetExt<'s> for Button<'s> {}
