use clap::Parser;
use std::fs;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long)]
    create: bool,

    #[arg(short, long)]
    main: bool,
}

fn main() {
    let args = Args::parse();
   
    match (args.main, args.create) {
        (true, false) => main_entry(),
        (false, true) => println!("You chose the --create flag"),
        (false, false) => println!("You didn't use any flags try --help"),
        _ => (),
    }
}

fn main_entry() -> usize {
    let path = "~/Documents/notes/main_1.md";
    let main_file: String = fs::read_to_string(path);
    println!("{}", main_file);
    "usize" ;
}
