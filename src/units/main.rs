use std::{fmt::Display, vec::IntoIter};

use strum::IntoEnumIterator;
use crate::converter::ConvertError;

use super::{length::Length, time::Time, weight::Weight};

#[derive(Debug, Clone)]
pub enum Unit {
    Time(Time),
    Length(Length),
    Weight(Weight),

    Unknown
}

impl Display for Unit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Unit::Time(time) => write!(f, "{}", time),
            Unit::Length(length) => write!(f, "{}", length),
            Unit::Weight(weight) => write!(f, "{}", weight),
            Unit::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Unit {
    pub fn into_iter() -> IntoIter<Box<Self>> {
        let units = vec![
            Time::iter().map(|x| Box::new(Unit::Time(x))).collect::<Vec<_>>(),
            Length::iter().map(|x| Box::new(Unit::Length(x))).collect::<Vec<_>>(),
            Weight::iter().map(|x| Box::new(Unit::Weight(x))).collect::<Vec<_>>(),
        ].concat();
        units.into_iter()
    }

    pub fn defined_only() -> Vec<Box<Self>> {
        Self::into_iter()
            .filter(|it| match **it {
                Unit::Time(Time::Unknown) => false,
                Unit::Length(Length::Unknown) => false,
                Unit::Weight(Weight::Unknown) => false,
                Unit::Unknown => false,
                _ => true,
            })
            .collect()
    }
}

impl From<String> for Unit {
    fn from(value: String) -> Self {
        let result = Time::from(value.to_owned());

        if result != Time::Unknown {
            return Unit::Time(result);
        }

        let result = Length::from(value.to_owned());

        if result != Length::Unknown {
            return Unit::Length(result);
        }

        let result = Weight::from(value.to_owned());

        if result != Weight::Unknown {
            return Unit::Weight(result);
        }

        Unit::Unknown
    }
}

impl From<Time> for Unit {
    fn from(value: Time) -> Self {
        Unit::Time(value)
    }
}


impl From<Length> for Unit {
    fn from(value: Length) -> Self {
        Unit::Length(value)
    }
}

impl From<Weight> for Unit {
    fn from(value: Weight) -> Self {
        Unit::Weight(value)
    }
}

pub trait Convertible {
    fn convert(&self, value: f64, to: Box<Self>) -> Result<f64, ConvertError>;
}
