use clap::Parser;
use std::ffi::OsStr;
use walkdir::WalkDir;
use std::io::Cursor;
use skim::prelude::*;
use chrono;
use std::{
    path::{PathBuf, Path},
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

    #[arg(short, long)]
    find: bool,
}

#[derive(Debug)]
struct Config {
    editor: String,
    notes_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: var("EDITOR").unwrap_or_else(|_| "nano".to_string()),
            notes_dir: "Documents_1/notes_1/".to_string(),
        }
    }
}

fn main() {
    let args = Args::parse();
    let config = Config::default();
   
    let _ = match (args.main, args.create, args.find) {
        (true, false, false) => main_entry(&config),
        (false, true, false) => create_entry(&config),
        (false, false, true) => find(&config),
        _ => todo!(),
    };
}

fn main_entry(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut tmp_path: PathBuf = dirs::home_dir().expect("Can't find your home directory.");
        tmp_path.push(&config.notes_dir);
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

fn create_entry(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut file: String = chrono::offset::Local::now()
        .format("%Y-%m-%d")
        .to_string()
        .to_owned();
        file.push_str(".md");

    let time: String = chrono::offset::Local::now()
        .format("%H:%M:%S")
        .to_string()
        .to_owned();

    let today: String = chrono::offset::Local::now()
        .format("%A %B %d")
        .to_string()
        .to_owned();

    let mut tmp_path: PathBuf = dirs::home_dir().expect("Can't find your home directory.");
        tmp_path.push(&config.notes_dir);
        tmp_path.push("journal/");
        tmp_path.push(file);

    let path = tmp_path
        .to_str()
        .expect("Path has invalid stuff!")
        .to_string();

    match Path::new(&path).exists() {
        true => {
            let mut file = OpenOptions::new()
                .append(true)
                .open(&path)?;
            writeln!(file, "\n## {}\n", time)?;
        }
        false => {
            let mut file = File::create(&path)?;
            writeln!(file, "# {}\n\n## {}", today, time)?;
        }
    }

    Command::new(&config.editor)
        .arg(&path)
        .status()
        .expect("Something went wrong");

    Ok(())
}

fn find(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let mut notes_dir = dirs::home_dir().expect("Can't find your home directory");
    notes_dir.push(&config.notes_dir);

    let options = SkimOptions::default();

    let files: Vec<String> = WalkDir::new(&notes_dir)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.file_type().is_file())
        .map(|entry| entry.path().strip_prefix(&notes_dir).unwrap().to_string_lossy().into_owned())
        .collect();

    let item_reader = SkimItemReader::default();
    let items = item_reader.of_bufread(Cursor::new(files.join("\n")));

    let selected_items = Skim::run_with(&options, Some(items))
        .map(|out| out.selected_items)
        .unwrap_or_else(Vec::new);

    for item in selected_items.iter() {
        let output: &str = &item.output().to_string();
        let chosen_file: &OsStr = OsStr::new(output);
        notes_dir.push(chosen_file);

        Command::new(&config.editor)
            .arg(&notes_dir)
            .status()
            .expect("Can't open your editor");
    }

    Ok(())
}
