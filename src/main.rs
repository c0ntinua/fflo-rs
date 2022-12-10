mod field;mod finger;mod hand;mod hands;mod input;
mod init;mod settings;mod file;mod plot;mod string;mod canvas;
extern crate rand;
extern crate libm;
use std::fs::File;
use raylib::prelude::*;
use std::env;
use input::*;
use init::*;
use settings::*;
use canvas::*;
use hands::*;
use hand::*;
use field::*;
use plot::*;



fn main() {
	let mut settings = settings_FROM_args(env::args().collect());
	let (mut handle, thread) = handle_thread_FROM_settings(&settings);
	let mut canvas = canvas_FROM_settings(&settings);
	let mut field = field_FROM_settings(&settings);
	let mut hands = hands_FROM_settings(&settings);
	while !handle.window_should_close() {
		respond_to_input(&handle);
		for hand in hands.iter() {
			field = field_FROM_hand_field(hand, &field);
		}
		canvas = canvas_FROM_field_canvas(&field, &canvas);
		let mut screen = handle.begin_drawing(&thread);
		plot_WITH_screen_canvas(&mut screen, &canvas);
	}
}
