use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use crate::field::*;
use crate::fflo::*;
use crate::filter::*;
use std::process::exit;

pub fn respond_to_input(rl : &RaylibHandle, fflo : &mut Fflo) {
    if rl.is_key_down(KEY_Q) {exit(0);}
	if rl.is_key_down(KEY_LEFT) {fflo.flickers = 0;}
	if rl.is_key_down(KEY_RIGHT) {fflo.flickers += 1;}
    if rl.is_key_down(KEY_UP) {fflo.filterings += 1;}
	if rl.is_key_down(KEY_DOWN) {fflo.filterings = 1;}
	if rl.is_key_down(KEY_P) {fflo.paused = true;}
	if rl.is_key_down(KEY_U) {fflo.paused = false;}
	if rl.is_key_down(KEY_COMMA) {fflo.apply_filters();}
	if rl.is_key_down(KEY_F) {fflo.replace_filters();}
	if rl.is_key_down(KEY_X) {fflo.replace_filters();fflo.randomize_field();}
	if rl.is_key_down(KEY_G) {fflo.rect_mode =!fflo.rect_mode;}
	if rl.is_key_down(KEY_SPACE) { fflo.randomize_field();}
    if rl.is_key_down(KEY_C) { fflo.field = uniform_field(fflo.rows, fflo.cols,-1.0f64);}
	if rl.is_key_down(KEY_A) { fflo.apply_filters();}
	if rl.is_key_down(KEY_W) { fflo.write_filters();}
}