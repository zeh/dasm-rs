use crate::filesystem;
use crate::operations;
use crate::transient;
use crate::constants::{
    ErrorDefinitions,
};
use crate::types::{
    enums::{
        AsmErrorEquates,
        ErrorFormat,
    },
    flags::{
        FileFlags,
    },
    structs::{
        GlobalState,
        ErrorDefinition,
    },
};

// FIXME: remove this once pIncFile is not needed
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _INCFILE {
    pub next: *mut _INCFILE,
    pub name: *mut i8,
    pub fi: *mut FILE,
    pub lineno: u64,
    pub flags: u8,
    pub args: *mut _STRLIST,
    pub strlist: *mut _STRLIST,
    pub saveidx: u64,
    pub savedolidx: u64,
}
extern "C" {
    #[no_mangle]
    static mut pIncfile: *mut _INCFILE;
}

/**
 * Searches for an ErrorDefinition in the ErrorDefinitions list,
 * based on its errorType. This replaces the old index access
 * where the entry would be in the index of the errorType's uint value.
 */
pub fn find_error_definition(errorType: AsmErrorEquates) -> &'static ErrorDefinition {
	return &ErrorDefinitions.iter().find(|e| e.errorType == errorType).unwrap();
}

/**
 * Prints a message and exists.
 * In original C code, "panic()" in main.c
 * FIXME: this should probably be a panic!() instead, but for now we use this to follow original DASM C behavior.
 * The output for errors should also use eprintln!(), but once again, the original used puts() instead.
 */
 pub fn panic(message: &str) {
	println!("{}", message);
	std::process::exit(1);
}

/**
 * Handle error messaging and behavior.
 * In original C code, "asmerr()" in main.c
 */
// FIXME: remove this unsafe() once pIncFile is not needed
pub unsafe fn handle_error(state: &mut GlobalState, err: AsmErrorEquates, abort: bool, text: &str) {
    let mut errorOutput: String = String::new();

    // FIXME: replace this with a IncludeFile reference
    let mut pincfile: *mut _INCFILE = 0 as *mut _INCFILE;

    // File we print error messages to
    let errorToFile = !state.parameters.listFile.is_empty();
    let mut errorFile = &mut state.output.listFile;
    if find_error_definition(err).fatal {
        state.other.stopAtEnd = true
    }
    pincfile = pIncfile;
    while (*pincfile).flags & FileFlags::Macro != 0 {
        pincfile = (*pincfile).next;
    }
    let mut errorDescription = find_error_definition(err).description.clone().to_owned();
    errorDescription.push_str("\n");
    /*
        New error format selection for 2.20.11 since some
        people *don't* use MS products. For historical
        reasons we currently send errors to stdout when
        they should really go to stderr, but we'll switch
        eventually I hope... [phf]
    */
    /* print first part of message, different formats offered */
    match state.parameters.errorFormat {
        ErrorFormat::Woe => {
            /*
                Error format for MS VisualStudio and relatives:
                "file (line): error: string"
            */
            let mut errorMessage = format!(
                "{} ({}): error: ",
                transient::str_pointer_to_string((*pincfile).name),
                (*pincfile).lineno
            );
            if errorToFile {
                filesystem::write_to_file_maybe(errorFile, errorMessage.as_str());
            }
            errorOutput.push_str(errorMessage.as_str());
        }
        ErrorFormat::Dillon => {
            /*
                Matthew Dillon's original format, except that
                we don't distinguish writing to the terminal
                from writing to the list file for now. Matt's
                2.16 uses these:

                    "*line %4ld %-10s %s\n" (list file)
                    "line %4ld %-10s %s\n" (terminal)
            */
            let mut errorMessage = format!(
                "line {:>7} {:10} ",
                (*pincfile).lineno,
                transient::str_pointer_to_string((*pincfile).name),
            );
            if errorToFile {
                filesystem::write_to_file_maybe(errorFile, errorMessage.as_str());
            }
            errorOutput.push_str(errorMessage.as_str());
        }
        ErrorFormat::GNU => {
            /*
                GNU format error messages, from their coding
                standards.
            */
            let mut errorMessage = format!(
                "{}:{}: error: ",
                transient::str_pointer_to_string((*pincfile).name),
                (*pincfile).lineno,
            );
            if errorToFile {
                filesystem::write_to_file_maybe(errorFile, errorMessage.as_str());
            }
            errorOutput.push_str(errorMessage.as_str());
        }
    }
    // This is a bit of a hack: since we can't use variables as the template in format!(),
    // we simply replace "{}" in the template with the expected string. This works well,
    // but it means the template only supports a single {}, and no other formatting directive.
    if text.len() > 0 {
        errorDescription = errorDescription.replace("{}", text);
    }
    if errorToFile {
        /* print second part of message, always the same for now */
        filesystem::write_to_file_maybe(errorFile, errorDescription.as_str());
    }
    errorOutput.push_str(errorDescription.as_str());
    operations::update_passbuffer(&mut state.output.passBufferErrors, errorOutput.as_str());
    if abort {
        operations::output_passbuffer(&mut state.output.passBufferMessages);
        if errorToFile {
            filesystem::writeln_to_file_maybe(
                errorFile,
                "Aborting assembly"
            );
            filesystem::close_file_maybe(errorFile);
        } else {
            println!("Aborting assembly");
        }
        operations::output_passbuffer(&mut state.output.passBufferErrors);
        std::process::exit(1);
    }
    return err;
}
