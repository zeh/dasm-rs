// ----------------------------------------------------------------------
// - This file's contents came from "asm.h" in the original DASM C code -
// ----------------------------------------------------------------------

use num_enum::{
	IntoPrimitive,
	TryFromPrimitive,
};

// Used for -T option
// In original C code, "sortmode_t"
// FIXME: revise whether we actually need the u8 conversion
#[derive(Copy, Clone, IntoPrimitive, TryFromPrimitive, PartialEq, SmartDefault)]
#[repr(u8)]
pub enum SortMode {
	#[default]
	Alpha,
	Address,
}

// Used for -E option
// In original C code, "errorformat_t"
// FIXME: revise whether we actually need the u8 conversion
#[derive(Copy, Clone, IntoPrimitive, TryFromPrimitive, PartialEq, SmartDefault)]
#[repr(u8)]
pub enum ErrorFormat {
	#[default]
	Woe,
	Dillon,
	GNU,
}

// Used for -f option
// In original C code, "FORMAT"
// FIXME: revise whether we actually need the u8 conversion
#[derive(Copy, Clone, IntoPrimitive, TryFromPrimitive, PartialEq, SmartDefault)]
#[repr(u8)]
pub enum Format {
	#[default]
	Default = 1, // No equivalent entry
	Ras,
	Raw,
}

#[derive(Copy, Clone, Debug)]
#[repr(i8)]
pub enum ExitCode {
	Ok = 0,
	Failure = 1, // "EXIT_FAILURE" in original C code
}


// In original C code, "ASM_ERROR_EQUATES"
// FIXME: check which aren't used anymore. "BadError", for example, shouldn't be needed,
// as it's about passing invalid ints to error handling, which should never happen with
// our named enum type.
// FIXME: revise whether we actually need the u8 conversion
#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive, PartialEq, SmartDefault)]
#[repr(u8)]
pub enum AsmErrorEquates {
	#[default]
	None = 0,
	CommandLine,   // Check format of command-line
	FileError,     // Unable to open file
	NotResolvable, // Source is not resolvable
	TooManyPasses, // Too many passes - something wrong
	NonAbort,      // one or more non-abort errors occured

	SyntaxError,
	ExpressionTableOverflow,
	UnbalancedBraces,
	DivisionByZero,
	UnknownMnemonic,
	IllegalAddressingMode,
	IllegalForcedAddressingMode,
	NotEnoughArgumentsPassedToMacro,
	PrematureEOF,
	IllegalCharacter,
	BranchOutOfRange,
	ErrPseudoOpEncountered,
	OriginReverseIndexed,
	EquValueMismatch,
	AddressMustBeLowerThan100,
	AddressMustBeLowerThan10000,
	IllegalBitSpecification,
	NotEnoughArgs,
	LabelMismatch,
	MacroRepeated,
	ValueUndefined,
	ProcessorNotSupported,
	RepeatNegative,
	BadError,
	OnlyOneProcessorSupported, // Only allow one type of processor
	BadFormat,                 // Bad format specifier

	// F8 support
	ValueMustBeOneOrFour,
	ValueMustBeLowerThan10,
	ValueMustBeLowerThan8,
	ValueMustBeLowerThanF,
	ValueMustBeLowerThan10000,
	IllegalOperandCombination,

	EndOfTable, // FIXME: remove? This was added but might not be needed
}

// In original C code, used for "F_verbose" but without a clear enum
// FIXME: remove derives as possible
#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive, PartialEq, SmartDefault)]
#[repr(u8)]
pub enum Verbosity {
	#[default]
	None = 0,
	One = 1,
	Two = 2,
	Three = 3,
	Four = 4,
}

// In original C code, used for "ListMode" but without a clear enum
// FIXME: remove derives as possible
#[derive(Copy, Clone, Debug, IntoPrimitive, TryFromPrimitive, PartialEq, SmartDefault)]
#[repr(u8)]
pub enum ListMode {
	#[default]
	List,
	None,
}

