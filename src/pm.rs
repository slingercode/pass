use std::fs::{create_dir, self};

pub fn init() {
  match home::home_dir() {
    Some(home) => {
      let path = &home.join(".pm");

      if let Err(error) = create_dir(path) {
        panic!("Could not create .pm directory: {}", error);
      }
      
      println!("Initializated in path {}", path.display());
    }
    None => { panic!("Imposible to get your home directory") }
  }
}

pub fn set(key: String, value: String) {
  if let Some(home) = home::home_dir() {
    let path = &home.join(".pm").join(key);

    if let Err(error) = fs::write(path, value) {
      panic!("{}", error);
    }

    println!("Document created!");
  }
}

pub fn get(key: String) -> String {
  key
}
