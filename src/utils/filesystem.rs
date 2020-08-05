use std::fs::File;
use std::io::prelude::*;

pub fn create_new_file(filename: &str) -> std::io::Result<File> {
	File::create(filename)
}

pub fn write_buffer_to_file_maybe(maybe_file: &mut Option<File>, buffer: &[u8]) {
	match maybe_file {
		Some(file) => {
            file.write_all(buffer).expect("Error writing to file");
		}
		None => {}
	}
}

pub fn write_to_file_maybe(maybe_file: &mut Option<File>, message: &str) {
	write_buffer_to_file_maybe(maybe_file, message.as_bytes());
}

pub fn writeln_to_file_maybe(maybe_file: &mut Option<File>, message: &str) {
	write_to_file_maybe(maybe_file, &([message, "\n"]).concat());
}

pub fn close_file_maybe(maybe_file: &mut Option<File>) {
	match maybe_file {
		Some(file) => {
            file.sync_all().expect("Error closing file");
		}
		None => {}
	}
}
