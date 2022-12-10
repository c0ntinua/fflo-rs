use crate::settings::*;
use crate::field::*;
use raylib::prelude::*;
use raylib::core::color::Color;
use raylib::core::text::Font;

#[derive(Clone)]
pub struct Canvas {
    pub size : (i32,i32),
    pub pixel_size : (i32,i32),
    pub colors : Vec<Color>,
}
pub fn canvas_FROM_settings(settings : &Settings) -> Canvas {
    Canvas {
        size : settings.size,
        pixel_size : settings.pixel_size,
        colors : vec!(),
    }
}

pub fn canvas_FROM_field_canvas( field : &Field, canvas : &Canvas ) -> Canvas {
    let mut colors = vec![];
    for x in field.cells.iter() { colors.push(color_FROM_f64(*x)); }
    Canvas {  
        size : field.size,
        pixel_size : canvas.pixel_size,
        colors,
    }
}

