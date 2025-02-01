use dialoguer::{Input, Select};
use strum::IntoEnumIterator;

use crate::{cli::Args, units::{length::Length, units::Units}};

pub fn ask_prompts(args: Args) -> Args {
    let value = args.value.unwrap_or_else(|| {
        Input::new()
            .with_prompt("Enter the numeric value you want to convert")
            .interact_text()
            .unwrap()
    });

    let from = args.from.unwrap_or_else(|| {
        let units= Units::defined_only();

        let idx = Select::new()
            .with_prompt("Enter the unit you want to convert from")
            .items(&units[..])
            .interact()
            .unwrap();

        units[idx].to_string()
    });


    let to = args.to.unwrap_or_else(|| {
        let units = Units::defined_only();

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
        to: Some(to)
    }
}