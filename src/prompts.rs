use dialoguer::{Input, Select};

use crate::{cli::Args, units::main::Unit};

pub fn ask_prompts(args: Args) -> Args {
    if args.expression.is_some() {
        return args;
    }

    let value = args.value.unwrap_or_else(|| {
        Input::new()
            .with_prompt("Enter the numeric value you want to convert")
            .interact_text()
            .unwrap()
    });

    let from = args.from.unwrap_or_else(|| {
        let units= Unit::defined_only();

        let idx = Select::new()
            .with_prompt("Enter the unit you want to convert from")
            .items(&units[..])
            .interact()
            .unwrap();

        units[idx].to_string()
    });


    let to = args.to.unwrap_or_else(|| {
        let units = Unit::defined_only();

        let idx = Select::new()
            .with_prompt("Enter the unit you want to convert to")
            .items(&units[..])
            .interact()
            .unwrap();
        units[idx].to_string()
    });


    Args {
        value : Some(value),
        from: Some(from),
        to: Some(to),
        expression: None
    }
}