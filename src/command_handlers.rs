use std::fs;

use super::virtual_file_system::SessionState;

// FILES

pub fn list(session: &mut SessionState, _args: &str) -> String {
	let files = fs::read_dir(&session.cwd).unwrap();

	let mut res = String::new();
	for file in files {
		let path = file.unwrap().path();
		let relative_path = path.strip_prefix(&session.root).unwrap();

		res.push_str(&relative_path.to_string_lossy().replace('\\', "/"));
		res.push('\n');
	}

	return res;
}

pub fn retr(session: &mut SessionState, args: &str) -> String {
	return String::from("temp");
}

// DIRECTORIES

pub fn pwd(session: &mut SessionState, _args: &str) -> String {
	return "/".to_string()
		+ &session
			.cwd
			.strip_prefix(&session.root)
			.unwrap()
			.to_string_lossy()
			.replace('\\', "/");
}
