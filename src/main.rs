extern crate rand;
extern crate libm;
use std::fs::File;
use raylib::prelude::*;
mod global;mod fflo;mod field;mod filter;mod input;mod init;mod settings;mod file;
use fflo::*;use input::*;use init::*;use settings::*;use file::*;



fn main() {
	// let mut file = File::create("foo.txt").unwrap();

	let settings = fundamental_settings();
	let (mut handle, thread, font ) = handle_thread_font(&settings);
	let mut fflo = new_fflo(&settings,font);
	fflo.write_filters();
	// while !handle.window_should_close() {
	// 	respond_to_input(&handle, &mut fflo);
	// 	fflo.flicker();
	// 	let mut screen = handle.begin_drawing(&thread);
	// 	fflo.plot(&mut screen);
	// }
}
