use crate::field::*;
use crate::filter::*;
use crate::settings::*;
use raylib::prelude::*;
use raylib::core::color::Color;
use raylib::core::text::Font;
use std::fs::File;
pub struct Fflo {
    pub rows : usize,
    pub cols : usize,
    pub pixel_height : usize,
    pub pixel_width : usize,
    pub field : Field,
    pub canvas : Vec<Color>,
    pub y_address_book : Vec<i32>,
    pub x_address_book : Vec<i32>,
    pub filters : Vec<Filter>,
    pub num_rect_filters : usize,
    pub num_gen_filters : usize,
    pub row_span : usize,
    pub col_span : usize,
    pub targets : usize,
    pub rect_mode : bool,
    pub mask_intensity : f64,
    pub filterings : usize,
    pub paused : bool,
    pub flickers : u64,
    pub flicker_counter : u64,
    pub noise_on : bool,
    pub noise_targets : usize,
    pub text_height : usize,
    pub text : String,
    pub font : Font,
    pub file : File,
    pub save_counter : usize,
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
    pub fn replace_filters(&mut self) {
        match self.rect_mode {
            true => self.filters = self.random_filters("rect", self.num_rect_filters),
            false => self.filters = self.random_filters("jagged",self.num_gen_filters),
        }
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
        self.plot_canvas(screen);
        self.plot_text(screen);
    }
    pub fn plot_canvas(&mut self, screen : &mut RaylibDrawHandle) {
        for row in 0..self.rows {
            for col in 0..self.cols {
                self.plot_pixel(screen, row ,col);
            }
        }
    }
    pub fn plot_pixel(&self, screen : &mut RaylibDrawHandle, row : usize, col : usize) {
        // screen.draw_rectangle(
        //     (col*self.pixel_width) as i32,
        //     (row*self.pixel_height) as i32,
        //     self.pixel_width as i32,
        //     self.pixel_height as i32,
        //     self.canvas[col *self.rows + row]
        // );
        screen.draw_rectangle(
            self.x_address_book[col],
            self.y_address_book[row],
            self.pixel_width as i32,
            self.pixel_height as i32,
            self.canvas[col *self.rows + row]
        );
    }
    pub fn flicker(&mut self) {
        if !self.paused && self.flicker_counter >= self.flickers {
            self.apply_filters();
            self.flicker_counter = 0;
        } else {
            self.flicker_counter += 1;
        }
    }
    pub fn plot_text(&mut self, screen : &mut RaylibDrawHandle ) {
        screen.draw_text_ex(
            &self.font,
            &self.text,
            Vector2 {x : 0.0 , y : (self.rows*self.pixel_height) as f32 },
            self.text_height as f32,
            2.0, 
            Color {r : 255, g : 255, b : 255, a : 255}
        );
    }
    fn load_address_books(&mut self) {
        self.y_address_book = vec![];
        for row in 0..self.rows {
            self.y_address_book.push((row * self.pixel_height) as i32);
        }
        self.x_address_book = vec![];
        for col in 0..self.cols {
            self.x_address_book.push((col * self.pixel_width) as i32);
        }
    }
}
fn default_fflo(settings : &Settings, font : Font) -> Fflo {
    let rows = settings.rows as usize;
    let cols = settings.cols as usize;
    let mut file = File::create("foo.txt").unwrap();
    Fflo {
        rows,
        cols,
        pixel_height : settings.pixel_height,
        pixel_width : settings.pixel_width,
        field : random_field(settings.rows,settings.cols),
        canvas : vec![Color {r : 0, g : 0 ,b : 0, a : 255}; rows*cols],
        y_address_book : vec![],
        x_address_book : vec![],
        filters : vec![],
        num_rect_filters : 11,
        num_gen_filters : 15,
        row_span : 9,
        col_span : 9,
        targets : 15,
        rect_mode : true,
        mask_intensity : 8.0,
        filterings : 1,
        paused : false,
        flickers: 0,
        flicker_counter: 0,
        noise_on : false,
        noise_targets : 1000,
        text_height : settings.text_height,
        text: "".to_string(),
        font,
        file,
        save_counter : 0,

    }
}
pub fn new_fflo(settings : &Settings, font : Font) -> Fflo {
    let mut fflo = default_fflo(settings,font);
    fflo.load_address_books();
    fflo.replace_filters();
    fflo
}


