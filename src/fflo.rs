use crate::field::*;
use crate::filter::*;
use crate::global;
use raylib::prelude::*;
use raylib::core::color::Color;
pub struct Fflo {
    pub rows : usize,
    pub cols : usize,
    pub pixel_height : usize,
    pub pixel_width : usize,
    pub field : Field,
    pub canvas : Vec<Color>,
    pub filters : Vec<Filter>,
    pub filterings : usize,
    pub paused : bool,
    pub delay : u128,
    pub flickers : u64,
}
impl Fflo {
    pub fn apply_filters(&mut self) {
        let mut field_state = self.field.clone();
        for _ in 0..self.filterings {
            for f in self.filters.iter() {
                field_state = f.field_from_filter(&field_state);
            }
        }
        self.field  = field_state;
        self.canvas = self.field.to_monochrome_canvas();
    }
    pub fn plot_canvas(&self, draw_handle : &mut RaylibDrawHandle) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                draw_handle.draw_rectangle(
                    (col*self.pixel_width) as i32,
                    (row*self.pixel_height) as i32,
                    self.pixel_width as i32,
                    self.pixel_height as i32,
                    self.canvas[col *self.rows + row]
                );
            }
        }
    }
}
pub fn default_fflo() -> Fflo {
    let rows = 500usize;
    let cols = 500usize;
    let pixel_height = 1usize;
    let pixel_width = 1usize;
    Fflo {
        rows,
        cols,
        pixel_height,
        pixel_width,
        field : random_field(rows,cols),
        canvas : vec![Color {r : 0, g : 0 ,b : 0, a : 255}; rows*cols as usize],
        filters : random_generalized_filters(),
        filterings : 1,
        paused : false,
        delay : 10000,
        flickers: 100,
    }
}
