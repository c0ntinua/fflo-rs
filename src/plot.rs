use crate::canvas::*;
use raylib::prelude::*;
use raylib::core::color::Color;

pub fn plot_WITH_screen_canvas(screen :  &mut RaylibDrawHandle, canvas : &Canvas) {
    screen.clear_background(Color {r : 0, g : 0, b: 0, a: 255});
    let size = (canvas.pixel_size.0, canvas.pixel_size.1);
    for row in 0..canvas.size.0 {
        for col in 0..canvas.size.1 {
            let target = (row * canvas.pixel_size.0, col * canvas.pixel_size.1);
            let color = canvas.colors[row as usize][col as usize];
            plot_pixel_WITH_screen_color_target_size(screen,color,target,size);
        }
    }
}

pub fn plot_pixel_WITH_screen_color_target_size(screen : &mut RaylibDrawHandle , color : Color ,  target : (i32,i32), size : (i32,i32)) {
    screen.draw_rectangle(target.0,target.1,size.0,size.1,color);
}
