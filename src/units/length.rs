use strum::{Display, EnumIter};

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