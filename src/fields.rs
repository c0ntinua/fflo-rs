use crate::settings::*;
use crate::field::*;

pub fn fields_FROM_settings(settings : &Settings) -> Vec<Field> {
    let mut fields = vec!();
    for _ in 0..settings.num_color_fields {
        fields.push(field_FROM_settings(settings));
    }
    fields
}
pub fn UPDATE_fields(fields : &mut Vec<Field>) {
    for field in fields {
        UPDATE_field(field);
    }
}