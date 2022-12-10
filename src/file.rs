
//use std::fs::File;
use std::fs::*;
use std::io::Write;
use crate::fflo::*;
use crate::hand::*;
use crate::finger::*;
use crate::string::*;


pub fn write_hand(hand : &Hand, file : &mut File)  {
    let s = hand_as_string(hand);
    writeln!(file, "#\n{}", &s).unwrap();
}
pub fn new_file_name() -> String {
    format!("{:?}.txt", chrono::offset::Local::now())
}
pub fn new_file() -> File {
    File::create(new_file_name()).unwrap()
}


impl Fflo {

    pub fn print_hands_from_file(&self, filename : &str) {
        let string = read_to_string(filename).expect("no file!");
        print!("{}", string);
        let mut hands = hands_from_string(&string);
        for hand in &hands {
            println!("{}", string_from_hand(&hand));
        }
    }


    pub fn load_hands_from_file(&mut self, filename : &str) {
        let string = read_to_string(filename).expect("no file!");
        let mut hands = hands_from_string(&string);
        self.hands = hands;
    }

    pub fn write_hands(&self) {
        let mut file = new_file();
        for hand in self.hands.iter() {
            write_hand(hand, &mut file);
        }
    }
}
   