
use crate::layer::*;
use crate::global;
pub struct Buffer {
    pub cells : Vec<u32>,
}
pub fn new_buffer() -> Buffer {
    Buffer {
        cells : vec![0u32;(global::rows*global::pixel_height * global::cols*global::pixel_width) as usize],
    }  
}

impl Buffer {
    pub fn add_abstract_pixels(&mut self, layers : &Vec<Layer>) {
        let mut grayness = 0u32;
        let mut index = 0u32;
        let mut sum = 0f64;
        let mut hue_u32 = 0u32;
        let num_layers_float = layers.len() as f64;
        for row in 0..global::rows {
            for col in 0..global::cols {
                sum = 0.0;
                index = row*global::cols + col;
                for layer in layers.iter() {
                    sum += layer.cells[index as usize];
                }
                grayness = f64_to_u32(sum/num_layers_float);
                hue_u32 = (grayness << 16) | (grayness << 8) | grayness;
                self.add_abstract_pixel(row, col, hue_u32);
            }
        }
    }
    pub fn add_abstract_pixel(&mut self, row : u32, col : u32, grayness : u32) {
        let start_row = row*global::pixel_height;
        let start_col = col*global::pixel_width;
        let buffer_cols = global::cols*global::pixel_width;
        //let buffer_rows = global::rows*global::pixel_height;
        let mut index = 0usize;
        let rgb_code = (grayness << 16) | (grayness << 8) | grayness;
        for sub_row in 0u32..global::pixel_height {
            for sub_col in 0u32..global::pixel_width {
                index = ((start_row + sub_row)*buffer_cols + start_col + sub_col) as usize;
                self.cells[index] = rgb_code;
            }
        }
    }
    pub fn add_color_pixels(&mut self, layers : &Vec<Layer>) {
        let mut grayness = 0u32;
        let mut index = 0u32;
        let mut rgb_sums = [0f64;3];
        let mut r = 0u32;
        let mut g = 0u32;
        let mut b = 0u32;
        let mut hue_u32 = 0u32;
        let num_layers_float = layers.len() as f64;
        for row in 0..global::rows {
            for col in 0..global::cols {
                rgb_sums = [0f64;3];
                index = row*global::cols + col;
                for (l, layer) in layers.iter().enumerate() {
                    rgb_sums[l%3] += layer.cells[index as usize];
                }
                r = f64_to_u32(rgb_sums[0]/num_layers_float);
                g = f64_to_u32(rgb_sums[1]/num_layers_float);
                b = f64_to_u32(rgb_sums[2]/num_layers_float);
                hue_u32 = (r << 16) | (g << 8) | b;
                self.add_color_pixel(row, col, hue_u32);
            }
        }
    }
    pub fn add_color_pixel(&mut self, row : u32, col : u32, rgb_code : u32) {
        let start_row = row*global::pixel_height;
        let start_col = col*global::pixel_width;
        let buffer_cols = global::cols*global::pixel_width;
        let mut index = 0usize;
        for sub_row in 0u32..global::pixel_height {
            for sub_col in 0u32..global::pixel_width {
                index = ((start_row + sub_row)*buffer_cols + start_col + sub_col) as usize;
                self.cells[index] = rgb_code;
            }
        }
    }
}



pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
pub fn f64_to_u32(x : f64) -> u32 {
    ((x+1.0)/global::scale).trunc() as u32
}

