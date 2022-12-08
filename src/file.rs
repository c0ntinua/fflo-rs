
//use std::fs::File;
use std::fs::*;
use std::io::Write;
use crate::fflo::*;
use crate::hand::*;
use crate::finger::*;


pub fn write_hand(hand : &Hand, file : &mut File)  {
    let s = hand_as_string(hand);
    writeln!(file, "#\n{}", &s).unwrap();
}
pub fn new_file_name() -> String {
    format!("fflofiles/H{:?}.txt", chrono::offset::Local::now())
}
pub fn new_file() -> File {
    File::create(new_file_name()).unwrap()
}


impl Fflo {
    pub fn write_hands(&self) {
        let mut file = new_file();
        for hand in self.hands.iter() {
            write_hand(hand, &mut file);
        }
    }

}