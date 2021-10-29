use clap::{Parser};
use std::fs::OpenOptions;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::exit;

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

fn generate_name() -> String {String::from("task")}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcommand {
        SubCommand::Add(add) => {
            // TODO in home directory
            let mut path_to_task: &Path;
            // create_dir_all(path_to_task.parent().expect(""))
            //     .expect("Can not create data directory for routine-bender");

            let mut file = loop {
                let name = generate_name();
                path_to_task = Path::new(&name);
                match OpenOptions::new().write(true).create_new(true).open(path_to_task) {
                    Ok(file) => break file,
                    Err(e) => {
                        if e.kind() != io::ErrorKind::AlreadyExists {
                            println!("Something went wrong during creation of a file\n{:?}", e);
                            exit(1);
                        }
                    },
                };
            };

            file.write_all(add.what.as_bytes());
        },
    };
}
