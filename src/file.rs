use std::fs::*;
use std::io::Write;
use crate::string::*;
use crate::hand::*;
use crate::hands::*;

pub fn hands_FROM_filename(filename : &str) -> Vec<Hand> {
    let string = read_to_string(filename).expect("no file!");
    let mut hands = hands_FROM_string(&string);
    hands
}
pub fn write_WITH_hand_filename(hand : &Hand, file : &mut File)  {
    let s = string_FROM_hand(hand);
    writeln!(file, "\n{}", &s).unwrap();
}
pub fn filename() -> String {
    format!("{:?}.txt", chrono::offset::Local::now())
}
pub fn file() -> File {
    File::create(format!("{:?}.txt", chrono::offset::Local::now())).unwrap()
}
pub fn write_WITH_hands(hands : &Vec<Hand>) {
    let mut file = file();
    for hand in hands.iter() {write_WITH_hand_filename(hand, &mut file);}
}
   