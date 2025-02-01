use strum::{Display, EnumIter};

use super::{consts::{
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
}, units::Convertible};

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
    fn convert(&self, value: f64, to: Box<Time>) -> f64 {
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

pub fn convert_time(value: f64, from: &Time, to: Box<Time>) -> f64 {
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
        Time::Unknown => return value,
    };

    match *to {
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
        Time::Unknown => seconds,
    }
}