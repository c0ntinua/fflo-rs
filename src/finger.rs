use rand::Rng;
use crate::field::*;
#[derive(Debug,Clone)]
pub struct Finger {
    pub target : (i32,i32),
    pub action : f64,
}

pub fn finger_FROM_size_f64(size : (i32,i32),  pow : f64) -> Finger {
    Finger { target : target_FROM_size(size) , action : random_f64_FROM_f64(pow),}
}

pub fn random_f64_FROM_f64( max : f64) -> f64 {
    let mut rng = rand::thread_rng();
    (1.0 - 2.0*rand::random::<f64>())*max
}

pub fn target_FROM_size(size : (i32,i32)) -> (i32,i32) {
    let mut rng = rand::thread_rng();
    (rng.gen_range(0..size.0 as usize) as i32,rng.gen_range(0..size.1 as usize) as i32)
}

pub fn f64_FROM_finger_field_target(finger : &Finger, field : &Field, target : (i32,i32)) -> f64 {
    finger.action*f64_FROM_field_target(field, target)
}

pub fn box_fingers_FROM_size_f64(size : (i32,i32), pow : f64) -> Vec<Finger> {
    assert!(size.0 % 2 == 1 && size.1 % 2 == 1);
    let mut fingers = vec!();
    let span = (size.0/2, size.1/2);
    for r in -span.0..=span.0 {
        for c in -span.1..=span.1 {
            let target = (r,c);
            let action = random_f64_FROM_f64(pow);
            fingers.push(Finger { target, action});
        }
    } 
    fingers
}


