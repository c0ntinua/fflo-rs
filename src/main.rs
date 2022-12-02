
extern crate rand;
extern crate libm;
extern crate minifb;use minifb::*;
mod filter; use filter::*;
mod settings; use settings::*;
mod layer; use layer::*;
mod buffer; use buffer::*;
mod window; use window::*;
use std::thread::sleep;
use std::time::{Duration, Instant};


   

fn main() {
    let mut settings = default_settings(100,200,10,5);
    let mut window = new_window(&settings);
    let mut buffer = new_buffer(&settings);
    let mut layers = random_layers(&settings);
    let mut filters = random_filters(&settings);
	let mut delay = settings.delay;
	let mut filterings = settings.filterings_between_frames as usize;
    let mut paused = false;
    let mut delay_counter = 0;
	let mut window = new_window(&settings);
    
	window.set_key_repeat_delay(0.5);
	while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.add_abstract_pixels(&layers);
		window.update_with_buffer(&buffer.cells, settings.pixel_rows as usize,settings.pixel_cols as usize).ok();
		window.get_keys_pressed(KeyRepeat::Yes).iter().for_each(|key|
			match key {			
				Key::P => paused = !paused,
                Key::F => filters = random_filters(&settings),
                Key::L => layers = random_layers(&settings),
				Key::O => delay = 2,
				Key::A => {if delay > 2 {delay -= 1;}},
				Key::S => delay += 1,
				Key::X => filterings += 1,
				Key::Z => filterings = 1,
				_ => (),
			}
    	);
		if !paused && delay_counter >= delay {
			for _ in 0..filterings {
				for (i, filter) in filters.iter().enumerate() {
					//filter.filter_layer_in_place(&mut layers[i%(settings.num_layers as usize)]);
					filter.filter_random_piece_of_layer(&mut layers[i%(settings.num_layers as usize)], 500);
				}
			}
			delay_counter = 0;
			//sleep(Duration::from_millis(delay));
		} else {
			delay_counter += 1;
		}
	}
}