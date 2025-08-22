use std::fs;
use crate::VIRTUAL_ROOT;

pub fn list(_args: &str) -> String {
	let files = fs::read_dir(VIRTUAL_ROOT).unwrap();

	let mut res = String::new();
	for file in files {
		res.push_str(&file.unwrap().path().display().to_string());
		res.push('\n');
	}

	return res;
}

pub fn retr(args: &str) -> String {
	return String::from("temp");
}
