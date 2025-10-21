use crate::parsing::errors::ParsingError;
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GeneratingError {
    #[error("Generated rust code cannot be parsed to string.")]
    SynError(#[from] syn::Error),
    #[error("Could not parse spec.")]
    ParseSpecError(#[from] ParsingError),
    #[error("could not write to file.")]
    WriteToFileError(#[from] io::Error),
}
