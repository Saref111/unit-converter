use strum::{Display, EnumIter};

use super::consts::{KM_M_FACTOR, MI_M_FACTOR, M_FT_FACTOR};

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

pub fn convert_length(value: f64, from: &Length, to: &Length) -> f64 {
    let meters = match from {
        Length::Meter => value,
        Length::Kilometer => value * KM_M_FACTOR,
        Length::Mile => value * MI_M_FACTOR,
        Length::Foot => value * M_FT_FACTOR,
        Length::Unknown => return value,
    };

    match to {
        Length::Meter => meters,
        Length::Kilometer => meters / KM_M_FACTOR,
        Length::Mile => meters / MI_M_FACTOR,
        Length::Foot => meters / 0.3048,
        Length::Unknown => meters,
    }
}