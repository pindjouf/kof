use clap::Parser;
use chrono;
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

#[derive(Debug)]
struct Config {
    editor: String
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: var("EDITOR").unwrap_or_else(|_| "nano".to_string()),
        }
    }
}

fn main() {
    let args = Args::parse();
    let config = Config::default();
   
    let _ = match (args.main, args.create) {
        (true, false) => main_entry(&config),
        //(false, true) => create_entry(),
        //(false, false) => println!("You didn't use any flags try --help"),
        _ => todo!(),
    };
}

fn main_entry(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut tmp_path: PathBuf = dirs::home_dir().expect("Can't find your home directory.");
        tmp_path.push("Documents_1");
        tmp_path.push("notes_1");
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

    Command::new(&config.editor)
        .arg(&path)
        .status()
        .expect("Something went wrong");

    Ok(())
}

//fn create_entry() -> Result<(), Box<dyn std::error::Error>> {
//    let mut date: String = chrono::offset::Local::now().format("%Y-%m-%d").to_string().to_owned();
//    let file_ext: &str = ".md";
//
//    date.push_str(file_ext);
//    println!("{}", date);
//
//    let time: String = chrono::offset::Local::now().format("%H:%M:%S").to_string().to_owned();
//    println!("{}", time);
//
//    let file = OpenOptions::new()
//        .write(true)
//        .create(true)
//        .open(&path)?;
//
//    if file.metadata()?.len() == 0 {
//        let mut file = File::create(&date)?;
//        writeln!(file, time"\n\n")?;
//
//    Ok(())
//}

//fn fzf() -> Result<(), Box<dyn std::error::Error>> {
//
//}
