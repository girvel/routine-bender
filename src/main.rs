use clap::{Parser};

#[derive(Parser)]
#[clap(version = "0.1", author = "Nikita Dobrynin / girvel <widauka@ya.ru>")]
struct Opts {
    #[clap(subcommand)]
    subcommand: SubCommand,
}

#[derive(Parser)]
enum SubCommand {
    Add(Add),
}

#[derive(Parser)]
struct Add {
    what: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let SubCommand::Add(a) = opts.subcommand;
    println!("{}", a.what)
}
