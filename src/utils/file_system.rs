use crate::error::Error;
use std::path::{Path, PathBuf};
use std::env::current_dir;
use std::fs::{read_to_string, File, write, remove_file};
use std::io::prelude::*;

pub fn get_full_path(from_request: PathBuf) -> Result<PathBuf, Error> {
  let mut cwd = current_dir()?;
  cwd.push(PathBuf::from("archive"));
  cwd.push(PathBuf::from(from_request));
  Ok(cwd)
}

pub fn read_file(path: PathBuf) -> Result<String, Error> {
  let file_contents = read_to_string(path)?;
  Ok(file_contents)
}
pub fn create_file(filename: PathBuf, contents: String) -> Result<String, Error> {
  let path = get_full_path(filename)?;
  let mut file = File::create(path)?;
  let bytes = contents.as_bytes();
  file.write_all(bytes)?;
  Ok(String::from_utf8(bytes.to_vec())?)
}
pub fn update_file(filename: PathBuf, contents: String) -> Result<String, Error> {
  let full_path = get_full_path(filename)?;
    if Path::new(&full_path).exists() {
    write(&full_path, contents)?;
    return Ok(read_to_string(full_path)?);
  }

  Err(Error::new(404, &format!("File: \"{}\", doesn't exists", full_path.to_str().unwrap())))
}
pub fn delete_file(filename: PathBuf) -> Result<(), Error> {
  match remove_file(get_full_path(filename)?) {
    Ok(()) => Ok(()),
    Err(err) => Err(Error::from(err))
  }
}