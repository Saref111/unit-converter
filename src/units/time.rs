use strum::{Display, EnumIter};

use crate::converter::ConvertError;

use super::{
    consts::{
        C_S_FACTOR, 
        DE_S_FACTOR, 
        D_S_FACTOR, 
        H_S_FACTOR, 
        MI_S_FACTOR, 
        MO_S_FACTOR, 
        M_S_FACTOR, 
        S_MS_FACTOR, 
        W_S_FACTOR, 
        Y_S_FACTOR
    },
    main::Convertible
};

#[derive(Debug, EnumIter, PartialEq, Display, Clone)]
pub enum Time {
    Millisecond,
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
    Decade,
    Century,
    Millennium,

    Unknown
}

impl Convertible for Time {
    fn convert(&self, value: f64, to: Box<Time>) -> Result<f64, ConvertError> {
        convert_time(value, self, to)
    }
}

impl From<String> for Time {
    fn from(value: String) -> Self {
        let l_value = value.to_lowercase();
        let value = l_value.as_str();
        
        match value {
            "ms" | "millisecond" | "milliseconds" => Self::Millisecond,
            "s" | "second" | "seconds" => Self::Second,
            "m" | "minute" | "minutes" => Self::Minute,
            "h" | "hour" | "hours" => Self::Hour,
            "d" | "day" | "days" => Self::Day,
            "w" | "week" | "weeks" => Self::Week,
            "month" | "months" => Self::Month,
            "y" | "year" | "years" => Self::Year,
            "decade" | "decades" => Self::Decade,
            "century" | "centuries" => Self::Century,
            "millennium" | "millennia" => Self::Millennium,
            _ => Self::Unknown,
        }
    }
}

pub fn convert_time(value: f64, from: &Time, to: Box<Time>) -> Result<f64, ConvertError> {
    let seconds = match from {
        Time::Millisecond => value / S_MS_FACTOR,
        Time::Second => value,
        Time::Minute => value * M_S_FACTOR,
        Time::Hour => value * H_S_FACTOR,
        Time::Day => value * D_S_FACTOR,
        Time::Week => value * W_S_FACTOR,
        Time::Month => value * MO_S_FACTOR,
        Time::Year => value * Y_S_FACTOR,
        Time::Decade => value * DE_S_FACTOR,
        Time::Century => value * C_S_FACTOR,
        Time::Millennium => value * MI_S_FACTOR,
        Time::Unknown => return Err(ConvertError::UnknownUnit(value, (*from).clone().into(), (*to).into())),
    };

    let result = match *to {
        Time::Millisecond => seconds / S_MS_FACTOR,
        Time::Second => seconds,
        Time::Minute => seconds / M_S_FACTOR,
        Time::Hour => seconds / H_S_FACTOR,
        Time::Day => seconds / D_S_FACTOR,
        Time::Week => seconds / W_S_FACTOR,
        Time::Month => seconds / MO_S_FACTOR,
        Time::Year => seconds / Y_S_FACTOR,
        Time::Decade => seconds / DE_S_FACTOR,
        Time::Century => seconds / C_S_FACTOR,
        Time::Millennium => seconds / MI_S_FACTOR,
        Time::Unknown => return Err(ConvertError::UnknownUnit(value, (*from).clone().into(), (*to).into())),
    };

    Ok(result)
}