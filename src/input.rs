use raylib::consts::KeyboardKey::*;
use raylib::prelude::*;
use std::process::exit;
use crate::hands::*;
use crate::field::*;
use crate::fields::*;
use crate::settings::*;
use crate::file::*;
use crate::hand::*;
use crate::hands::*;
//use crate::canvas::*;
// use crate::plot::*;use crate::string::*;use crate::finger::*;

pub fn respond_to_input(rl : &RaylibHandle, hands : &mut Vec<Hand>, fields : &mut Vec<Field>, settings : &mut Settings) {
	if rl.is_key_down(KEY_Q) {exit(0);}
	if rl.is_key_released(KEY_W) {write_WITH_hands(&hands);}
	if rl.is_key_released(KEY_M) {settings.mode += 1; if settings.mode >= settings.num_modes {settings.mode = 0;} }
	if rl.is_key_released(KEY_F) {settings.flickers += 1; if settings.flickers >= settings.max_flickers {settings.flickers = 0;} }
	if rl.is_key_down(KEY_ZERO) {UPDATE_hands_WITH_index_settings(hands, 0, settings);}
	if rl.is_key_down(KEY_ONE) {UPDATE_hands_WITH_index_settings(hands, 1, settings);}
	if rl.is_key_down(KEY_TWO) {UPDATE_hands_WITH_index_settings(hands, 2, settings);}
	if rl.is_key_down(KEY_THREE) {UPDATE_hands_WITH_index_settings(hands, 3, settings);}
	if rl.is_key_down(KEY_FOUR) {UPDATE_hands_WITH_index_settings(hands, 4, settings);}
	if rl.is_key_down(KEY_FIVE) {UPDATE_hands_WITH_index_settings(hands, 5, settings);}
	if rl.is_key_down(KEY_SIX) {UPDATE_hands_WITH_index_settings(hands, 6, settings);}
	if rl.is_key_down(KEY_SEVEN) {UPDATE_hands_WITH_index_settings(hands, 7, settings);}
	if rl.is_key_down(KEY_EIGHT) {UPDATE_hands_WITH_index_settings(hands, 8, settings);}
	if rl.is_key_down(KEY_NINE) {UPDATE_hands_WITH_index_settings(hands, 9, settings);}
	if rl.is_key_down(KEY_X) {*hands = random_hands_FROM_settings(settings);}

	match settings.mode {
		0 => if rl.is_key_down(KEY_SPACE) {UPDATE_field(&mut fields[0]);},
		1 => if rl.is_key_down(KEY_SPACE) {UPDATE_fields(fields);}
		_ => {},
	}	
}