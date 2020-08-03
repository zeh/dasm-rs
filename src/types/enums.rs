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

// In original C code, "ADDRESS_MODES"
#[derive(Copy, Clone, PartialEq)]
pub enum AddressModes {
	Imp,						// Implied
	Imm8,						// Immediate 8 bits
	Imm16,		        		// Immediate 16 bits
	ByteAdr,					// Address 8 bits
	ByteAdrX,					// Address 16 bits
	ByteAdrY,					// Relative 8 bits
	WordAdr,					// Index x 0 bits
	WordAdrX,					// Index x 8 bits
	WordAdrY,					// Index x 16 bits
	Rel,						// Bit inst. special
	IndByteX,					// Bit-bra inst. spec.
	IndByteY,					// Index y 0 bits
	IndWord,					// Index y 8 bits
	ZeroX,						// Index x 0 bits
	ZeroY,						// Index y 0 bits
	BitMod,						// Ind addr 8 bits
	BitBraMod,					// Ind addr 16 bits
	Symbol,
	ExpList,
	Long,
	BSS,
}

// Made to replace "NUMOC" in the original type
impl AddressModes {
    pub fn length() -> u32 {
		return Self::BSS as u32 - Self::Imp as u32 + 1
    }
}
