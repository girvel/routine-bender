use clap::{Parser};
use std::fs::{create_dir_all, File, OpenOptions};
use std::io;
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

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcommand {
        SubCommand::Add(add) => {
            let path_to_task = Path::new("/var/rb/task");
            create_dir_all(path_to_task.parent().expect(""))
                .expect("Can not create data directory for routine-bender");

            let mut file = loop {
                match OpenOptions::new().write(true).create_new(true).open(path_to_task) {
                    Ok(file) => break file,
                    Err(io::Error::) => {}
                    _ => { println!("Can not create file for task, sorry."); exit(1) },
                }
            };

            let mut file = File::create("/var/rb/task").expect("Can not open file");
            file.write_all(add.what.as_bytes()).expect("Can not write to file");
        },
    };
}
