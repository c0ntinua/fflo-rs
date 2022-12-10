use crate::field::*;
use crate::settings::*;
use crate::hand::*;
use crate::finger::*;
use raylib::prelude::*;
use raylib::core::color::Color;
use raylib::core::text::Font;
use std::fs::File;
use rand::Rng;
pub struct Fflo {
    pub rows : usize,
    pub cols : usize,
    pub pixel_height : usize,
    pub pixel_width : usize,
    pub field : Field,
    pub canvas : Vec<Color>,
    pub y_address_book : Vec<i32>,
    pub x_address_book : Vec<i32>,
    pub hands : Vec<Hand>,
    pub num_box_hands : usize,
    pub num_gen_hands : usize,
    pub row_span : usize,
    pub col_span : usize,
    pub targets : usize,
    pub rect_mode : bool,
    pub pow : f64,
    pub filterings : usize,
    pub paused : bool,
    pub flickers : u64,
    pub flicker_counter : u64,
    pub noise_on : bool,
    pub noise_targets : usize,
    pub text_height : usize,
    pub text : String,
}
fn default_fflo(settings : &Settings) -> Fflo {
    let rows = settings.rows as usize;
    let cols = settings.cols as usize;
    //let mut file = File::create("foo.txt").unwrap();
    Fflo {
        rows,
        cols,
        pixel_height : settings.pixel_height,
        pixel_width : settings.pixel_width,
        field : random_field(settings.rows,settings.cols),
        canvas : vec![Color {r : 0, g : 0 ,b : 0, a : 255}; rows*cols],
        y_address_book : vec![],
        x_address_book : vec![],
        hands : vec!(),
        num_box_hands : 10,
        num_gen_hands : 5,
        row_span : 7,
        col_span : 7,
        targets : 19,
        rect_mode : true,
        pow : 10.0,
        filterings : 1,
        paused : false,
        flickers: 0,
        flicker_counter: 0,
        noise_on : false,
        noise_targets : 1000,
        text_height : settings.text_height,
        text: "".to_string(),
    }
}
pub fn new_fflo(settings : &Settings) -> Fflo {
    let mut fflo = default_fflo(settings);
    fflo.load_address_books();
    //fflo.load_hands();
    //fflo.load_hands_from_file(&"newer.txt".to_string());
    fflo
}

impl Fflo {
    pub fn wring_field(&mut self) {
        let mut field = self.field.clone();
        for _ in 0..self.filterings {
            for h in self.hands.iter() {
                field = handled_field(&h,&field);
            }
        }
        self.field  = field;
    }
   
    pub fn flicker(&mut self) {
        if !self.paused && self.flicker_counter >= self.flickers {
            //self.apply_filters();
            self.wring_field();
            self.paint_canvas_from_field();
            self.flicker_counter = 0;
        } else {
            self.flicker_counter += 1;
        }
    }
    pub fn plot_text(&mut self, font : Font, screen : &mut RaylibDrawHandle ) {
        screen.draw_text_ex(
            &font,
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




