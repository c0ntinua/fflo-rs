use rand::random;
use rand::Rng;
use libm::tanh;
use crate::global;
use crate::field::*;
use crate::fflo::*;
use crate::finger::*;

type Hand = Vec<Finger>;

pub fn hand_of_cell(hand : &Hand , field: &Field, target : (i64,i64) ) -> f64 {

}
pub fn of_cell(&self, field : &Field, row : i64, col: i64) -> f64 {
    let mut sum = 0.0f64;
    for (i, t) in self.targets.iter().enumerate() {
        sum += field.get(row + t.0,col + t.1)*self.mask[i];
    }
    tanh(sum)
}



    pub fn field_from_filter(&self, field : &Field) -> Field {
        let mut cells = vec![0.0f64; field.rows*field.cols];
        for r in 0..field.rows {
            for c in 0..field.cols {
                cells[(r*field.cols +c) as usize] = self.of_cell(field, r as i64, c as i64);
            }
        }
        Field {
            rows : field.rows,
            cols : field.cols,
            cells
        }
    }
    pub fn of_cell(&self, field : &Field, row : i64, col: i64) -> f64 {
        let mut sum = 0.0f64;
        for (i, t) in self.targets.iter().enumerate() {
            sum += field.get(row + t.0,col + t.1)*self.mask[i];
        }
        tanh(sum)
    }


