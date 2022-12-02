use rand::random;
use rand::Rng;
use crate::settings::*;
use crate::layer::*;
use libm::{sin,cos,tanh, exp};
use float_extras::f64::fmod;

pub struct Filter {
	pub mask : Vec<f64>,
    pub targets : Vec<(i64,i64)>,
}
impl Filter {
    pub fn filter_layer_in_place(&self, layer : &mut Layer) {
        let mut new_cells = vec![0.0f64; (layer.rows*layer.cols) as usize];
        for r in 0..layer.rows {
            for c in 0..layer.cols {
                    new_cells[(r*layer.cols +c) as usize] = self.of_cell(layer, r as i64, c as i64);
            }
        }
        layer.cells = new_cells;
    }

    pub fn of_cell(&self, layer : &Layer, row : i64, col : i64) -> f64 {
        let mut sum = 0.0f64;
        for (i, t) in self.targets.iter().enumerate() {
            sum += layer.get(row+t.0,col+t.1)*self.mask[i];
        }
        tanh(sum)

    }
    pub fn filter_random_piece_of_layer(&self, layer : &mut Layer, num_cells : usize) {
        let mut rng = rand::thread_rng();
        let mut new_value = 0.0f64;
        for _ in 0..num_cells {
            let row = rng.gen_range(0..layer.rows) as i64;
            let col = rng.gen_range(0..layer.cols) as i64;
            new_value = self.of_cell(layer, row, col);
            layer.cells[((row as u32)*layer.cols+(col as u32)) as usize] = new_value;
        }
    }
}

pub fn random_filter(settings : &Settings) -> Filter {
    let mut rng = rand::thread_rng();
    let row_span = rng.gen_range(0..settings.max_filter_span) as i64;
    let col_span = rng.gen_range(0..settings.max_filter_span) as i64;
    let mut mask = vec![0.0;((2*row_span+1)*(2*col_span+1)) as usize];
    for x in mask.iter_mut() {
             *x = 4.0f64*(1.0 - 2.0*rand::random::<f64>());
    }
    let targets = rectangular_target_set(row_span, col_span);
    Filter {
        mask,
        targets,
    }
}

fn random_generalized_filter(settings : &Settings) -> Filter {
    let mut rng = rand::thread_rng();
    let mut mask = vec![0.0;settings.generalized_targets as usize];
    for x in mask.iter_mut() {
        *x = 4.0f64*(1.0 - 2.0*rand::random::<f64>());
    }
    let targets = generalized_target_set(
        settings.generalized_targets as usize, 
        settings.max_filter_span as i64,
        settings.max_filter_span as i64,
    );
    Filter {
        mask,
        targets,
    }
}
    
pub fn random_filters(settings : &Settings) -> Vec<Filter> {
	let mut filters = vec![];
    for l in 0..settings.num_layers {
        filters.push(random_filter(settings));
    }
    filters
}
pub fn random_generalized_filters(settings : &Settings) -> Vec<Filter> {
	let mut filters = vec![];
    for l in 0..settings.num_layers {
        filters.push(random_generalized_filter(settings));
    }
    filters
}

pub fn generalized_target_set(num_targets :  usize, row_span :  i64, col_span : i64) -> Vec<(i64,i64)> {
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
    
    




pub fn rectangular_target_set(row_span :  i64, col_span : i64) -> Vec<(i64,i64)> {
    let counter = 0usize;
    let mut targets : Vec<(i64,i64)> = vec![]; 
    for row in -row_span..=row_span {
        for col in -col_span..=col_span {
            targets.push((row,col));
        }
    } 
    targets
}


