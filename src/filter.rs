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
}

// pub fn mutate(&self, layer : &mut Layer) {
// 	let mut workspace = vec![0.0; layer.rows*layer.cols];	
// 	for row in 0u32..layer.rows {
// 		for col in 0u32..layer.cols {
// 				workspace[r*layer.cols +c] = self.apply_to_cell(layer, row as i64, col as i64);
// 		}
// 	}
// 	layer.cells = workspace;
// }
	
	
// pub fn random_filter(settings : &Settings) -> Filter {
// 	let mut rng = rand::thread_rng();	
// 	let row_span = rng.gen_range(0..settings.max_filter_span) as u32;
// 	let col_span = rng.gen_range(0..settings.max_filter_span) as u32;
//     let mut mask = vec![0.0;((2*row_span+1)*(2*col_span+1)) as usize];
//     for x in mask.iter_mut() {
//              *x = 1.0 - 2.0*rand::random::<f64>();
//     }
// 	Filter {
//         mask, 
//         row_span,
//         col_span,  
//         }
// }


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
pub fn random_filters(settings : &Settings) -> Vec<Filter> {
	let mut filters = vec![];
    for l in 0..settings.num_layers {
        filters.push(random_filter(settings));
    }
    filters
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


