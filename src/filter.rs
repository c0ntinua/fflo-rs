// use rand::random;
// use rand::Rng;
// use libm::tanh;
// use crate::global;
// use crate::field::*;
// use crate::fflo::*;
// use crate::finger::*;

// pub struct Filter {
// 	pub mask : Vec<f64>,
//     pub targets : Vec<(i64,i64)>,
// }


// impl Filter {
//     pub fn to_string(&self) -> String {
//         let strings : Vec<String>= self.to_fingers().iter().map(|x| x.to_string()).collect();
//         strings.join("\n")
//         //.iter().map(|x| x.to_string()).collect();
//     }
//     pub fn field_from_filter(&self, field : &Field) -> Field {
//         let mut cells = vec![0.0f64; field.rows*field.cols];
//         for r in 0..field.rows {
//             for c in 0..field.cols {
//                 cells[(r*field.cols +c) as usize] = self.of_cell(field, r as i64, c as i64);
//             }
//         }
//         Field {
//             rows : field.rows,
//             cols : field.cols,
//             cells
//         }
//     }
//     pub fn of_cell(&self, field : &Field, row : i64, col: i64) -> f64 {
//         let mut sum = 0.0f64;
//         for (i, t) in self.targets.iter().enumerate() {
//             sum += field.get(row + t.0,col + t.1)*self.mask[i];
//         }
//         tanh(sum)
//     }
//     pub fn to_fingers(&self) -> Vec<Finger> {
//         debug_assert!(self.mask.len() == self.targets.len());
//         let mut hand = vec!();
//         for i in 0..self.targets.len() {
//             hand.push( Finger {target : self.targets[i],action : self.mask[i]});
//         }
//         hand
//     }
//     pub fn filter_from_string(filter_string : &str) -> Vec<Finger> {
//         let mut filter: Vec<Finger> = vec!();
//         let mut fingers_as_strings = filter_string.split("\n");
//         for finger_string in fingers_as_strings {
//             filter.push(finger_from_string(finger_string));
//         }
//         filter
//     }
// }

// impl Fflo {
//     pub fn filter(&self, kind : &str) -> Filter {
//         let mut targets = vec![];
//         let mut rng = rand::thread_rng();
//         let row_span = rng.gen_range(0..self.row_span) as i64 + 1;
//         let col_span = rng.gen_range(0..self.col_span) as i64 + 1;
//         match kind {
//             "rect" => targets = rect_target_set(row_span, col_span),
//             "jagged" => targets = jagged_target_set(self.targets, row_span, col_span),
//             _ => targets = rect_target_set(row_span, col_span),
//         }
//         let mut mask = vec![0.0;targets.len()];
//         for x in mask.iter_mut() {
//             *x = self.mask_intensity*(1.0 - 2.0*rand::random::<f64>());
//         }
//         Filter {
//             mask,
//             targets,
//         }
//     }
//     pub fn random_filters(&self, kind : &str, num : usize) -> Vec<Filter> {
//         let mut filters = vec![];
//         for _ in 0..num {
//             filters.push(self.filter(kind));
//         }
//         filters
//     }
// }

// fn jagged_target_set(num_targets :  usize, row_span :  i64, col_span : i64) -> Vec<(i64,i64)> {
//     let mut rng = rand::thread_rng();
//     let mut targets = vec![];
//     for i in 0..num_targets {
//         targets.push((
//             rng.gen_range(0..row_span as usize) as i64,
//             rng.gen_range(0..col_span as usize) as i64
//         ));
//     }
//     targets
// }

// fn rect_target_set(row_span :  i64, col_span : i64) -> Vec<(i64,i64)> {
//     let counter = 0usize;
//     let mut targets : Vec<(i64,i64)> = vec![]; 
//     for row in -row_span..=row_span {
//         for col in -col_span..=col_span {
//             targets.push((row,col));
//         }
//     } 
//     targets
// }


