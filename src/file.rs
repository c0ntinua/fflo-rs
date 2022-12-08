
//use std::fs::File;
use std::fs::*;
use std::io::Write;

//use chrono::Utc;
use crate::fflo::*;
use crate::hand::*;
use crate::finger::*;


pub fn write_hand(hand : &Hand, file : &mut File)  {
    let s = hand_as_string(hand);
    writeln!(file, "#\n{}", &s).unwrap();
}


impl Fflo {


    pub fn write_hands(&self) {
        let name = format!("fflofiles/H{:?}.txt", chrono::offset::Local::now());
        let mut file = File::create(name).unwrap();
        for hand in self.hands.iter() {
            write_hand(hand, &mut file);
        }
    }
    // pub fn load_filters(&mut self, filename : &str) {
    //     let filters_as_string = read_to_string(filename).expect("no file!");
    //     let mut filters: Vec<Finger> = vec!();
    //     let mut filters_as_strings = filters_as_string.split("#");
    //     for filter_string in filters_as_strings {
    //         let filters = filter_string.split(",");
    //     }
    // }

}