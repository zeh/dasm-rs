use crate::globals::state;
use crate::types::flags::{
    ReasonCodes,
    SegmentTypes,
};
use crate::types::enums::{
    AsmErrorEquates,
    ExitCode,
    Format,
};
use crate::types::legacy::{
    FILE,
};
use crate::utils::{
    filesystem,
    formatting,
    transient,
};

extern "C" {
    #[no_mangle]
    fn fclose(__stream: *mut FILE) -> i32;
    #[no_mangle]
    fn fopen(__filename: *const i8, __modes: *const i8) -> *mut FILE;
    #[no_mangle]
    fn strchr(_: *const i8, _: i32) -> *mut i8;
    #[no_mangle]
    fn asmerr(err: AsmErrorEquates, bAbort: bool, sText: *const i8) -> i32;
}

#[no_mangle]
pub unsafe extern "C" fn pfopen(name: *const i8, mode: *const i8) -> *mut FILE {
    // FIXME: replace with filesystem::try_open_file_with_locations()
    let mut f: *mut FILE = 0 as *mut FILE;
    f = fopen(name, mode);
    if !f.is_null() {
        return f;
    }
    /* Don't use the incdirlist for absolute pathnames */
    if !strchr(name, ':' as i32).is_null() {
        return 0 as *mut FILE;
    } /*	multiplied later    */
    for incDir in &state.execution.includeDirList {
        let path = filesystem::combine_paths(incDir.as_str(), transient::str_pointer_to_string(name).as_str());
        f = fopen(transient::string_to_str_pointer(path), mode);
        if !f.is_null() {
            break;
        }
    }
    return f;
}
#[no_mangle]
pub unsafe extern "C" fn generate() {
    let mut seekpos: i64 = 0;
    static mut org: u64 = 0;
    let mut i: i32 = 0;
    let mut currentSegment = &mut state.other.segments[state.other.currentSegment];
    if state.execution.redoIndex == 0 {
        if currentSegment.flags & SegmentTypes::BSS == 0 {
            i = state.output.generatedLength as i32 - 1;
            while i >= 0 {
                // In practice, this will never wrap since the value is never high enough, but just in case,
                // we use "wrapping_add()" instead of "+="
                state.output.checksum = state.output.checksum.wrapping_add(state.output.generated[i as usize] as u64);
                i -= 1;
            }
            if state.execution.isClear {
                state.execution.isClear = false;
                if currentSegment.flags & SegmentTypes::Unknown != 0 {
                    state.execution.redoIndex += 1;
                    state.execution.redoWhy |= ReasonCodes::Obscure;
                    return;
                }
                org = currentSegment.org;
                if state.parameters.format == Format::Default || state.parameters.format == Format::Ras {
                    filesystem::write_buffer_to_file_maybe(
                        &mut state.output.outFile,
                        &[(org & 0xff) as u8, (org >> 8 & 0xff) as u8]
                    );
                    if state.parameters.format == Format::Ras {
                        state.output.seekBack = filesystem::get_stream_position_maybe(&mut state.output.outFile) as i64;
                        state.output.segmentLength = 0;
                        filesystem::write_buffer_to_file_maybe(&mut state.output.outFile, &[0u8, 0u8]);
                    }
                }
            }
            match state.parameters.format {
                Format::Raw | Format::Default => {
                    if currentSegment.org < org {
                        println!(
                            "segment: {} {}  vs current org: {:04x}",
                            currentSegment.name,
                            formatting::segment_address_to_string(currentSegment.org, currentSegment.flags),
                            org
                        );
                        asmerr(AsmErrorEquates::OriginReverseIndexed, true, 0 as *const i8);
                        std::process::exit(ExitCode::Failure as u8 as i32);
                    }
                    while currentSegment.org != org {
                        filesystem::write_char_to_file_maybe(&mut state.output.outFile, state.output.orgFill as char);
                        org = org.wrapping_add(1)
                    }
                    filesystem::write_buffer_to_file_maybe(
                        &mut state.output.outFile,
                        &state.output.generated[0..state.output.generatedLength]
                    );
                }
                Format::Ras => {
                    if org != currentSegment.org {
                        org = currentSegment.org;
                        seekpos = filesystem::get_stream_position_maybe(&mut state.output.outFile) as i64;
                        filesystem::seek_maybe(&mut state.output.outFile, state.output.seekBack as u64);
                        filesystem::write_buffer_to_file_maybe(
                            &mut state.output.outFile,
                            &[(state.output.segmentLength & 0xff) as u8, (state.output.segmentLength >> 8 & 0xff) as u8]
                        );
                        filesystem::seek_maybe(&mut state.output.outFile, seekpos as u64);
                        filesystem::write_buffer_to_file_maybe(
                            &mut state.output.outFile,
                            &[(org & 0xff) as u8, (org >> 8 & 0xff) as u8]
                        );
                        state.output.seekBack = filesystem::get_stream_position_maybe(&mut state.output.outFile) as i64;
                        state.output.segmentLength = 0;
                        filesystem::write_buffer_to_file_maybe(&mut state.output.outFile, &[0u8, 0u8]);
                    }
                    filesystem::write_buffer_to_file_maybe(
                        &mut state.output.outFile,
                        &state.output.generated[0..state.output.generatedLength]
                    );
                    state.output.segmentLength += state.output.generatedLength;
                }
            }
            org = org.wrapping_add(state.output.generatedLength as u64)
        }
    }
    currentSegment.org = currentSegment.org.wrapping_add(state.output.generatedLength as u64);
    if currentSegment.flags & SegmentTypes::RelocatableOrigin != 0 {
        currentSegment.rorg = currentSegment.rorg.wrapping_add(state.output.generatedLength as u64)
    };
}
#[no_mangle]
pub unsafe extern "C" fn closegenerate() {
    if state.execution.redoIndex == 0 {
        if state.parameters.format == Format::Ras {
            filesystem::seek_maybe(&mut state.output.outFile, state.output.seekBack as u64);
            filesystem::write_buffer_to_file_maybe(
                &mut state.output.outFile,
                &[(state.output.segmentLength & 0xff) as u8, (state.output.segmentLength >> 8 & 0xff) as u8]
            );
            filesystem::seek_end_maybe(&mut state.output.outFile, 0);
        }
    };
}
