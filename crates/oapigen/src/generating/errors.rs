use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratingError {
    #[error("could not write to file.")]
    WriteToFileError(#[from] io::Error),
}