// In original C code, "ADDRESS_MODES"
#[derive(Copy, Clone, Debug, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum AddressModes {
	Imp,                              // 0  Implied
	Imm8,                             // 1  Immediate 8 bits
	Imm16,                            // 2  Immediate 16 bits
	ByteAdr,                          // 3  Address 8 bits
	ByteAdrX,                         // 4  Address 16 bits
	ByteAdrY,                         // 5  Relative 8 bits
	WordAdr,                          // 6  Index x 0 bits
	WordAdrX,                         // 7  Index x 8 bits
	WordAdrY,                         // 8  Index x 16 bits
	Rel,                              // 9  Bit inst. special
	IndByteX,                         // 10 Bit-bra inst. spec.
	IndByteY,                         // 11 Index y 0 bits
	IndWord,                          // 12 Index y 8 bits
	ZeroX,                            // 13 Index x 0 bits
	ZeroY,                            // 14 Index y 0 bits
	BitMod,                           // 15 Ind addr 8 bits
	BitBraMod,                        // 16 Ind addr 16 bits
	Symbol,                           // 17
	ExpList,                          // 18
	Long,                             // 19
	BSS,                              // 20

	None,                             // 21 Made to simulate reset of -1 in findext()
}

impl AddressModes {
	// Made to replace "NUMOC" in the original type
	pub fn length() -> u32 {
		return Self::BSS as u32 - Self::Imp as u32 + 1;
	}

	// In original C code, "Opsize" (as an array with a different length than the enum list)
	pub fn operation_size(&self) -> u8 {
		match self {
			Self::Imp => 0,
			Self::Imm8 => 1,
			Self::Imm16 => 2,
			Self::ByteAdr => 1,
			Self::ByteAdrX => 1,
			Self::ByteAdrY => 1,
			Self::WordAdr => 2,
			Self::WordAdrX => 2,
			Self::WordAdrY => 2,
			Self::Rel => 2,
			Self::IndByteX => 1,
			Self::IndByteY => 1,
			Self::IndWord => 2,
			Self::ZeroX => 0,
			Self::ZeroY => 0,
			Self::BitMod => 1,
			Self::BitBraMod => 1,
			_ => 0,
		}
	}

	// In original C code, "Cvt" (as an array with a different length than the enum list)
	pub fn convert(&self) -> Self {
		match self {
			Self::Imp => Self::Imp,
			Self::Imm8 => Self::Imm16,
			Self::Imm16 => Self::Imp,
			Self::ByteAdr => Self::WordAdr,
			Self::ByteAdrX => Self::WordAdrX,
			Self::ByteAdrY => Self::WordAdrY,
			Self::WordAdr => Self::Rel,
			Self::WordAdrX => Self::Imp,
			Self::WordAdrY => Self::Imp,
			Self::Rel => Self::Imp,
			Self::IndByteX => Self::Imp,
			Self::IndByteY => Self::Imp,
			Self::IndWord => Self::Imp,
			Self::ZeroX => Self::ByteAdrX,
			Self::ZeroY => Self::ByteAdrY,
			Self::BitMod => Self::Imp,
			Self::BitBraMod => Self::Imp,
			_ => Self::Imp,
		}
	}
}

// In original C code, used for "MsbOrder" but without a clear enum
#[derive(Copy, Clone, Debug, PartialEq, SmartDefault)]
pub enum BitOrder {
	LeastMost, // lsb, msb
	#[default]
	MostLeast, // msb, lsb
}

// In original C code, used for "Processor" but without a clear enum\
// The enum values are... creative.
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Processors {
	None = 0,
	MOS_6502 = 6502,
	MOTOROLA_6803 = 6803,
	HD_6303 = 6303,
	MOTOROLA_68705 = 68705,
	MOTOROLA_68HC11 = 6811,
	FAIRCHILD_F8 = 0xf8,
}
