// Std imports
use std::ffi::OsString;
use std::path::PathBuf;

// External imports
use clap::{clap_app, crate_authors, crate_description, crate_version};

// Internal imports
use crate::error::{Error, Result};
use crate::utils::parse_or_exit;

#[derive(Debug)]
pub struct Cli {
    pub input_path: PathBuf,
    pub working_directory: PathBuf,
    pub extension: OsString,
    pub new_name_base: String,
    pub lines: usize,
    pub include_headers: bool,
}

impl Cli {
    pub fn from_args() -> Result<Self> {
        let matches = clap_app!(file_split =>
            (version: crate_version!())
            (author: crate_authors!())
            (about: crate_description!())
            (@arg file: -f --file
                +takes_value
                +required
                "Path to the file to be split")
            (@arg new_file_name: -n --("new-file-name")
                +takes_value
                "Name of the new files, which will be appended with an incremented number \
                (default: '<original file name>-split')")
            (@arg lines: -l --("line-count")
                +takes_value
                "Number of lines per file (default: 1)")
            (@arg include_headers: -i --("include-headers")
                +takes_value
                "Include the headers from the original file (assumed to be the first line) \
                in all split files (default: true if the original file is a CSV file, false \
                otherwise)")
        )
        .get_matches();

        let input_path: PathBuf = matches
            .value_of("file")
            .map(parse_or_exit)
            .ok_or_else(|| Error::new("argument '--file <file>' missing a required value"))?;
        let wd = input_path.parent().ok_or_else(|| {
            Error::new("required value of argument '--file <file>' missing or invalid")
        })?;
        let file_stem = input_path.file_stem().ok_or_else(|| {
            Error::new("required value of argument '--file <file>' missing or invalid")
        })?;
        let extension = input_path.extension().ok_or_else(|| {
            Error::new("required value of argument '--file <file>' missing or invalid")
        })?;

        let new_name_base = matches.value_of("new_file_name").map_or_else(
            || format!("{}-split", file_stem.to_str().unwrap()),
            String::from,
        );
        let lines = matches.value_of("lines").map_or(1, parse_or_exit);

        let include_headers = matches
            .value_of("include_headers")
            .map_or_else(|| extension.to_str().unwrap() == "csv", parse_or_exit);

        Ok(Cli {
            input_path: input_path.clone(),
            working_directory: PathBuf::from(wd),
            extension: OsString::from(extension),
            new_name_base,
            lines,
            include_headers,
        })
    }
}
