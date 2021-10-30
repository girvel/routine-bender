use clap::{Parser};
use std::fs::{create_dir_all, OpenOptions};
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::exit;

// TODO restructure code
// TODO add docs

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

    let mut home_path = dirs::home_dir().expect("Can not find home directory");
    home_path.push(".rb");

    create_dir_all(home_path.clone()).expect("Can not create home directory");

    match opts.subcommand {
        SubCommand::Add(add) => {
            let mut generator = names::Generator::default();

            let mut file = loop {
                let mut task_path = home_path.clone();

                {
                    let name = generator.next().unwrap();
                    task_path.push(name);
                }

                match OpenOptions::new().write(true).create_new(true).open(task_path) {
                    Ok(file) => break file,
                    Err(e) => {
                        if e.kind() != io::ErrorKind::AlreadyExists {
                            println!("Something went wrong during creation of a file\n{:?}", e);
                            exit(1);
                        }
                    },
                };
            };

            file.write_all(add.what.as_bytes()).expect("Can not write a file");
        },
    };
}
