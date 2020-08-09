// ----------------------------------------------------------------------
// - This file's contents came from "asm.h" in the original DASM C code -
// ----------------------------------------------------------------------

use num_enum::{IntoPrimitive, TryFromPrimitive};

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
#[derive(Copy, Clone, Debug, PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum AddressModes {
	Imp,						// 0  Implied
	Imm8,						// 1  Immediate 8 bits
	Imm16,		        		// 2  Immediate 16 bits
	ByteAdr,					// 3  Address 8 bits
	ByteAdrX,					// 4  Address 16 bits
	ByteAdrY,					// 5  Relative 8 bits
	WordAdr,					// 6  Index x 0 bits
	WordAdrX,					// 7  Index x 8 bits
	WordAdrY,					// 8  Index x 16 bits
	Rel,						// 9  Bit inst. special
	IndByteX,					// 10 Bit-bra inst. spec.
	IndByteY,					// 11 Index y 0 bits
	IndWord,					// 12 Index y 8 bits
	ZeroX,						// 13 Index x 0 bits
	ZeroY,						// 14 Index y 0 bits
	BitMod,						// 15 Ind addr 8 bits
	BitBraMod,					// 16 Ind addr 16 bits
	Symbol,						// 17
	ExpList,					// 18
	Long,						// 19
	BSS,						// 20

	None,						// 21 Made to simulate reset of -1 in findext()
}

// Made to replace "NUMOC" in the original type
impl AddressModes {
    pub fn length() -> u32 {
		return Self::BSS as u32 - Self::Imp as u32 + 1
    }
}

// In original C code, used for "MsbOrder" but without a clear enum
#[derive(Copy, Clone, Debug, PartialEq, SmartDefault)]
pub enum BitOrder {
	LeastMost, // lsb, msb
	#[default]
	MostLeast, // msb, lsb
}
