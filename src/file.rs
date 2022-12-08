
//use std::fs::File;
use std::fs::*;
use std::io::Write;

//use chrono::Utc;
use crate::fflo::*;
use crate::filter::*;
use crate::finger::*;

impl Fflo {
    pub fn write_filter(&self, index : usize, file : &mut File)  {
        let s =  self.filters[index].to_string();
        //writeln!(file, "FILTER {:02} START", index).unwrap();
        writeln!(file, "#{}", s).unwrap();
        //writeln!(file, "FILTER {:02} STOP", index).unwrap();
    }
    pub fn write_filters(&mut self) {
        let name = format!("fflofiles/{:?}.txt", chrono::offset::Local::now());
        let mut file = File::create(name).unwrap();
        for i in 0..self.filters.len() {
            self.write_filter(i, &mut file);
        }
        self.save_counter += 1;
    }
    pub fn load_filters(&mut self, filename : &str) {
        let filters_as_string = read_to_string(filename).expect("no file!");
        let mut filters: Vec<Finger> = vec!();
        let mut filters_as_strings = filters_as_string.split("#");
        for filter_string in filters_as_strings {
            let filters = filter_string.split(",");
        }
    }

}