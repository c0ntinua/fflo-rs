extern crate rand;
extern crate libm;
use std::fs::File;
use raylib::prelude::*;
use std::env;
mod fflo;mod field;mod finger;mod hand;mod input;mod init;mod settings;mod file;mod plot;mod string;
use fflo::*;use input::*;use init::*;use settings::*;use file::*;use plot::*;

fn main() {

	let settings = fundamental_settings();
	let (mut handle, thread, font ) = handle_thread_font(&settings);
	let mut fflo = new_fflo(&settings);
	fflo.init_hands(env::args().collect());
	while !handle.window_should_close() {
		respond_to_input(&handle, &mut fflo);
		fflo.flicker();
		let mut screen = handle.begin_drawing(&thread);
		fflo.plot(&mut screen);
	}
}
