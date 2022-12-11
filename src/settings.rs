pub const SCALE : f64 = 1.0/128.0;

pub struct Settings {
    pub size : (i32,i32),
    pub pixel_size : (i32,i32),
    pub load_from_file : bool,
    pub filename : String,
    pub num_box_hands : usize,
    pub box_max_size : (i32,i32),
    pub max_power : f64,
    pub filterings : usize,

}
pub const S : usize = 250;//Array width
fn settings_DEFAULT() -> Settings {
    Settings {
        size : (250i32,100i32),
        pixel_size : (4i32,10i32),
        load_from_file : false,
        filename : "".to_string(),
        num_box_hands : 10,
        box_max_size : (3,3),
        max_power : 9.0f64,
        filterings : 1,

    }
}

pub fn settings_FROM_args(args : Vec<String>) -> Settings {
    let mut settings = settings_DEFAULT();
    if args.len() == 6 {
        settings.size.0 = args[1].parse::<i32>().unwrap();
        settings.size.0 = args[2].parse::<i32>().unwrap();
        settings.pixel_size.0 = args[3].parse::<i32>().unwrap();
        settings.pixel_size.0  = args[4].parse::<i32>().unwrap();
        settings.load_from_file = true;
        settings.filename = args[5].clone();
    }
    settings
}

