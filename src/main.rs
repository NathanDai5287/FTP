use std::collections::HashMap;
use std::{fs, thread};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

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

fn process_command(mut stream: TcpStream, command_map: &HashMap<&str, fn(&str) -> String>) {
	let mut buffer = [0; BUFFER_SIZE];
	stream.read(&mut buffer).expect(errors::READ_FAILED);

	let request = String::from_utf8_lossy(&buffer[..]);

	let mut parts = request.splitn(2, ' ');
	let command = parts.next().unwrap_or("");
	let args = parts.next().unwrap_or("");
	std::mem::drop(parts);

	let f = command_map.get(command);

	let response = match f {
		Some(f) => f(args),
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
				std::thread::spawn(move || process_command(stream, &command_map));
			}
			Err(error) => {
				eprintln!("{}", format!("{}: {}", errors::CONN_FAILED, error));
			}
		}
	}
}
