use std::collections::HashMap;
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use once_cell::sync::Lazy;

const BUFFER_SIZE: usize = 1024;
const SERVER_ADDRESS: &str = "127.0.0.1";
const SERVER_PORT: u32 = 21;
const VIRTUAL_ROOT: &str = "data";

mod errors {
	pub const BIND_FAILED: &str = "Failed to bind to address";
	pub const READ_FAILED: &str = "Failed to read from client";
	pub const WRITE_FAILED: &str = "Failed to write response";
	pub const CONN_FAILED: &str = "Failed to establish connection";

	pub const INVALID_COMMAND: &str = "Command not recognized";
}

type CommandHandler = fn(&str) -> String;
type CommandMap = HashMap<&'static str, CommandHandler>;

static COMMAND_MAP: Lazy<CommandMap> = Lazy::new(|| {
	let mut command_map = HashMap::new();
	command_map.insert("list", list as CommandHandler);
	command_map.insert("retr", retr as CommandHandler);

	return command_map;
});

fn process_command(mut stream: TcpStream) {
	let mut buffer = [0; BUFFER_SIZE];
	stream.read(&mut buffer).expect(errors::READ_FAILED);

	let request = String::from_utf8_lossy(&buffer[..]);

	let mut parts = request.splitn(2, ' ');
	let command = parts.next().unwrap_or("").trim_matches('\0').trim();
	let args = parts.next().unwrap_or("").trim_matches('\0').trim();
	std::mem::drop(parts);

	let response = match COMMAND_MAP.get(command) {
		Some(handler) => handler(args),
		None => String::from(errors::INVALID_COMMAND),
	};

	stream
		.write(response.as_bytes())
		.expect(errors::WRITE_FAILED);
}

fn list(_args: &str) -> String {
	let files = fs::read_dir(VIRTUAL_ROOT).unwrap();

	let mut res = String::new();
	for file in files {
		res.push_str(&file.unwrap().path().display().to_string());
		res.push('\n');
	}

	return res;
}

fn retr(args: &str) -> String {
	return String::from("temp");
}

fn main() {
	let mut command_map: HashMap<&str, fn(&str) -> String> = HashMap::new();
	command_map.insert("list", list);
	command_map.insert("retr", retr);

	let full_address = format!("{SERVER_ADDRESS}:{SERVER_PORT}");
	let listener = TcpListener::bind(&full_address).expect(errors::BIND_FAILED);
	println!("Server listening on {full_address}");

	for stream in listener.incoming() {
		match stream {
			Ok(stream) => {
				std::thread::spawn(move || process_command(stream));
			}
			Err(error) => {
				eprintln!("{}", format!("{}: {}", errors::CONN_FAILED, error));
			}
		}
	}
}
