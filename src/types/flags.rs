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
