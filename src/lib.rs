use typenum;
use std::ffi::OsStr;
use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit},
    Aes256Gcm, Nonce, Key
};
use scrypt::{
    password_hash::{
        rand_core::OsRng,
        PasswordHasher, SaltString
    },
    Scrypt,
    Params
};
use walkdir::WalkDir;
use std::io::Cursor;
use skim::prelude::*;
use chrono;
use std::{
    fs,
    path::{PathBuf, Path},
    process::Command,
    fs::{OpenOptions, File},
    io::Write,
};

pub struct Config {
    pub editor: String,
    pub notes_dir: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            editor: std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string()),
            notes_dir: "Documents_1/notes_1/".to_string(),
        }
    }
}

pub fn encrypt(content: Vec<u8>, key: &Key<Aes256Gcm>) -> Result<(Vec<u8>, Nonce<typenum::U12>), Box<dyn std::error::Error>> {
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, &*content.as_ref())
        .map_err(|e| format!("Encryption failed: {}", e))?;
    Ok((ciphertext, nonce))
}

pub fn decrypt(encrypted_data: Vec<u8>, key: &Key<Aes256Gcm>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    const NONCE_SIZE: usize = 12;
    
    if encrypted_data.len() < NONCE_SIZE {
        println!("Damn size fucked up");
        //return Err(Box::new(format!("Invalid encrypted data size: {}", encrypted_data.len())));
    }
    
    let (ciphertext, nonce_vec) = encrypted_data.split_at(encrypted_data.len() - NONCE_SIZE);
    let nonce = Nonce::from_slice(nonce_vec);
    let cipher = Aes256Gcm::new(key);

    let decrypted_content = cipher.decrypt(nonce, &*ciphertext)
        .map_err(|e| format!("Decryption failed: {}", e))?;

    Ok(decrypted_content)
}

pub fn derive_key(hash: Vec<u8>, salt: Vec<u8>) -> Result<Key<Aes256Gcm>, Box<dyn std::error::Error>> {
    let params: Params = Params::new(15, 8, 1, 32)?;

    let hash_str = String::from_utf8(hash)?;
    let salt_str = String::from_utf8(salt)?;

    let mut key_bytes = vec![0u8; 32];
    
    scrypt::scrypt(
        hash_str.as_bytes(),
        salt_str.as_bytes(),
        &params,
        &mut key_bytes
    )?;

    let key = Key::<Aes256Gcm>::from_slice(&key_bytes);

    Ok(*key)
}

pub fn generate_hash_and_salt() -> Result<(), Box<dyn std::error::Error>> {
    let binding = rprompt::prompt_reply("Enter a password to be hashed: ").unwrap();
    let password = binding.as_bytes();
    let salt = SaltString::generate(&mut OsRng);

    let password_hash = Scrypt.hash_password(password, &salt)?;

    //fs::write("hash.txt", password_hash.to_string())?;
    //fs::write("salt.txt", salt.to_string())?;

    Ok(())
}

pub fn main_entry(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
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

pub fn create_entry(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
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

    let year: String = chrono::offset::Local::now()
        .format("%Y")
        .to_string()
        .to_owned();

    let month: String = chrono::offset::Local::now()
        .format("%B")
        .to_string()
        .to_lowercase()
        .to_owned();

    let month_n: String = chrono::offset::Local::now()
        .format("%m")
        .to_string()
        .to_owned();

    let mut tmp_path: PathBuf = dirs::home_dir().expect("Can't find your home directory.");
        tmp_path.push(&config.notes_dir);
        tmp_path.push("journal");
        tmp_path.push(format!("{}/{}.{}/{}", year, month_n, month, file));

    let path = tmp_path.to_str().expect("Path has invalid stuff!").to_string();

    if let Some(parent) = Path::new(&path).parent() {
        fs::create_dir_all(parent)?;
    }

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

pub fn find(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
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

    let skim_output = match Skim::run_with(&options, Some(items)) {
        Some(output) => output,
        None => return Ok(()),
    };

    if skim_output.is_abort {
        return Ok(());
    }

    let selected_items = skim_output.selected_items;

    if selected_items.is_empty() {
        return Ok(());
    }

    for item in selected_items.iter() {
        let output: &str = &item.output().to_string();
        let chosen_file: &OsStr = OsStr::new(output);
        notes_dir.push(chosen_file);

        Command::new(&config.editor)
            .arg(&notes_dir)
            .status()
            .expect("Can't open your editor!");
    }

    Ok(())
}

pub fn help() -> Result<(), Box<dyn std::error::Error>> {
    let ascii_art = "
  _          __ 
 | | _____  / _|
 | |/ / _ \\| |_ 
 |   < (_) |  _|
 |_|\\_\\___/|_|  
                ";

    let help_message = format!(
        "{}\n\n{} {}\n\n{}\n    {}    {}\n    {}      {}\n    {}      {}\n    {}      {}\n    {}      {}\n\n{}\n    {}        {}\n    {}        {}\n    {}        {}\n\n{}\n    {}          {}",
        ascii_art,
        "\x1b[1mUsage:\x1b[0m", // Bold
        "notes [OPTIONS]",
        "\x1b[1mOptions:\x1b[0m", // Bold
        "-c, --create",
        "Create a new note for today in the journal",
        "-m, --main",
        "Open the main notes file",
        "-f, --find",
        "Search and find notes interactively",
        "-h, --help",
        "Print help",
        "-V, --version",
        "Print version",
        "\x1b[1mExamples:\x1b[0m", // Bold
        "notes -m",
        "Opens the main notes file",
        "notes -c",
        "Creates a new journal entry for today",
        "notes -f",
        "Find and open existing notes",
        "\x1b[1mEnvironment:\x1b[0m", // Bold
        "EDITOR",
        "Specify the text editor to use (default: nano)"
    );

    println!("{}", help_message);

    Ok(())
}

//fn sync(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
//    let path = Path::new("")
//    if condition {
//        todo!();
//    }
//    Ok(())
//}
