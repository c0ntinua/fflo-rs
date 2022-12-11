use crate::settings::*;
use crate::hand::*;

use raylib::core::color::Color;

use rand::random;
use rand::Rng;

#[derive(Clone)]
pub struct Field {
    pub size : (i32,i32),
    //pub cells : [[f64;S];S],
    pub cells : Vec<Vec<f64>>,
}

pub fn field_FROM_settings(settings : &Settings) -> Field {
    let mut cells = vec![vec![0.0f64;S];S];
    for r in 0..settings.size.0 as usize { 
        for c in 0..settings.size.1 as usize { cells[r][c] =random_f64();}}
    Field { size : settings.size, cells }
}

pub fn field_FROM_hand_field(hand : &Hand, field: &Field) -> Field {
    let mut cells = vec![vec![0.0f64;S];S];
    for r  in 0..field.size.0 { 
        for c in 0..field.size.1 {
            cells[r as usize][c as usize] = f64_FROM_hand_field_target(hand, field, (r,c));
        } 
    }
    Field {
        size : field.size,
        cells,
    }
}

pub fn UPDATE_field(field : &mut Field) {
    for r  in 0..field.size.0 { 
        for c in 0..field.size.1 {
            field.cells[r as usize][c as usize] = random_f64();
        } 
    }
}

pub fn f64_FROM_field_target(f : &Field, t : (i32,i32)) -> f64 {
    let u = usize_target_FROM_target_size(t, f.size);
    f.cells[u.0 as usize][u.1 as usize]
}

pub fn f64_FROM_field_utarget(f : &Field, t : (i32,i32)) -> f64 {
    f.cells[t.0 as usize][t.1 as usize]

}

pub fn usize_FROM_i32_i32(x : i32, max : i32) -> i32 {
    if x < 0 {return x + max;}
    if x > max - 1 {return x - max;}
    x
}

pub fn usize_target_FROM_target_size(target : (i32,i32), max : (i32,i32)) -> (i32,i32) {
    (usize_FROM_i32_i32(target.0,max.0),usize_FROM_i32_i32(target.1,max.1))
}



pub fn u8_FROM_f64(x : f64) -> u8 {
    ((x + 1.0)/SCALE).trunc() as u8
}

pub fn random_f64() -> f64 {
    1.0 - 2.0*rand::random::<f64>()
}
