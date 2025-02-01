use cli::try_parse_args;
use parser::parse;
use prompts::ask_prompts;
use units::length::{convert_length, Length};

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
    convert_length(value, from, to);
}
