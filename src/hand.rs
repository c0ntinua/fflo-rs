use rand::random;
use rand::Rng;
use libm::tanh;

use crate::field::*;use crate::finger::*;

#[derive(Clone)]
pub struct Hand {
    pub fingers : Vec<Finger>,
}

pub fn fingerless_hand() -> Hand {
    Hand {
        fingers : vec!(),
    }
}

pub fn f64_FROM_hand_field_target(hand : &Hand , field: &Field, t : (i32,i32) ) -> f64 {
    let mut s = 0.0f64;
    for finger in hand.fingers.iter() {
        let next_target = (finger.target.0+t.0,finger.target.1+t.1);
        s += f64_FROM_finger_field_target(finger, field, next_target);
    }
    tanh(s)
}

pub fn field_FROM_hand_field(hand : &Hand, field: &Field) -> Field {
    let mut cells = vec![0.0f64; (field.size.0*field.size.1) as usize];
    for r  in 0..field.size.0 { 
        for c in 0..field.size.1 {
            let u = (r*field.size.1 + c) as usize;
            cells[u] = f64_FROM_hand_field_target(hand, field, (r,c));
        } 
    }
    Field {
        size : field.size,
        cells
    }
}



pub fn box_hand_FROM_size_f64(max_size : (i32,i32), max_pow : f64) -> Hand {
    let mut rng = rand::thread_rng();
    let size = random_size_FROM_size(max_size);
    let fingers = box_fingers_FROM_size_f64(size,max_pow);
    Hand { fingers }
}

pub fn random_size_FROM_size(max_size : (i32,i32)) -> (i32,i32) {
    (2*random_i32_FROM_i32(max_size.0) + 1,2*random_i32_FROM_i32(max_size.1)+1)
}

pub fn random_i32_FROM_i32(max : i32) -> i32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..(max as usize)) as i32
}












