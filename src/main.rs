use fltk::{app, prelude::*, window, frame::Frame, button::Button};
use std::process;

fn main() {
    let app = app::App::default();
    let mut wind = window::Window::new(100, 100, 600, 400, "Snake");
    let mut but_start = Button::new(260, 150, 80, 40, "Start Game");
    let mut but_exit = Button::new(260, 250, 80, 40, "Exit");
    let mut frame = Frame::default().with_size(200, 100).center_of(&wind);

    wind.end();
    wind.show();

    but_start.set_callback(move |_| frame.set_label("hello world"));
    but_exit.set_callback(move |_| process::exit(1));
    app.run().unwrap();
}
