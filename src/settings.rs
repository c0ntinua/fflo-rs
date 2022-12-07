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
        rows : 250,
        cols : 500,
        pixel_height : 2,
        pixel_width : 1,
        text_height : 0,
        font_name : "FragmentMono-Regular.ttf".to_string(),
    }
}