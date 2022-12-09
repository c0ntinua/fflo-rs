use crate::fflo::*;
use crate::field::*;
use raylib::prelude::*;
use raylib::core::color::Color;
use raylib::core::text::Font;

impl Fflo {
    pub fn paint_canvas_from_field(&mut self) {
        self.canvas = self.field.to_monochrome_canvas();
    }
    
    pub fn plot(&mut self, screen :  &mut RaylibDrawHandle ) {
        screen.clear_background(Color {r : 0, g : 0, b: 0, a: 255});
        screen.draw_rectangle(
            0i32,
            (self.rows*self.pixel_height) as i32,
            (self.cols*self.pixel_width) as i32,
            self.text_height as i32,
            Color {r : 0, g : 0, b: 0, a: 255}
        );
        //self.plot_canvas(screen);
        self.plot_canvas_circle(screen);
        self.plot_text(screen);
    }    
    
    pub fn plot_canvas(&mut self, screen : &mut RaylibDrawHandle) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.plot_pixel(screen, row ,col);
            }
        }
    }
    pub fn plot_canvas_circle(&mut self, screen : &mut RaylibDrawHandle) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.plot_circle_gradient(screen, row ,col);
            }
        }
    }
    pub fn plot_pixel(&self, screen : &mut RaylibDrawHandle, row : usize, col : usize) {
        screen.draw_rectangle(
            self.x_address_book[col],
            self.y_address_book[row],
            self.pixel_width as i32,
            self.pixel_height as i32,
            self.canvas[col *self.rows + row]
        );
    }
    pub fn plot_circle(&self, screen : &mut RaylibDrawHandle, row : usize, col : usize) {
        screen.draw_circle(
            self.x_address_book[col],
            self.y_address_book[row],
            (self.pixel_height/2) as f32,
            self.canvas[col *self.rows + row]
        );
    } 
    pub fn plot_circle_gradient(&self, screen : &mut RaylibDrawHandle, row : usize, col : usize) {
        screen.draw_circle_gradient(
            self.x_address_book[col],
            self.y_address_book[row],
            (self.pixel_height/2) as f32,
            self.canvas[col *self.rows + row],
            Color {r : 0, g :0, b: 0, a: 255}
        );
    }

}
