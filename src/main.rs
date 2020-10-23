#![windows_subsystem = "windows"]

extern crate sciter;

use std::env;

fn main() {
	let mut frame = sciter::Window::new();
	let dir = env::current_dir().unwrap().as_path().display().to_string();
	let filename = format!("{}\\{}", dir, "index.htm");
	frame.load_file(&filename);
	frame.run_app();
}
