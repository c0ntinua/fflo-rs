mod field;mod finger;mod hand;mod hands;mod input;mod fields;
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
use fields::*;



fn main() {
	let mut settings = settings_FROM_args(env::args().collect());
	let (mut handle, thread) = handle_thread_FROM_settings(&settings);
	let mut canvas = canvas_FROM_settings(&settings);
	//let mut field = field_FROM_settings(&settings);
	let mut fields = fields_FROM_settings(&settings);
	let mut hands = hands_FROM_settings(&settings);
	let mut counter = 0usize;
	while !handle.window_should_close() {
		respond_to_input(&handle, &mut hands, &mut fields, &mut settings);
		match settings.mode {
			0 => {
				for hand in hands.iter() {
					fields[0] = field_FROM_hand_field(hand, &fields[0]);
						
				}
				UPDATE_canvas_WITH_field(&mut canvas, &fields[0]);
			},
			1 => {
				for (i,hand) in hands.iter().enumerate() {
					fields[i%3] = field_FROM_hand_field(hand, &fields[i%3]);
				}
				UPDATE_canvas_WITH_fields(&mut canvas, &fields);
			},
			_ => {},
			}
		let mut screen = handle.begin_drawing(&thread);
		plot_WITH_screen_canvas(&mut screen, &canvas);
	}
}
