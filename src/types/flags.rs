// ----------------------------------------------------------------------
// - This file's contents came from "asm.h" in the original DASM C code -
// ----------------------------------------------------------------------

// In original C code, "REASON_CODES"
pub mod ReasonCodes {
    pub const MnemonicNotResolved: u64 = 0b0000_0000_0000_0001;
    pub const Obscure: u64 = 0b0000_0000_0000_0010;
    pub const DCNotResolved: u64 = 0b0000_0000_0000_0100;
    pub const DVNotResolvedProbably: u64 = 0b0000_0000_0000_1000;
    pub const DVNotResolvedCould: u64 = 0b0000_0000_0001_0000;
    pub const DSNotResolved: u64 = 0b0000_0000_0010_0000;
    pub const AlignNotResolved: u64 = 0b0000_0000_0100_0000;
    pub const AlignRelocatableOriginNotKnown: u64 = 0b0000_0000_1000_0000;
    pub const AlignNormalOriginNotKnown: u64 = 0b0000_0001_0000_0000;
    pub const EquNotResolved: u64 = 0b0000_0010_0000_0000;
    pub const EquValueMismatch: u64 = 0b0000_0100_0000_0000;
    pub const IfNotResolved: u64 = 0b0000_1000_0000_0000;
    pub const RepeatNotResolved: u64 = 0b0001_0000_0000_0000;
    pub const ForwardReference: u64 = 0b0010_0000_0000_0000;
    pub const PhaseError: u64 = 0b0100_0000_0000_0000;
    pub const BranchOutOfRange: u64 = 0b1000_0000_0000_0000;
}

// In original C code, used for flags as "SYM_*" but without a clear enum
pub mod SymbolTypes {
    pub const Unknown: u8 = 0b0000_0001;            // 0x01: Value Uknown
    pub const Referenced: u8 = 0b0000_0100;         // 0x04: Referenced
    pub const StringResult: u8 = 0b0000_1000;       // 0x08: Result is a string
    pub const Set: u8 = 0b0001_0000;                // 0x10: SET instruction used
    pub const Macro: u8 = 0b0010_0000;              // 0x20: Symbol is a macro
    pub const MasterReference: u8 = 0b0100_0000;    // 0x40: Master reference
}

// In original C code, used for flags as "SF_*" but without a clear enum
pub mod SegmentTypes {
    pub const Unknown: u8 = 0b0000_0001;            // 0x01: ORG unknown
    pub const Referenced: u8 = 0b0000_0100;         // 0x04: ORG referenced
    pub const BSS: u8 = 0b0001_0000;                // 0x10: uninitialized area (U flag)
    pub const RelocatableOrigin: u8 = 0b0010_0000;  // 0x20: Relocatable origin active
}
