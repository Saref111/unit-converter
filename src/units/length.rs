use strum::{Display, EnumIter};

use super::consts::{KM_M_FACTOR, MI_FT_FACTOR};

#[derive(Debug, EnumIter, PartialEq, Display)]
pub enum Length {
    Meter,
    Kilometer,
    Mile,
    Foot,

    Unknown
}

impl From<String> for Length {
    fn from(value: String) -> Self {
        let l_value = value.to_lowercase();
        let value = l_value.as_str();
        
        match value {
            "meter" | "meters" | "m" => Self::Meter,
            "kilometer" | "kilometers" | "km" => Self::Kilometer,
            "mile" | "miles" | "mi" => Self::Mile,
            "foot" | "feet" | "ft" => Self::Foot,
            _ => Self::Unknown,
        }
    }
}

fn convert_from_meter(v: f64, to: Length) -> f64 {
    match to {
        Length::Meter => v,
        Length::Kilometer => v * KM_M_FACTOR,
        Length::Mile => v * 1609.34,
        Length::Foot => v * 0.3048,
        Length::Unknown => v,
    }
}

fn convert_from_kilometer(v: f64, to: Length) -> f64 {
    match to {
        Length::Meter => v / KM_M_FACTOR,
        Length::Kilometer => v,
        Length::Mile => v * 0.621371,
        Length::Foot => v * 3280.84,
        Length::Unknown => v,
    }
}

fn convert_from_mile(v: f64, to: Length) -> f64 {
    match to {
        Length::Meter => v * 1609.34,
        Length::Kilometer => v * 1.60934,
        Length::Mile => v,
        Length::Foot => v * MI_FT_FACTOR,
        Length::Unknown => v,
    }
}

fn convert_from_foot(v: f64, to: Length) -> f64 {
    match to {
        Length::Meter => v * 0.3048,
        Length::Kilometer => v * 0.0003048,
        Length::Mile => v / MI_FT_FACTOR,
        Length::Foot => v,
        Length::Unknown => v,
    }
}

pub fn convert_length(value: f64, from: Length, to: Length) -> f64 {
    match from {
        Length::Meter => convert_from_meter(value, to),
        Length::Kilometer => convert_from_kilometer(value, to),
        Length::Mile => convert_from_mile(value, to),
        Length::Foot => convert_from_foot(value, to),
        Length::Unknown => value,
    }
}