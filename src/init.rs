use raylib::prelude::*;
use crate::settings::*;
pub fn handle_thread_FROM_settings(settings : &Settings) -> (RaylibHandle, RaylibThread)  {
    let height = settings.size.0*settings.pixel_size.0;
    let width  = settings.size.1*settings.pixel_size.1;
    let (mut handle, thread) = raylib::init().size(width,height).title("ffl0-rs").build();
    (handle, thread)  
}