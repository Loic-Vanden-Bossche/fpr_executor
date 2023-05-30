use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

pub fn check_game_script_path(script_path: &String) -> Result<(), Error> {
    let path = Path::new(&script_path);

    if !path.exists() {
        return Err(Error::new(ErrorKind::NotFound, "File does not exist."));
    }

    if !path.is_file() {
        return Err(Error::new(ErrorKind::InvalidInput, "Not a file."));
    }

    match fs::metadata(&path) {
        Ok(metadata) => {
            if !metadata.is_file() {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    "Not a regular file.",
                ));
            }
        }
        Err(error) => {
            return Err(error);
        }
    }

    Ok(())
}
