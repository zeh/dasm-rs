use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn create_new_file(filename: &str) -> std::io::Result<File> {
	File::create(filename)
}

pub fn try_open_file(filename: &str) -> std::io::Result<File> {
	File::open(filename)
}

pub fn open_file(filename: &str) -> File {
	try_open_file(filename).expect(format!("Error opening file {}", filename).as_str())
}

pub fn try_open_file_with_locations(filename: &str, locations: &Vec<String>) -> std::io::Result<File> {
	// We won't use the include dir list for absolute pathnames.
	// This is probably not entirely correct (should it use is_absolute()?),
	// but is probably right in the context and follows the orginal dasm
	// implementation.
	let is_absolute = filename.contains(":");

	// Try direct file first. Returns if it exists, or if it's absolute,
	// return whatever the result is since it ignores includes.
	let mut file_attempt = try_open_file(filename);
	if file_attempt.is_ok() || is_absolute {
		return file_attempt;
	}

	// Finally, try all include locations
	for location in locations {
		let mut filename_with_location = combine_paths(location.as_str(), filename);
		let mut file_location_attempt = try_open_file(filename_with_location.as_str());
		if file_location_attempt.is_ok() {
			return file_location_attempt;
		}
	}

	// With all said and done and no file found, fall back to the original result error
	file_attempt
}

/**
 * Concatenate two paths, adding a forward slash between them if needed.
 * This is used to concatenate include dirs and include files when trying
 * to find includes, and follows behavior of the original addpart() in ops.c.
 */
pub fn combine_paths(a: &str, b: &str) -> String {
	let mut path = String::new();
	path.push_str(a);
	if !a.ends_with(":") && !a.ends_with("/") {
		path.push_str("/");
	}
	path.push_str(b);
	path
}

pub fn write_buffer_to_file_maybe(maybe_file: &mut Option<File>, buffer: &[u8]) {
	match maybe_file {
		Some(file) => {
			write_buffer_to_file(file, buffer);
		}
		None => {}
	}
}

pub fn write_buffer_to_file(file: &mut File, buffer: &[u8]) {
	file.write_all(buffer).expect("Error writing to file");
}

pub fn write_to_file_maybe(maybe_file: &mut Option<File>, message: &str) {
	write_buffer_to_file_maybe(maybe_file, message.as_bytes());
}

pub fn write_to_file(file: &mut File, message: &str) {
	write_buffer_to_file(file, message.as_bytes());
}

pub fn writeln_to_file_maybe(maybe_file: &mut Option<File>, message: &str) {
	write_to_file_maybe(maybe_file, &([message, "\n"]).concat());
}

pub fn writeln_to_file(file: &mut File, message: &str) {
	write_to_file(file, &([message, "\n"]).concat());
}

pub fn close_file_maybe(maybe_file: &mut Option<File>) {
	match maybe_file {
		Some(file) => {
			close_file(file);
		}
		None => {}
	}
}

pub fn close_file(file: &mut File) {
	file.sync_all().expect("Error closing file");
}

pub fn file_exists(filename: &str) -> bool {
	Path::new(filename).exists()
}
