use crate::fflo::*;

pub struct Settings {
    pub rows : usize,
    pub cols : usize,
    pub pixel_height : usize,
    pub pixel_width : usize,
    pub text_height : usize,
    pub font_name : String,
}
pub fn fundamental_settings() -> Settings {
    Settings {
        rows : 100,
        cols : 100,
        pixel_height : 10,
        pixel_width : 10,
        text_height : 0,
        font_name : "FragmentMono-Regular.ttf".to_string(),
    }
}

impl Fflo {
    pub fn init_hands(&mut self, args : Vec<String>) {
        if args.len() == 2 {
            let filename = &args[1];
            self.load_hands_from_file(filename);
        } else {
            self.load_hands();
        }
    }
}