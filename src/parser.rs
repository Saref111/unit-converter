use crate::cli::Args;

use crate::units::length::Length;

pub fn parse(args: Args) -> (f64, Length, Length) {
    let value = args.value.unwrap().parse::<f64>().unwrap();
    let from: Length = args.from.unwrap().into();
    let to: Length = args.to.unwrap().into();

    (value, from, to)
}