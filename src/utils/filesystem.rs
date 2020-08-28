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
	let file_attempt = try_open_file(filename);
	if file_attempt.is_ok() || is_absolute {
		return file_attempt;
	}

	// Finally, try all include locations
	for location in locations {
		let filename_with_location = combine_paths(location.as_str(), filename);
		let file_location_attempt = try_open_file(filename_with_location.as_str());
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

pub fn write_char_to_file_maybe(maybe_file: &mut Option<File>, char: char) {
	write_buffer_to_file_maybe(maybe_file, &[char as u8]);
}

pub fn get_stream_position(file: &mut File) -> u64 {
	// Equivalent to ftell()
	// FIXME: this requires #![feature(seek_convenience)]; is there a better way?
	file.stream_position().expect("Could not get stream position")
}

pub fn get_stream_position_maybe(maybe_file: &mut Option<File>) -> u64 {
	match maybe_file {
		Some(file) => get_stream_position(file),
		None => 0,
	}
}

pub fn seek_maybe(maybe_file: &mut Option<File>, position: u64) {
	match maybe_file {
		Some(file) => {
			seek(file, position);
		}
		None => {}
	}
}

pub fn seek(file: &mut File, position: u64) {
	file.seek(std::io::SeekFrom::Start(position)).expect("Could not seek file from start");
}

pub fn seek_end_maybe(maybe_file: &mut Option<File>, position: i64) {
	match maybe_file {
		Some(file) => {
			seek_end(file, position);
		}
		None => {}
	}
}

pub fn seek_end(file: &mut File, position: i64) {
	file.seek(std::io::SeekFrom::End(position)).expect("Could not seek file from end");
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
	// FIXME: this is a bit of a weird solution,
	// since Rust doesn't have a classic way of
	// closing a file and attempting to sync_all()
	// on "/dev/null creates an OS error 22
	// ("Invalid argument").
	// Maybe the best solution is to have a Writer
	// instance, but this would change the structure
	// a bit much.
	file.flush().expect("Error closing file");
	file.sync_all().unwrap_or(());
}

pub fn file_exists(filename: &str) -> bool {
	Path::new(filename).exists()
}
