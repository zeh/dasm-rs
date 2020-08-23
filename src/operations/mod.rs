// FIXME: remove legacy module once it's all translated or moved somewhere else
pub mod legacy;

pub fn clear_passbuffer(passbuffer: &mut Vec<String>) {
	passbuffer.clear();
}

pub fn update_passbuffer(passbuffer: &mut Vec<String>, message: &str) {
	passbuffer.push(String::from(message));
}

pub fn output_passbuffer(passbuffer: &mut Vec<String>) {
	// Do we really still need to put this through stdout, instead of stderr?
	// FIXME: rather than adding "\n" to the message (inside asmerr() then
	// concatenating without newlines here, we should probably have the error
	// messages as-is and then add "\n" here, before display. But this disrupts
	// rendering messages on stdout and list files too, so it will need some
	// more work.
	println!("{}", passbuffer.join(""));
}
