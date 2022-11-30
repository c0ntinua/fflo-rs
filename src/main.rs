
extern crate rand;
extern crate libm;
extern crate minifb;use minifb::*;
mod filter; use filter::*;
mod settings; use settings::*;
mod layer; use layer::*;
mod buffer; use buffer::*;
mod window; use window::*;

fn main() {
    let mut settings = default_settings(250,500,4,2);
    let mut window = new_window(&settings);
    let mut buffer = new_buffer(&settings);
    let mut layers = random_layers(&settings);
    let mut filters = random_filters(&settings);
    let mut paused = false;
    let mut delay_counter = 0;
	let mut window = new_window(&settings);
    
	window.set_key_repeat_delay(0.5);
	while window.is_open() && !window.is_key_down(Key::Escape) {
        // buffer = layers_to_buffer(&triplex.red, &triplex.green, &triplex.blue);
        buffer.add_abstract_pixels(&layers);
		window.update_with_buffer(&buffer.cells, settings.pixel_rows as usize,settings.pixel_cols as usize).ok();
		window.get_keys_pressed(KeyRepeat::Yes).iter().for_each(|key|
			match key {			
				Key::P => paused = !paused,
                Key::F => filters = random_filters(&settings),
                Key::L => layers = random_layers(&settings),
				_ => (),
			}
    	);
		if !paused && !(delay_counter >= settings.delay) {
			for i in 0..settings.filterings_between_frames {
                for (i, filter) in filters.iter().enumerate() {
                    filter.filter_layer_in_place(&mut layers[i%(settings.num_layers as usize)]);
                }
				//filter_system[0].filter_layer_in_place(&mut layers[0]);		
			}
			delay_counter = 0;
		} else {
			delay_counter += 1;
		}

	}
}