use chrono::prelude::*;
use std::env;

pub fn logthis(msg: &str, cat: &str) {
	println!("{} [{}] {}", Utc::now().format("%Y-%m-%d %H:%M:%S"), cat, msg);
}
 
//pub fn remove_whitespace(s: &str) -> String {
//    s.replace(|c: char| !c.is_ascii(), "").chars().filter(|c| !c.is_whitespace()).collect()
//}

pub fn get_current_working_dir() -> String {
    env::current_dir().unwrap().to_str().unwrap().to_string()
}
