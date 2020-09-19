// Most of these come from:
// https://stackoverflow.com/questions/37157926/is-there-a-method-like-javascripts-substr-in-rust

pub trait StringExtensions {
	fn at(&self, pos: usize) -> &str;
	fn from(&self, start: usize) -> &str;
	fn substring(&self, start: usize, len: usize) -> &str;
}

impl StringExtensions for str {
    fn at(&self, pos: usize) -> &str {
        self.substring(pos, 1)
	}
	fn from(&self, pos: usize) -> &str {
        self.substring(pos, self.len() - pos)
    }
	fn substring(&self, start: usize, len: usize) -> &str {
		let mut char_pos = 0;
		let mut byte_start = 0;
		let mut it = self.chars();
		loop {
			if char_pos == start {
				break;
			}
			if let Some(c) = it.next() {
				char_pos += 1;
				byte_start += c.len_utf8();
			} else {
				break;
			}
		}
		char_pos = 0;
		let mut byte_end = byte_start;
		loop {
			if char_pos == len {
				break;
			}
			if let Some(c) = it.next() {
				char_pos += 1;
				byte_end += c.len_utf8();
			} else {
				break;
			}
		}
		&self[byte_start..byte_end]
    }
}

// Tests

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_StringExtensions_at() {
		let s = "abcdèfghij";
		assert_eq!(s.at(0), "a");
		assert_eq!(s.at(2), "c");
		assert_eq!(s.at(4), "è");
		assert_eq!(s.at(9), "j");
		assert_eq!(s.at(100), "");
	}

	#[test]
	fn test_StringExtensions_from() {
		let s = "abcdèfghij";
        assert_eq!(s.from(0), "abcdèfghij");
		assert_eq!(s.from(3), "dèfghij");
		assert_eq!(s.from(8), "ij");
		assert_eq!(s.from(100), "");
	}

	#[test]
	fn test_StringExtensions_substring() {
		let s = "abcdèfghij";
        assert_eq!(s.substring(0, 5), "abcdè");
		assert_eq!(s.substring(0, 50), "abcdèfghij");
		assert_eq!(s.substring(3, 5), "dèfgh");
		assert_eq!(s.substring(3, 50), "dèfghij");
	}
}
