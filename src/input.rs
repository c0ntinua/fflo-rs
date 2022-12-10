use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use std::process::exit;

// use crate::init::*;use crate::settings::*;use crate::file::*;use crate::canvas::*;
// use crate::plot::*;use crate::string::*;use crate::hand::*;use crate::hands::*;use crate::field::*;use crate::finger::*;

pub fn respond_to_input(rl : &RaylibHandle) {
    if rl.is_key_down(KEY_Q) {exit(0);}
	
	// if rl.is_key_down(KEY_ONE)   {fflo.switch_hand(1);}
	// if rl.is_key_down(KEY_TWO)   {fflo.switch_hand(2);}
	// if rl.is_key_down(KEY_THREE) {fflo.switch_hand(3);}
	// if rl.is_key_down(KEY_FOUR)  {fflo.switch_hand(4);}
	// if rl.is_key_down(KEY_FIVE)  {fflo.switch_hand(5);}
	// if rl.is_key_down(KEY_SIX)   {fflo.switch_hand(6);}
	// if rl.is_key_down(KEY_SEVEN) {fflo.switch_hand(7);}
	// if rl.is_key_down(KEY_EIGHT) {fflo.switch_hand(8);}
	// if rl.is_key_down(KEY_NINE)  {fflo.switch_hand(9);}
	// if rl.is_key_down(KEY_ZERO)  {fflo.switch_hand(0);}
	// if rl.is_key_down(KEY_RIGHT) {fflo.flickers += 1;}
	// if rl.is_key_down(KEY_LEFT) {fflo.flickers = 1;}
    // if rl.is_key_down(KEY_UP)    {fflo.filterings += 1;}
	// if rl.is_key_down(KEY_DOWN)  {fflo.filterings = 1;}
	// if rl.is_key_down(KEY_P)     {fflo.paused = true;}
	// if rl.is_key_down(KEY_U)     {fflo.paused = false;}
	// if rl.is_key_down(KEY_R)     {fflo.load_saved_hands();}
	// // if rl.is_key_down(KEY_COMMA) {fflo.apply_filters();}
	// // if rl.is_key_down(KEY_F) {fflo.replace_filters();}
	// if rl.is_key_down(KEY_X) {fflo.load_hands();fflo.randomize_field();}
	// // if rl.is_key_down(KEY_G) {fflo.rect_mode =!fflo.rect_mode;}
	// if rl.is_key_down(KEY_SPACE) { fflo.randomize_field();}
    // // if rl.is_key_down(KEY_C) { fflo.field = uniform_field(fflo.rows, fflo.cols,-1.0f64);}
	// // if rl.is_key_down(KEY_A) { fflo.apply_filters();}
	// if rl.is_key_down(KEY_W) { fflo.write_hands();}
}