use crate::types::enums::AsmErrorEquates;

// value, flags, lastWasOp
pub type ResultValues = (i64, i32, bool);
pub type ResultOfOperation = Result<ResultValues, AsmErrorEquates>;
pub type ExpressionOperationFunc = fn(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation;

pub fn noop(_v1: i64, _v2: i64, _f1: i32, _f2: i32) -> ResultOfOperation {
	Ok((0, 0, false))
}

pub fn take_least_significant_byte(v1: i64, _v2: i64, f1: i32, _f2: i32) -> ResultOfOperation {
	Ok((v1 & 0xff as i64, f1, false))
}

pub fn take_most_significant_byte(v1: i64, _v2: i64, f1: i32, _f2: i32)-> ResultOfOperation {
	Ok((v1 >> 8 & 0xff, f1, false))
}

pub fn negate(v1: i64, _v2: i64, f1: i32, _f2: i32)-> ResultOfOperation {
	Ok((-v1, f1, false))
}

pub fn invert(v1: i64, _v2: i64, f1: i32, _f2: i32)-> ResultOfOperation {
	Ok((!v1, f1, false))
}

pub fn not(v1: i64, _v2: i64, f1: i32, _f2: i32)-> ResultOfOperation {
	Ok(((v1 == 0) as i64, f1, false))
}

pub fn multiply(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	Ok((v1 * v2, f1 | f2, true))
}

pub fn divide(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	// Conversion note: in the original C code, this sets lastWasOp *before* stackarg() executes,
	// which means it's ignored, since it's reset to "false" inside stackarg(). I believe this is
	// a bug in the original code (since multiply, adding, and subtracting *do* set it to "true")
	// so I opted to returning "true" here, rather than the "false" that would mimic the original
	// code.
	if f1 | f2 != 0 {
		Ok((0, f1 | f2, true))
	} else if v2 == 0 {
 		Err(AsmErrorEquates::DivisionByZero)
	} else {
		Ok((v1 / v2, 0, true))
	}
}

pub fn modulo(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	if f1 | f2 != 0 {
		// Conversion note: in the original, only this path doesn't enable lastWasOp, but I believe this is a bug.
		Ok((0, f1 | f2, true))
	} else if v2 == 0 {
		Ok((v1, 0, true))
	} else {
		Ok((v1 % v2, 0, true))
	}
}

pub fn question(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	if f1 != 0 {
		Ok((0, f1, false))
	} else {
		Ok((
			if v1 != 0 {
				v2
			} else {
				0
			},
			if v1 != 0 {
				f2
			} else {
				0
			},
			false,
		))
	}
}

pub fn add(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	Ok((v1 + v2, f1 | f2, true))
}

pub fn subtract(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	Ok((v1 - v2, f1 | f2, true))
}

pub fn shift_right(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	if f1 | f2 != 0 {
		Ok((0, f1 | f2, false))
	} else {
		Ok((v1 >> v2, 0, false))
	}
}

pub fn shift_left(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	if f1 | f2 != 0 {
		Ok((0, f1 | f2, false))
	} else {
		Ok((v1 << v2, 0, false))
	}
}

pub fn greater(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	Ok(((v1 > v2) as i32 as i64, f1 | f2, false))
}

pub fn greater_or_equal(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	Ok(((v1 >= v2) as i32 as i64, f1 | f2, false))
}

pub fn lesser(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	Ok(((v1 < v2) as i32 as i64, f1 | f2, false))
}

pub fn lesser_or_equal(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	Ok(((v1 <= v2) as i32 as i64, f1 | f2, false))
}

pub fn equal(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	Ok(((v1 == v2) as i32 as i64, f1 | f2, false))
}

pub fn not_equal(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	Ok(((v1 != v2) as i32 as i64, f1 | f2, false))
}

pub fn and_and(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	if f1 == 0 && v1 == 0 || f2 == 0 && v2 == 0 {
		Ok((0, 0, false))
	} else {
		Ok((1, f1 | f2, false))
	}
}

pub fn or_or(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	if f1 == 0 && v1 != 0 || f2 == 0 && v2 != 0 {
		Ok((1, 0, false))
	} else {
		Ok((0, f1 | f2, false))
	}
}

pub fn xor(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	Ok((v1 ^ v2, f1 | f2, false))
}

pub fn and(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	Ok((v1 & v2, f1 | f2, false))
}

pub fn or(v1: i64, v2: i64, f1: i32, f2: i32) -> ResultOfOperation {
	Ok((v1 | v2, f1 | f2, false))
}
