use crate::minifb::*;
use crate::global;

pub fn new_window() -> Window {
    Window::new(
        "fflo-rs", 
        (global::rows*global::pixel_height) as usize,
        (global::cols*global::pixel_height) as usize, 
        WindowOptions::default()
    ).unwrap()
}

pub fn update_window_with(window : &mut Window, values : &[u32] ) {
    window.update_with_buffer(
        &values,
        (global::rows*global::pixel_height) as usize,
        (global::cols*global::pixel_height) as usize,
    ).ok();
}