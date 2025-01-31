use cli::try_parse_args;
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
    // convert value
}
