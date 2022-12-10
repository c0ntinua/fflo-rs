use crate::settings::*;
use crate::field::*;
use raylib::prelude::*;
use raylib::core::color::Color;
use raylib::core::text::Font;

#[derive(Clone)]
pub struct Canvas {
    pub size : (i32,i32),
    pub pixel_size : (i32,i32),
    pub colors : [[Color;S];S],
}
pub fn canvas_FROM_settings(settings : &Settings) -> Canvas {
    Canvas {
        size : settings.size,
        pixel_size : settings.pixel_size,
        colors : [[Color { r: 0, g:0, b:0,a:255}; S];S],
    }
}

// pub fn canvas_FROM_field_canvas( field : &Field, canvas : &Canvas ) -> Canvas {
//     let mut colors = [[Color { r: 0, g:0, b:0,a:0}; S];S];
//     for r in 0..field.size.0 {
//         for c in 0..field.size.1 { 
//             colors[r][c] = color_FROM_f64(field[r][c]);
//         }
//     }
//     Canvas {  
//         size : field.size,
//         pixel_size : canvas.pixel_size,
//         colors,
//     }
// }
pub fn UPDATE_canvas_WITH_field(canvas : &mut Canvas, field : &Field) {
    for r in 0..field.size.0 as usize {
        for c in 0..field.size.1 as usize { 
            canvas.colors[r][c] = color_FROM_f64(field.cells[r][c]);
        }
    }
}


