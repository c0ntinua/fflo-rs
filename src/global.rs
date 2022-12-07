#![allow(non_upper_case_globals)]
pub const rows : u32 = 500;
pub const cols : u32 = 500;
pub const pixel_height : u32 = 2;
pub const pixel_width : u32 = 2;
pub const text_height : u32 = 40;
pub const num_layers : u32 = 1;
pub const num_filters : u32 = 3;

pub const filterings_between_frames : u32 = 1;
pub const max_filter_span : u32 = 11;
pub const max_filter_height : u32 = 11;
pub const max_filter_width : u32 = 11;
pub const generalized_targets : u32 = 23;
pub const scale : f64 = 1.0/128.0;
pub const mask_intensity : f64 = 2.0;