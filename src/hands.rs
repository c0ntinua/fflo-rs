use crate::settings::*;
use crate::hand::*;
use crate::file::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn random_hands_FROM_settings(settings : &Settings) -> Vec<Hand> {
    let mut hands = vec!();
    for _ in 0..settings.num_box_hands { hands.push(hand_FROM_settings(settings)); }
    hands
}
pub fn hand_FROM_settings(settings : &Settings) -> Hand {
    box_hand_FROM_size_f64(settings.box_max_size,settings.max_power)
}
pub fn hands_FROM_settings(settings : &Settings) -> Vec<Hand> {
    match settings.load_from_file {
        true => hands_FROM_filename(&settings.filename),
        false => random_hands_FROM_settings(settings),
    }
}
pub fn UPDATE_hands_WITH_index_settings(hands : &mut Vec<Hand> , index : usize, settings : &Settings) {
    if hands.len() > index {
        hands[index] = hand_FROM_settings(settings);
    }
}

pub fn SHUFFLE_hands(hands : &mut Vec<Hand>) {
    let mut rng = thread_rng();
    hands.shuffle(&mut rng);
}
