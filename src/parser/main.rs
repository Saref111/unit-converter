use crate::{cli::Args, units::main::Unit};

pub fn parse(args: Args) -> (f64, Unit, Unit) {

    if args.expression.is_none() {
        let value = args.value.unwrap().parse::<f64>().unwrap();
        let from: Unit = args.from.unwrap().into();
        let to: Unit = args.to.unwrap().into();

        return (value, from, to);
    }

    let expression = args.expression.unwrap();
}