use kof::*;
use clap::{Parser, Command};
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    create: bool,

    #[arg(short, long)]
    main: bool,

    #[arg(short, long)]
    find: bool,
}

fn main() {
    let args = Args::parse();
    let config = Config::default();
    let mut home: PathBuf = dirs::home_dir().expect("Can't find your home directory");
    home.push(&config.notes_dir);

    let _: Command = match Path::new(&home).exists() {
        true => Command::new("command"),
        false => Command::new("mkdir -p $HOME/Documents_1/notes_1;"),
    };
   
    let _ = match (args.main, args.create, args.find) {
        (true, false, false) => main_entry(&config),
        (false, true, false) => create_entry(&config),
        (false, false, true) => find(&config),
        _ => help(),
    };
}
