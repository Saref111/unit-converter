use strum::{Display, EnumIter};

use crate::converter::ConvertError;

use super::{
    consts::{
        G_MG_FACTOR,
        KG_G_FACTOR,
        T_G_FACTOR,
        OZ_G_FACTOR,
        LB_G_FACTOR,
        ST_G_FACTOR,
    }, 
    main::Convertible
};

#[derive(Debug, EnumIter, PartialEq, Display, Clone)]
pub enum Weight {
    Milligram,
    Gram,
    Kilogram,
    MetricTon,
    Ounce,
    Pound,
    Stone,

    Unknown
}

impl From<String> for Weight {
    fn from(value: String) -> Self {
        let l_value = value.to_lowercase();
        let value = l_value.as_str();
        dbg!(value);
        match value {
            "milligram" | "milligrams" | "mg" => Self::Milligram,
            "gram" | "grams" | "g" => Self::Gram,
            "kilogram" | "kilograms" | "kg" => Self::Kilogram,
            "metric ton" | "metric tons" | "ton" | "tons" => Self::MetricTon,
            "ounce" | "ounces" | "oz" => Self::Ounce,
            "pound" | "pounds" | "lb" => Self::Pound,
            "stone" | "stones" | "st" => Self::Stone,

            _ => Self::Unknown,
        }
    }
}

impl Convertible for Weight {
    fn convert(&self, value: f64, to: Box<Weight>) -> Result<f64, ConvertError> {
        convert_length(value, self, to)
    }
}

pub fn convert_length(value: f64, from: &Weight, to: Box<Weight>) -> Result<f64, ConvertError> {
    let grams = match from {
        Weight::Milligram => value / G_MG_FACTOR,
        Weight::Gram => value,
        Weight::Kilogram => value * KG_G_FACTOR,
        Weight::MetricTon => value * T_G_FACTOR,
        Weight::Ounce => value * OZ_G_FACTOR,
        Weight::Pound => value * LB_G_FACTOR,
        Weight::Stone => value * ST_G_FACTOR,
        Weight::Unknown => return Err(ConvertError::UnknownUnit(value, (*from).clone().into(), (*to).into())),
    };

    let result = match *to {
        Weight::Milligram => grams * G_MG_FACTOR,
        Weight::Gram => grams,
        Weight::Kilogram => grams / KG_G_FACTOR,
        Weight::MetricTon => grams / T_G_FACTOR,
        Weight::Ounce => grams / OZ_G_FACTOR,
        Weight::Pound => grams / LB_G_FACTOR,
        Weight::Stone => grams / ST_G_FACTOR,
        Weight::Unknown => return Err(ConvertError::UnknownUnit(value, (*from).clone().into(), (*to).into())),
    };

    Ok(result)
}