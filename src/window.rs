use crate::minifb::*;
use crate::settings::*;

pub fn new_window(settings : &Settings) -> Window {
    Window::new("fflo-rs", settings.pixel_rows as usize,settings.pixel_cols as usize, WindowOptions::default()).unwrap()
}