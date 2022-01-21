// Std imports
use std::cmp;
use std::ffi::OsString;
use std::fmt::Display;
use std::fs::{self, File};
use std::io::{ErrorKind, Read, Write};
use std::path::PathBuf;
use std::process;
use std::str::FromStr;

// Internal imports
use crate::error::{Error, Result};
use crate::iter::FileChunkIterator;

pub fn parse_or_exit<T: FromStr>(input: &str) -> T
where
    <T as FromStr>::Err: Display,
{
    T::from_str(input).unwrap_or_else(|e| {
        eprintln!("error: {}", e);
        process::exit(65);
    })
}

pub fn open_file(path: &PathBuf) -> Result<String> {
    let mut contents = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

pub fn split(contents: &str, chunk_size: usize, headers: bool) -> FileChunkIterator {
    let lines: Vec<&str> = contents.split('\n').into_iter().collect();

    let header = if headers { Some(lines[0]) } else { None };

    let offset = if headers { 1 } else { 0 };
    let len = lines.len() - offset;

    let chunk_count = (len as f32 / chunk_size as f32).ceil() as usize;
    let mut chunks = Vec::with_capacity(chunk_count);

    for c in 0..chunk_count {
        let start = c * chunk_size + offset;
        let end = cmp::min(start + chunk_size, len);

        let mut chunk = String::new();

        if headers {
            chunk.push_str(header.unwrap());
            chunk.push_str("\r\n");
        }

        for line in &lines[start..end] {
            chunk.push_str(line);
            chunk.push_str("\r\n");
        }

        chunks.push(chunk);
    }

    FileChunkIterator::new(chunks)
}

pub fn create_split_file(
    wd: &PathBuf,
    file_name_base: &str,
    number: usize,
    ext: &OsString,
    contents: &str,
) -> Result<()> {
    let file_name = format!(
        "{base}-{n}.{ext}",
        base = file_name_base,
        n = number,
        ext = ext.to_str().unwrap()
    );

    let path = wd.join(file_name_base).join(file_name);

    let mut file = File::create(path)?;
    file.write_all(contents.as_bytes())?;

    Ok(())
}

pub fn create_split_dir(wd: &PathBuf, dir_name: &str) -> Result<()> {
    let path = wd.join(dir_name);

    let res = fs::create_dir(path.clone());

    if let Err(e) = res {
        if e.kind() == ErrorKind::AlreadyExists {
            fs::remove_dir_all(path)?;
            return create_split_dir(wd, dir_name);
        } else {
            return Err(Error::from(e));
        }
    }

    Ok(())
}
