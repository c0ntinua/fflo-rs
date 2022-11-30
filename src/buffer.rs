use crate::settings::*;
use crate::layer::*;
pub struct Buffer {
    pub rows : u32,
    pub cols : u32,
    pub abstract_rows : u32,
    pub abstract_cols : u32,
    pub abstract_height : u32,
	pub abstract_width : u32,
    pub cells : Vec<u32>,
}

pub fn new_buffer(settings : &Settings) -> Buffer {
    Buffer {
        rows : settings.pixel_rows,
        cols : settings.pixel_cols,
        abstract_rows : settings.abstract_rows,
        abstract_cols : settings.abstract_cols,
        abstract_height : settings.abstract_height,
        abstract_width : settings.abstract_width,
        cells : vec![0u32;(settings.pixel_rows * settings.pixel_cols) as usize],
    }
}

// pub fn paint_layers_on_buffer(layers : Vec<Layer>, buffer : &Buffer) {
//     let mut grayness = 0u32;
//     let mut index = 0u32;
//     let sum = 0f64;
//     let hue_u32 = 0u32;
//     let num_layers_float = layer.len() as f64;
//     for row in 0..buffer.rows {
//         for col in 0..buffer.cols {
//             index = row*buffer.abstract_cols + col;
//             for layer in layers.iter() {
//                 sum += layer.cells[index as usize];
//             }
//             grayness = f64_to_u32(sum/num_layers_float,2.0/256.0);
//             hue_u32 = (grayness << 16) | (grayness << 8) | grayness;
//             add_abstract_pixel(&mut buffer, row, col, hue_u32);
//         }
// }
impl Buffer {
    pub fn add_abstract_pixels(&mut self, layers : &Vec<Layer>) {
        let mut grayness = 0u32;
        let mut index = 0u32;
        let mut sum = 0f64;
        let mut hue_u32 = 0u32;
        let num_layers_float = layers.len() as f64;
        for row in 0..self.abstract_rows {
            for col in 0..self.abstract_cols {
                sum = 0.0;
                index = row*self.abstract_cols + col;
                for layer in layers.iter() {
                    sum += layer.cells[index as usize];
                }
                grayness = f64_to_u32(sum/num_layers_float,2.0/256.0);
                hue_u32 = (grayness << 16) | (grayness << 8) | grayness;
                self.add_abstract_pixel(row, col, hue_u32);
            }
        }
    }

    pub fn add_abstract_pixel(&mut self, abstract_row : u32, abstract_col : u32, hue_code : u32) {
        let start_row = abstract_row*self.abstract_height;
        let start_col = abstract_col*self.abstract_width;
        for sub_row in 0u32..self.abstract_height {
            for sub_col in 0u32..self.abstract_width {
                self.cells[((start_row + sub_row)*self.cols + start_col + sub_col) as usize] = hue_code;
            }
        }
    }
}



pub fn from_u8_rgb(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
pub fn f64_to_u32(x : f64, scale : f64) -> u32 {
    ((x+1.0)/scale).trunc() as u32
}

