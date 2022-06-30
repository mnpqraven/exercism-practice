#[repr(usize)]
#[derive(Debug, PartialEq, Sequence, Clone, Copy, IntEnum)]
pub enum ResistorColor {
    Black = 0,
    Brown = 1,
    Red = 2,
    Orange = 3,
    Yellow = 4,
    Green = 5,
    Blue = 6,
    Violet = 7,
    Grey = 8,
    White = 9
}

use enum_iterator::{all, Sequence};
use int_enum::IntEnum;

pub fn color_to_value(_color: ResistorColor) -> usize {
    return _color.int_value();
}

pub fn value_to_color_string(value: usize) -> String {
    // grab the enum with the selected value
    match ResistorColor::from_int(value) {
        Ok(resistor) => format!("{:?}",resistor),
        Err(_) => "value out of range".to_string()
    }
}

pub fn colors() -> Vec<ResistorColor> {
    // partial type hint (_) with turbofish (::<SomeType>)
    all::<ResistorColor>().collect::<Vec<_>>()
}
