use clap::Parser;

#[derive(Parser)]
struct Cli {
    value: Option<String>,

    #[arg(short, long)]
    from: Option<String>,
    

    #[arg(short, long)]
    to: Option<String>,

    expression: Option<String>,
}

pub struct Args {
    pub value: Option<String>,
    pub from: Option<String>,
    pub to: Option<String>,
    pub expression: Option<String>,
}

pub fn try_parse_args() -> Args {
    let args = Cli::parse();

    Args {
        value: args.value,
        from: args.from,
        to: args.to,
        expression: args.expression,
    }
}