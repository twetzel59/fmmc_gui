use sfml::graphics::{Color, RenderTarget, RenderWindow};
use sfml::system::Vector2f;
use sfml::window::{ContextSettings, Event, mouse, Style, VideoMode};
use widget::Widget;

const COLOR_DEPTH: u32 = 32;

pub struct AppWindow<'s> {
    win: RenderWindow,
    widgets: Vec<Box<Widget<'s> + 's>>,
}

impl<'s> AppWindow<'s> {
    pub fn new(size: (u32, u32), title: &str, decoration: bool) -> AppWindow<'s> {
        let win_style = if decoration {
            Style::DEFAULT
        } else {
            Style::NONE
        };

        AppWindow {
            win: RenderWindow::new(VideoMode::new(size.0, size.1, COLOR_DEPTH),
                                   title, win_style, &ContextSettings::default()),
            widgets: Vec::new(),
        }
    }

    pub fn add<T: Widget<'s> + 's>(&mut self, widget: T) {
        self.widgets.push(Box::new(widget));
    }

    pub fn start(&mut self) {
        'outer: loop {
            self.win.clear(&Color::BLACK);

            for i in &self.widgets {
                i.draw(&mut self.win);
            }

            self.win.display();

            while let Some(e) = self.win.poll_event() {
                match e {
                    Event::Closed => break 'outer,
                    Event::MouseButtonPressed { button: mouse::Button::Left, x, y } => {
                        for i in &mut self.widgets {
                            if i.contains_point(Vector2f::new(x as f32, y as f32)) {
                                i.signal_manager().handle_click();
                            }
                        }
                    },
                    _ => {},
                }
            }
        }
    }
}
