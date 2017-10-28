extern crate fmmc_gui;

use fmmc_gui::*;

fn main() {
    let mut win = AppWindow::new((800, 500), "Hello", true);

    let mut x = 0;

    let mut btn = widget::Button::new();

    btn.connect_click(|| {
        println!("hello");
    });

    btn.connect_click(move || {
        println!("x: {}", x);
        x += 1;
    });

    win.add(btn);

    win.start();
}
