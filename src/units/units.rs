use std::{fmt::Display, vec::IntoIter};

use strum::IntoEnumIterator;
use super::{length::Length, time::Time};

#[derive(Debug, Clone)]
pub enum Units {
    Time(Time),
    Length(Length),
}

impl Display for Units {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Units::Time(time) => write!(f, "{}", time),
            Units::Length(length) => write!(f, "{}", length),
        }
    }
}

impl Units {
    pub fn into_iter() -> IntoIter<Box<Self>> {
        let units = vec![
            Time::iter().map(|x| Box::new(Units::Time(x))).collect::<Vec<_>>(),
            Length::iter().map(|x| Box::new(Units::Length(x))).collect::<Vec<_>>(),
        ].concat();
        units.into_iter()
    }

    pub fn defined_only() -> Vec<Box<Self>> {
        Units::into_iter()
            .filter(|it| match **it {
                Units::Time(Time::Unknown) => false,
                Units::Length(Length::Unknown) => false,
                _ => true,
            })
            .collect()
    }
}

pub trait Convertible {
    fn convert(&self, value: f64, to: Box<Self>) -> f64;
}
