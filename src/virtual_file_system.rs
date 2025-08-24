use std::path::{Path, PathBuf};

pub struct SessionState {
	pub root: PathBuf,
	pub cwd: PathBuf,
}

impl SessionState {
	pub fn new(root: impl AsRef<Path>) -> Self {
		let root = root.as_ref().canonicalize().unwrap();
		return Self {
			root: root.clone(),
			cwd: root.clone(),
		};
	}
}
