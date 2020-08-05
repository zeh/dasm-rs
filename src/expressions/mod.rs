// FIXME: remove legacy module once it's all translated or moved somewhere else
pub mod legacy;

pub const MAX_OPS: usize = 32;  // In original C code, "MAXOPS"
pub const MAX_ARGS: usize = 64; // In original C code, "MAXARGS"

pub fn is_alpha_num(mut c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c >= '0' && c <= '9'
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

	#[test]
	fn test_is_alpha_num() {
        assert_eq!(is_alpha_num('A'), true);
        assert_eq!(is_alpha_num('X'), true);
        assert_eq!(is_alpha_num('Z'), true);
        assert_eq!(is_alpha_num('a'), true);
        assert_eq!(is_alpha_num('y'), true);
        assert_eq!(is_alpha_num('z'), true);
        assert_eq!(is_alpha_num('0'), true);
        assert_eq!(is_alpha_num('5'), true);
        assert_eq!(is_alpha_num('9'), true);
        assert_eq!(is_alpha_num('-'), false);
        assert_eq!(is_alpha_num('!'), false);
        assert_eq!(is_alpha_num('@'), false);
        assert_eq!(is_alpha_num(' '), false);
        assert_eq!(is_alpha_num('.'), false);
        assert_eq!(is_alpha_num('é'), false);
    }
}
