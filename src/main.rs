mod command_handlers;

use std::collections::HashMap;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

use once_cell::sync::Lazy;

use crate::command_handlers::{list, retr};

const BUFFER_SIZE: usize = 1024;
const SERVER_ADDRESS: &str = "127.0.0.1";
const SERVER_PORT: u32 = 21;
pub const VIRTUAL_ROOT: &str = "data";

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

fn process_command(mut stream: TcpStream) {
	let peer = stream.try_clone().unwrap();
	let mut reader = BufReader::with_capacity(BUFFER_SIZE, peer);

	loop {
		let mut line = String::new();
		let n = reader.read_line(&mut line).expect(errors::READ_FAILED);
		if n == 0 {
			break;
		}

		let mut parts = line.splitn(2, ' ');
		let command = parts.next().unwrap_or("").trim_matches('\0').trim();
		let args = parts.next().unwrap_or("").trim_matches('\0').trim();
		std::mem::drop(parts);

		let response = match COMMAND_MAP.get(command) {
			Some(handler) => handler(args),
			None => String::from(errors::INVALID_COMMAND),
		};

		stream
			.write_all(response.as_bytes())
			.expect(errors::WRITE_FAILED);
	}
}
