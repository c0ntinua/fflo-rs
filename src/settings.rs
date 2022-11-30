pub struct Settings {
	pub pixel_rows : u32,
	pub pixel_cols : u32,
    pub abstract_rows : u32,
	pub abstract_cols : u32,
    pub abstract_height : u32,
	pub abstract_width : u32,
    pub num_layers : u32,	
	pub num_filters : u32,
	pub delay : u32,
    pub filterings_between_frames : u32,
    pub max_filter_span : u32,
}
pub fn default_settings(abstract_rows : u32, abstract_cols : u32, abstract_height : u32, abstract_width : u32) -> Settings{
    Settings { 
        pixel_rows : abstract_rows*abstract_height,
        pixel_cols : abstract_cols*abstract_width,
        abstract_rows,
        abstract_cols,
        abstract_height,
	    abstract_width,
        num_filters : 7u32,
        num_layers : 1u32,
        delay : 1u32,
        filterings_between_frames : 1u32,
        max_filter_span : 9u32,
    }
}