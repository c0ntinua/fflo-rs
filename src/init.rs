use raylib::prelude::*;
use raylib::core::text::FontLoadEx::*;
use crate::settings::*;
pub fn handle_thread_font(settings : &Settings) -> (RaylibHandle, RaylibThread, Font)  {
    let height = (settings.rows*settings.pixel_height + settings.text_height) as i32;
    let width  = (settings.cols*settings.pixel_width) as i32;
    let (mut handle, thread) = raylib::init().size(width,height).title("ffl0-rs").build();
    let font = handle.load_font_ex(&thread,&settings.font_name,500,Default(0)).unwrap();
    return (handle, thread, font);  
}