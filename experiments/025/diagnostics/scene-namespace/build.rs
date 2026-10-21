fn main() {assert!(std::process::Command::new("python").args(["-B","observe.py",&std::env::var("OUT_DIR").unwrap()]).status().unwrap().success());}
