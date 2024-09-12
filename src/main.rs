use clap::Parser;
use std::{
    path::PathBuf,
    env::var,
    process::Command,
    fs::{OpenOptions, File},
    io::Write,
};

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
   
    let _ = match (args.main, args.create) {
        (true, false) => main_entry(),
        //(false, true) => println!("You chose the --create flag"),
        //(false, false) => println!("You didn't use any flags try --help"),
        _ => todo!(),
    };
}

fn main_entry() -> Result<(), Box<dyn std::error::Error>> {
    let editor = var("EDITOR").unwrap();

    let mut tmp_path: PathBuf = dirs::home_dir().expect("Can't find your home directory.");
    tmp_path.push("Documents");
    tmp_path.push("notes");
    tmp_path.push("main_1.md");

    let path = tmp_path
        .to_str()
        .expect("Path has invalid stuff!")
        .to_string();
    
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&path)?;

    if file.metadata()?.len() == 0 {
        let mut file = File::create(&path)?;
        writeln!(file, "# Things to keep in mind\n\n")?;
    }

    Command::new(editor).arg(&path).status().expect("Something went wrong");

    Ok(())
}

//fn create_entry() {
//
//}
