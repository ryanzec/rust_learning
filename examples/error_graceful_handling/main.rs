use std::fs;
use std::fs::File;
use std::io::{Error, ErrorKind};

const FILE_PATH: &str = "error_handling_with_closure.txt";

fn open_or_create_file() -> Result<File, Error> {
    // instead of just letting this error happen, we handle it gracefully by creating the file if it doesn't exist, // this should always be done when possible over letting the error propagate
    File::open(FILE_PATH).or_else(|error| {
        if error.kind() != ErrorKind::NotFound {
            panic!("unable to open file: {error:?}");
        }

        File::create(FILE_PATH)
    })
}

fn main() -> Result<(), Error> {
    let _ = open_or_create_file();

    fs::remove_file(FILE_PATH)?;

    Ok(())
}
