// ----------------------------------------------------------------------
// - This file's contents came from "asm.h" in the original DASM C code -
// ----------------------------------------------------------------------

use crate::types::enums::{
    AsmErrorEquates,
};
use crate::types::structs::{
    ErrorDefinition,
};

pub const ALLOC_SIZE: u16 = 16384;   // In original C code, "ALLOCSIZE"
pub const DEF_ORG_FILL: u8 = 255;    // In original C code, "DEFORGFILL"
pub const MAX_LINES: usize = 1024;   // In original C code, "MAXLINE"
pub const MAX_MACRO_LEVEL: u8 = 32;  // In original C code, "MAXMACLEVEL"
pub const MAX_SYMBOLS: usize = 1024; // In original C code, "MAX_SYM_LEN"

// FIXME: a bit of a hack since we can't concatenate consts, and I wanted to still keep DASM_RELEASE separate
macro_rules! DASM_RELEASE {() => {"2.20.14-SNAPSHOT"}}
pub const DASM_ID: &'static str = concat!("DASM ", DASM_RELEASE!());

// Table encapsulates errors, descriptions, and fatality flags.
// Notice that the "{}" inside the descriptions are used for simple
// string substitution; it doesn't use println!()/sformat!(), so
// advanced {:} configurations won't work.
pub static ErrorDefinitions: [ErrorDefinition; 39] = [
    ErrorDefinition {
		errorType: AsmErrorEquates::None,
		fatal: true,
		description: "OK",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::CommandLine,
		fatal: true,
		description: "Check command-line format.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::FileError,
		fatal: true,
		description: "Unable to open file.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::NotResolvable,
		fatal: true,
		description: "Source is not resolvable.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::TooManyPasses,
		fatal: true,
		description: "Too many passes ({}).",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::NonAbort,
		fatal: true,
		description: "See previous output",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::SyntaxError,
		fatal: true,
		description: "Syntax Error '{}'.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ExpressionTableOverflow,
		fatal: true,
		description: "Expression table overflow.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::UnbalancedBraces,
		fatal: true,
		description: "Unbalanced Braces [].",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::DivisionByZero,
		fatal: true,
		description: "Division by zero.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::UnknownMnemonic,
		fatal: true,
		description: "Unknown Mnemonic '{}'.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::IllegalAddressingMode,
		fatal: false,
		description: "Illegal Addressing mode '{}'.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::IllegalForcedAddressingMode,
		fatal: true,
		description: "Illegal forced Addressing mode on '{}'.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::NotEnoughArgumentsPassedToMacro,
		fatal: true,
		description: "Not enough args passed to Macro.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::PrematureEOF,
		fatal: false,
		description: "Premature EOF.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::IllegalCharacter,
		fatal: true,
		description: "Illegal character '{}'.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::BranchOutOfRange,
		fatal: false,
		description: "Branch out of range ({} bytes).",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ErrPseudoOpEncountered,
		fatal: true,
		description: "ERR pseudo-op encountered.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::OriginReverseIndexed,
		fatal: false,
		description: "Origin Reverse-indexed.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::EquValueMismatch,
		fatal: false,
		description: "EQU: Value mismatch.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::AddressMustBeLowerThan100,
		fatal: true,
		description: "Value in '{}' must be <$100.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::AddressMustBeLowerThan10000,
		fatal: true,
		description: "Value in '{}' must be <$10000.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::IllegalBitSpecification,
		fatal: true,
		description: "Illegal bit specification.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::NotEnoughArgs,
		fatal: true,
		description: "Not enough arguments.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::LabelMismatch,
		fatal: false,
		description: "Label mismatch...\n --> {}",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::MacroRepeated,
		fatal: true,
		description: "Macro \"{}\" definition is repeated.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ValueUndefined,
		fatal: true,
		description: "Value Undefined.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ProcessorNotSupported,
		fatal: true,
		description: "Processor '{}' not supported.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::RepeatNegative,
		fatal: false,
		description: "REPEAT parameter < 0 (ignored).",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::BadError,
		fatal: true,
		description: "Bad error value (internal error).",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::OnlyOneProcessorSupported,
		fatal: true,
		description: "Only one processor type may be selected.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::BadFormat,
		fatal: true,
		description: "Bad output format specified.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ValueMustBeOneOrFour,
		fatal: true,
		description: "Value in '{}' must be 1 or 4.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ValueMustBeLowerThan10,
		fatal: true,
		description: "Value in '{}' must be <$10.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ValueMustBeLowerThan8,
		fatal: true,
		description: "Value in '{}' must be <$8.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ValueMustBeLowerThanF,
		fatal: true,
		description: "Value in '{}' must be <$f.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::ValueMustBeLowerThan10000,
		fatal: true,
		description: "Value in '{}' must be <$10000.",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::IllegalOperandCombination,
		fatal: true,
		description: "Illegal combination of operands '{}'",
	},
	ErrorDefinition {
		errorType: AsmErrorEquates::EndOfTable, // FIXME: remove? This was added but might not be needed
		fatal: true,
		description: "Doh! Internal end-of-table marker, report the bug!",
	}
];
