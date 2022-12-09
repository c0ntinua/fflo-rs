extern crate rand;
extern crate libm;
use std::fs::File;
use raylib::prelude::*;
mod fflo;mod field;mod finger;mod hand;mod input;mod init;mod settings;mod file;mod plot;
use fflo::*;use input::*;use init::*;use settings::*;use file::*;use plot::*;



fn main() {
	let settings = fundamental_settings();
	let (mut handle, thread, font ) = handle_thread_font(&settings);
	let mut fflo = new_fflo(&settings,font);
	//fflo.write_hands();
	while !handle.window_should_close() {
		respond_to_input(&handle, &mut fflo);
		fflo.flicker();
		let mut screen = handle.begin_drawing(&thread);
		fflo.plot(&mut screen);
	}
}
