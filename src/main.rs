
extern crate rand;
extern crate libm;
mod field; use field::*;
mod fflo; use fflo::*;
mod filter; use filter::*;
mod global;
use std::process::exit;
use std::time::{Duration, Instant};
use std::{thread, time};
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;



fn main() {
	let now = Instant::now();
	let mut flicker_counter = 0u64;
	let mut fflo = default_fflo();

	let (mut rl, thread) = raylib::init()
        .size(500, 500)
        .title("ffl0-rs")
        .build();
	while !rl.window_should_close() {
		if rl.is_key_down(KEY_Q) {exit(0);}
		if rl.is_key_down(KEY_LEFT) {fflo.flickers = 0;}
		if rl.is_key_down(KEY_RIGHT) {fflo.flickers += 1;}
		if rl.is_key_down(KEY_P) {fflo.paused = true;}
		if rl.is_key_down(KEY_U) {fflo.paused = false;}
		if rl.is_key_down(KEY_COMMA) {fflo.apply_filters();}
		if rl.is_key_down(KEY_ZERO) {fflo.flickers = 0;}
		if rl.is_key_down(KEY_ONE) {fflo.flickers = 50;}
		if rl.is_key_down(KEY_TWO) {fflo.flickers = 80;}
		if rl.is_key_down(KEY_THREE) {fflo.flickers = 100;}
		if rl.is_key_down(KEY_F) {fflo.filters = random_filters();}
		if rl.is_key_down(KEY_G) {fflo.filters = random_generalized_filters();}
		if rl.is_key_down(KEY_L) {
			fflo.field = random_field(fflo.rows, fflo.cols);
		}
		if rl.is_key_down(KEY_A) {
			fflo.apply_filters();
		}
		let mut screen = rl.begin_drawing(&thread);
		if !fflo.paused &&  flicker_counter >= fflo.flickers {
			fflo.apply_filters();
			flicker_counter = 0;

		} else {
			flicker_counter += 1;
		}
		fflo.plot_canvas(&mut screen);
	
		//if !paused {
			// 
			// for (i, filter) in filters.iter().enumerate() {
			// 	filter.filter_layer_in_place(&mut layer);
			// }
			// 
			// 	
			// }
		}
		

	}