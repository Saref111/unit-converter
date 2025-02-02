use crate::units::main::{
    Convertible, 
    Unit
};


pub enum ConvertError {
    UnknownUnit(f64, Unit, Unit),
    IncompatibleUnits(Unit, Unit)
}

pub fn convert(value: f64, from: &Unit, to: &Unit) -> Result<f64, ConvertError> {
    match from {
        Unit::Time(ref time) => {
            let Unit::Time(to_time) = to else {
                return Err(ConvertError::IncompatibleUnits(from.clone(), to.clone()));
            };

            time.convert(value, to_time.clone().into())
        },
        Unit::Length(ref length) => {
            let Unit::Length(to_length) = to else {
                return Err(ConvertError::IncompatibleUnits(from.clone(), to.clone()));
            };

            length.convert(value, to_length.clone().into())
        },
        Unit::Weight(ref weight) => {
            let Unit::Weight(to_weight) = to else {
                return Err(ConvertError::IncompatibleUnits(from.clone(), to.clone()));
            };

            weight.convert(value, to_weight.clone().into())
        },
        Unit::Unknown => Err(ConvertError::UnknownUnit(value, from.clone(), to.clone())),
    }
}