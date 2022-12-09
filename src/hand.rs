use rand::random;
use rand::Rng;
use libm::tanh;
use crate::fflo::*;
use crate::field::*;
use crate::finger::*;

pub type Hand = Vec<Finger>;



pub fn handled_cell(hand : &Hand , field: &Field, target : (i64,i64) ) -> f64 {
    let mut s = 0.0f64;
    for f in hand.iter() {
        s += field.spin(target_sum(f.target, target))*f.action;
    }
    tanh(s)
}

pub fn handled_field(hand : &Hand, field: &Field) -> Field {
    let mut cells = vec![0.0f64; field.rows*field.cols];
    for r  in 0..field.rows { 
        for c in 0..field.cols { 
            cells[field.go(r,c)] = handled_cell(hand, field, (r as i64 , c as i64));
        } 
    }
    Field {
        rows : field.rows,
        cols : field.cols,
        cells
    }
}


pub fn box_hand(rows :  i64, cols : i64, pow : f64) -> Hand {
    debug_assert!(rows % 2 == 1 && cols % 2 == 1);
    let mut hand = vec!();
    let (row_span, col_span) = (rows/2, cols/2);
    for r  in -row_span..=row_span {
        for c in -col_span..=col_span {
            hand.push(box_finger((r,c), pow));
        }
    } 
    hand
}

pub fn target_sum(x : (i64,i64), y : (i64,i64)) -> (i64,i64) {
    (x.0+y.0,x.1+y.1)
}

pub fn hand_as_string(hand : &Hand) -> String {
    hand.iter().map(|f : &Finger| finger_as_string(f)).collect::<Vec<String>>().join("\n")
}

pub fn random_box_hand(r : usize, c : usize, p : f64) -> Hand {
    let mut rng = rand::thread_rng();
    let rows = rng.gen_range(0..r);
    let cols = rng.gen_range(0..c);
    let act = random_action(p);
    box_hand((2*rows +1) as i64,(2*cols + 1) as i64,act)
}
pub fn random_hand(r : usize, c: usize,  pow : f64, targets : usize) -> Hand {
    let mut hand = vec!();
    let mut rng = rand::thread_rng();
    let rows = rng.gen_range(0..r);
    let cols = rng.gen_range(0..c);
    let act = random_action(pow);
    for _ in 0..targets {
        hand.push(finger(2*rows +1 ,2*cols + 1 ,act));
    }
    hand
}



impl Fflo {
    pub fn load_box_hands(&mut self) {
        self.hands = vec!();
        let mut rng = rand::thread_rng();
        for _ in 0..self.num_box_hands {
            self.hands.push(random_box_hand(self.row_span,self.col_span,self.pow));
        }
    }
    pub fn load_hands(&mut self) {
        self.hands = vec!();
        let mut rng = rand::thread_rng();
        for _ in 0..self.num_box_hands {
            self.hands.push(random_hand(self.row_span,self.col_span,self.pow, self.targets));
        }
    }
    pub fn switch_hand(&mut self, i : usize) {
        if i < self.hands.len() {
            self.hands[i] = random_box_hand(self.row_span,self.col_span,self.pow);
        }
    }
}





