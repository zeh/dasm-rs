use crate::types::{
    flags::{
        SegmentTypes,
    },
    structs::{
        Segment,
    }
};

/**
 * Clears segments, resetting them to their base state.
 * In original C code, "clearsegs()" in main.c
 */
pub fn clear_segments(segments: &mut Vec<Segment>) {
	for seg in segments {
		seg.flags = (seg.flags & SegmentTypes::BSS) | SegmentTypes::Unknown;
		seg.rflags = SegmentTypes::Unknown;
		seg.initflags = SegmentTypes::Unknown;
		seg.initrflags = SegmentTypes::Unknown;
	}
}
