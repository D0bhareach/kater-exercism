use enum_iterator::{all, Sequence};
use int_enum::IntEnum;
#[derive(Clone, Copy, Debug, PartialEq, Eq, Sequence, IntEnum)]
#[repr(u32)]
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
    White = 9,
}

pub fn color_to_value(color: ResistorColor) -> u32 {
    color.int_value()
}

pub fn value_to_color_string(value: u32) -> String {
    if value > 9 {
        return "value out of range".to_string();
    }
    let resistor = ResistorColor::from_int(value).unwrap();
    let res = match resistor {
        ResistorColor::Black => "Black",
        ResistorColor::Brown => "Brown",
        ResistorColor::Red => "Red",
        ResistorColor::Orange => "Orange",
        ResistorColor::Yellow => "Yellow",
        ResistorColor::Green => "Green",
        ResistorColor::Blue => "Blue",
        ResistorColor::Violet => "Violet",
        ResistorColor::Grey => "Grey",
        ResistorColor::White => "White",
    };
    res.to_string()
}

pub fn colors() -> Vec<ResistorColor> {
    all::<ResistorColor>().collect::<Vec<ResistorColor>>()
}
