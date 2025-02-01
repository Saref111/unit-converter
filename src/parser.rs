use crate::cli::Args;
use crate::units::main::Unit;

pub fn parse(args: Args) -> (f64, Unit, Unit) {
    let value = args.value.unwrap().parse::<f64>().unwrap();
    let from: Unit = args.from.unwrap().into();
    let to: Unit = args.to.unwrap().into();

    (value, from, to)
}