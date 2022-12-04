
extern crate rand;
extern crate libm;
extern crate minifb;use minifb::*;
mod filter; use filter::*;
mod layer; use layer::*;
mod buffer; use buffer::*;
mod window; use window::*;
mod global;
use std::process::exit;
use std::thread;


fn main() {

    let mut window = new_window();
    let mut buffer = new_buffer();
    let mut layers = random_layers();
    //let mut filters = random_filters();
	let mut filters = random_generalized_filters();
	let mut delay = global::delay;
	let mut filterings = global::filterings_between_frames as usize;
    let mut paused = false;
    let mut delay_counter = 0;
	let mut color = true;

	let mut window = new_window();
	window.set_key_repeat_delay(0.5);
	while window.is_open() && !window.is_key_down(Key::Escape) {
		if color {
			buffer.add_color_pixels(&layers);
		} else {
			buffer.add_abstract_pixels(&layers);
		}
		
		update_window_with(&mut window, &buffer.cells);
		window.get_keys_pressed(KeyRepeat::Yes).iter().for_each(|key|
			match key {			
				Key::P => paused = !paused,
                Key::F => filters = random_filters(),
                Key::L => layers = random_layers(),
				Key::O => delay = 0,
				Key::A => {if delay > 10 {delay -= 10;}},
				Key::D => delay = global::delay,
				Key::X => filterings += 1,
				Key::Z => filterings = 1,
				Key::C => color = !color,
				Key::Key0 => {delay = 0 ; filterings = 1},
				Key::Key1 => {delay = 10; filterings = 2},
				Key::N => for (i, filter) in filters.iter().enumerate() {
					filter.filter_layer_in_place(&mut layers[i%(global::num_layers as usize)]);
				},
				Key::Q => exit(0),
				
				_ => (),
			}
    	);
		if !paused && delay_counter >= delay {
			for _ in 0..filterings {
				for (i, filter) in filters.iter().enumerate() {
					filter.filter_layer_in_place(&mut layers[i%(global::num_layers as usize)]);
					//filter.filter_random_piece_of_layer(&mut layers[i%(global::num_layers as usize)], 500);
				}
			}
		} else {
			delay_counter += 1;
		}
	}
}