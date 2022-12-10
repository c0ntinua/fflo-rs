use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use std::process::exit;
use crate::hands::*;
use crate::field::*;
use crate::settings::*;
use crate::file::*;
use crate::hand::*;
//use crate::canvas::*;
// use crate::plot::*;use crate::string::*;use crate::finger::*;

pub fn respond_to_input(rl : &RaylibHandle, hands : &mut Vec<Hand>, field : &mut Field, settings : &mut Settings) {
    if rl.is_key_down(KEY_Q) {exit(0);}
	if rl.is_key_down(KEY_W) {write_WITH_hands(&hands);}
	if rl.is_key_down(KEY_SPACE) {UPDATE_field(field);}
	if rl.is_key_down(KEY_X) {*hands = random_hands_FROM_settings(settings);}
}