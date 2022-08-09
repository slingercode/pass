use std::fs::create_dir;

pub fn set(key: String, value: String) {
  match home::home_dir() {
    Some(path) => {
      if let Err(e) = create_dir(path.join(".pm")) {
        println!("Could not create .pm directory: {}", e)
      }

      println!("Path{}", path.join(".pm").display());
      println!("Key: {}, Value: {}", key, value);
    }
    None => { panic!("Impossible to get your home directory"); }
  }
}