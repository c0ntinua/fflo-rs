use crate::settings::*;

use raylib::core::color::Color;

use rand::random;
use rand::Rng;
pub const SCALE : f64 = 1.0/128.0;
#[derive(Clone)]
pub struct Field {
    pub size : (i32,i32),
    pub cells : Vec<f64>,
}

pub fn field_FROM_settings(settings : &Settings) -> Field {
    let mut cells = vec![0.0f64;(settings.size.0*settings.size.1) as usize];
    for cell in cells.iter_mut() {
        *cell = random_f64();
    }
    Field {
        size : settings.size,
        cells
    }
}


pub fn f64_FROM_field_target(f : &Field, t : (i32,i32)) -> f64 {
    let u = usize_target_FROM_target_size(t, f.size);
    f.cells[(u.0*f.size.1 + u.1) as usize]
}
pub fn f64_FROM_field_utarget(f : &Field, t : (i32,i32)) -> f64 {
    f.cells[(t.0*f.size.1 + t.1) as usize]
}

pub fn usize_FROM_i32_i32(x : i32, max : i32) -> i32 {
    if x < 0 {return x + max;}
    if x > max {return x - max;}
    x
}

pub fn usize_target_FROM_target_size(target : (i32,i32), max : (i32,i32)) -> (i32,i32) {
    (usize_FROM_i32_i32(target.0,max.0),usize_FROM_i32_i32(target.1,max.1))
}

pub fn color_FROM_f64(x : f64) -> Color {
    let h = u8_FROM_f64(x);
    Color {r : h, g : h, b : h, a : 255}
}
pub fn color_FROM_f64_f64_f64(x : f64 ,y : f64, z : f64 ) -> Color {
    let r = u8_FROM_f64(x);
    let g = u8_FROM_f64(x);
    let b = u8_FROM_f64(x);
    Color {r , g, b, a : 255 }
}

pub fn u8_FROM_f64(x : f64) -> u8 {
    ((x + 1.0)/SCALE).trunc() as u8
}

pub fn random_f64() -> f64 {
    1.0 - 2.0*rand::random::<f64>()
}
