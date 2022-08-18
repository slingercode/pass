use crate::dir::{generate_dir, generate_path, read_file, verify_file, write_file};

pub fn init() {
    generate_dir();
}

pub fn set(key: String, value: String) {
    let path = generate_path(key).unwrap();
    println!("{}", path.display());

    if verify_file(&path) {
        println!("File already exists");
        return;
    }

    // TODO: Encrypt data

    write_file(&path, value).unwrap();
    println!("Password stored");
}

pub fn get(key: String) -> String {
    let path = generate_path(key).unwrap();

    if let Some(value) = read_file(&path) {
        return value;
    }

    // TODO: Decrypt data

    return String::from("");
}
