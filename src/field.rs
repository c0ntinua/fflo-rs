use raylib::core::color::Color;
use crate::fflo::*;
use rand::random;
use rand::Rng;

#[derive(Clone)]
pub struct Field {
    pub rows : usize,
    pub cols : usize,
    pub cells : Vec<f64>,
}
pub const SCALE : f64 = 1.0/128.0;
impl Field {
    pub fn get(&self , r : i64, c : i64) -> f64 {
        let mut row =  r;
        let mut col =  c; 
        while row < 0 { row += self.rows as i64};
        while row > (self.rows as i64) - 1 { row -= self.rows as i64 };
        while col < 0 { col += (self.cols as i64)};
        while col > (self.cols as i64) - 1 { col -= self.cols as i64};
        self.cells[(row * self.cols  as i64 + col) as usize]
    }
    pub fn spin(&self , target : (i64,i64)) -> f64 {
        let (mut row, mut col) = target;
        while row < 0 { row += self.rows as i64};
        while row > (self.rows as i64) - 1 { row -= self.rows as i64 };
        while col < 0 { col += (self.cols as i64)};
        while col > (self.cols as i64) - 1 { col -= self.cols as i64};
        self.cells[(row * self.cols  as i64 + col) as usize]
    }
    pub fn access(&self, r : i64, c : i64) -> usize {
        (r*(self.cols as i64) + c) as usize
    }
    pub fn go(&self, r : usize, c : usize) -> usize {
        (r*self.cols + c) as usize
    }
    pub fn to_monochrome_canvas(&self) -> Vec<Color> {
        let mut canvas = vec![];
        for x in self.cells.iter() { canvas.push(color_from_float(*x)); }
        canvas
    }

}

impl Fflo {
    pub fn randomize_field(&mut self) {
        self.field = random_field(self.rows,self.cols);
    }
}

pub fn random_field(rows : usize, cols : usize) -> Field {
    let mut cells = vec![0.0; rows * cols];
	for x in cells.iter_mut() { *x = random_f64();}
    Field {
        rows,
        cols,
        cells    
    }
}
pub fn uniform_field(rows : usize, cols : usize, state : f64) -> Field {
    let mut cells = vec![0.0; rows * cols];
	for x in cells.iter_mut() { *x = state;}
    Field {
        rows,
        cols,
        cells    
    }
}


fn color_from_float(x : f64) -> Color {
    let h = u8_from_f64(x);
    Color {r : h, g : h, b : h, a : 255}
}

fn u8_from_f64(x : f64) -> u8 {
    ((x + 1.0)/SCALE).trunc() as u8
}

fn random_f64() -> f64 {
    1.0 - 2.0*rand::random::<f64>()
}
