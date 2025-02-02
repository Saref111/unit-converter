use strum::{Display, EnumIter};

use crate::converter::ConvertError;

use super::{
    consts::{
        KM_M_FACTOR, 
        MI_M_FACTOR, 
        M_FT_FACTOR
    }, 
    main::Convertible
};

#[derive(Debug, EnumIter, PartialEq, Display, Clone)]
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

impl Convertible for Length {
    fn convert(&self, value: f64, to: Box<Length>) -> Result<f64, ConvertError> {
        convert_length(value, self, to)
    }
}

pub fn convert_length(value: f64, from: &Length, to: Box<Length>) -> Result<f64, ConvertError> {
    let meters = match from {
        Length::Meter => value,
        Length::Kilometer => value * KM_M_FACTOR,
        Length::Mile => value * MI_M_FACTOR,
        Length::Foot => value * M_FT_FACTOR,
        Length::Unknown => return Err(ConvertError::UnknownUnit(value, (*from).clone().into(), (*to).into())),
    };

    let result = match *to {
        Length::Meter => meters,
        Length::Kilometer => meters / KM_M_FACTOR,
        Length::Mile => meters / MI_M_FACTOR,
        Length::Foot => meters / M_FT_FACTOR,
        Length::Unknown => return Err(ConvertError::UnknownUnit(value, (*from).clone().into(), (*to).into())),
    };

    Ok(result)
}