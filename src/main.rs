extern crate rand;
extern crate libm;
use raylib::prelude::*;
mod global;mod fflo;mod field;mod filter;mod input;mod init;mod settings;
use fflo::*;use input::*;use init::*;use settings::*;

fn main() {
	let settings = fundamental_settings();
	let (mut handle, thread, font ) = handle_thread_font(&settings);
	let mut fflo = new_fflo(&settings,font);
	while !handle.window_should_close() {
		respond_to_input(&handle, &mut fflo);
		fflo.flicker();
		let mut screen = handle.begin_drawing(&thread);
		fflo.plot(&mut screen);
	}
}

