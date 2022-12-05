use rand::random;
use rand::Rng;
//use crate::layer::*;
use libm::tanh;
use crate::global;
use crate::field::*;


pub struct Filter {
	pub mask : Vec<f64>,
    pub targets : Vec<(i64,i64)>,
}
impl Filter {
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
            sum += field.get(row +t.0,col  +t.1)*self.mask[i];
        }
        tanh(sum)
    }
}

pub fn random_filter() -> Filter {
    let mut rng = rand::thread_rng();
    let row_span = rng.gen_range(0..global::max_filter_span) as i64;
    let col_span = rng.gen_range(0..global::max_filter_span) as i64;
    let mut mask = vec![0.0;((2*row_span+1)*(2*col_span+1)) as usize];
    for x in mask.iter_mut() {
        *x = global::mask_intensity*(1.0 - 2.0*rand::random::<f64>());
    }
    let targets = rectangular_target_set(row_span, col_span);
    Filter {
        mask,
        targets,
    }
}

pub fn random_generalized_filter() -> Filter {
    let mut rng = rand::thread_rng();
    let mut mask = vec![0.0;global::generalized_targets as usize];
    for x in mask.iter_mut() {
        *x = global::mask_intensity*(1.0 - 2.0*rand::random::<f64>());
    }
    let targets = generalized_target_set(
        global::generalized_targets as usize, 
        global::max_filter_span as i64,
        global::max_filter_span as i64,
    );
    Filter {
        mask,
        targets,
    }
}
    
pub fn random_filters() -> Vec<Filter> {
	let mut filters = vec![];
    for l in 0..global::num_filters {
        filters.push(random_filter());
    }
    filters
}

pub fn random_generalized_filters() -> Vec<Filter> {
	let mut filters = vec![];
    for _ in 0..global::num_layers {
        filters.push(random_generalized_filter());
    }
    filters
}

fn generalized_target_set(num_targets :  usize, row_span :  i64, col_span : i64) -> Vec<(i64,i64)> {
    let mut rng = rand::thread_rng();
    let mut targets = vec![];
    for i in 0..num_targets {
        targets.push(
            (rng.gen_range(0..row_span as usize) as i64,
            rng.gen_range(0..col_span as usize) as i64)
        );
    }
    targets
}

fn rectangular_target_set(row_span :  i64, col_span : i64) -> Vec<(i64,i64)> {
    let counter = 0usize;
    let mut targets : Vec<(i64,i64)> = vec![]; 
    for row in -row_span..=row_span {
        for col in -col_span..=col_span {
            targets.push((row,col));
        }
    } 
    targets
}


