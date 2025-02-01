use cli::try_parse_args;
use converter::{convert, ConvertError};
use parser::parse;
use prompts::ask_prompts;

mod cli;
mod converter;
mod prompts;
mod units;
mod parser;

fn main() {
    // try get args
    let args = try_parse_args();
    // if not ask prompts
    let args = ask_prompts(args);
    // parse args 
    let (value, from, to) = parse(args);

    // convert
    let result = convert(value, &from, &to);

    match result {
        Ok(res) => println!("{value} {from} to {to} is {res}"),
        Err(e) => {
            match e {
                ConvertError::UnknownUnit(v, unit1, unit2) => eprint!("Cannot convert {v} from {unit1} to {unit2}"),
                ConvertError::IncompatibleUnits(unit1, unit2) => eprint!("Cannot convert from {unit1} to {unit2}"),
            }
        }
    }
}
