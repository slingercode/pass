use std::{
    fs::{self, create_dir},
    io,
    path::{self, PathBuf},
};

pub fn generate_dir() {
    match home::home_dir() {
        Some(home) => {
            let path = &home.join(".pm");

            if let Err(error) = create_dir(path) {
                println!("Could not create .pm directory: {}", error);
                return;
            }

            println!("Initializated in path {}", path.display());
        }
        None => {
            println!("Imposible to get your home directory")
        }
    }
}

pub fn generate_path(file_name: String) -> Option<PathBuf> {
    let mut path = home::home_dir()?;

    path.push(".pm");
    path.push(file_name);
    path.set_extension("pgp");

    return Some(path);
}

pub fn verify_file(path: &PathBuf) -> bool {
    path::Path::new(path).exists()
}

pub fn write_file(path: &PathBuf, contents: String) -> io::Result<()> {
    return fs::write(path, contents);
}

pub fn read_file(path: &PathBuf) -> Option<String> {
    match fs::read_to_string(path) {
        Ok(value) => Some(value),
        Err(error) => {
            println!("Error reading file: {}", error);
            None
        }
    }
}
