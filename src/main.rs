
extern crate rand;
extern crate libm;
mod fflo; use fflo::*;
mod field; use field::*;
mod filter; use filter::*;
mod input; use input::*;
use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;



fn main() {
	let now = Instant::now();
	let mut flicker_counter = 0u64;
	let mut fflo = default_fflo();

	let (mut rl, thread) = raylib::init()
        .size((fflo.cols*fflo.pixel_width) as i32,(fflo.rows*fflo.pixel_height) as i32)
        .title("ffl0-rs")
        .build();
	while !rl.window_should_close() {
		handle_input(&rl, &mut fflo);
		fflo.flicker();
		let mut screen = rl.begin_drawing(&thread);
		fflo.plot_canvas(&mut screen);
	}
}