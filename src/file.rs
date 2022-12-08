
use std::fs::File;
use std::io::Write;
use crate::fflo::*;
//use crate::filter::*;

impl Fflo {
    pub fn write_filter(&self, index : usize, file : &mut File)  {
        let s =  self.filters[index].to_string();
        writeln!(file, "FILTER {:02} START", index).unwrap();
        writeln!(file, "{}", s).unwrap();
        writeln!(file, "FILTER {:02} STOP", index).unwrap();
    }
    pub fn write_filters(&self) {
        let filename = format!("{:03}.ffl",self.save_counter);
        let mut file = File::create(filename).unwrap();
        for i in 0..self.filters.len() {
            self.write_filter(i, &mut file);
        }
    }
}
