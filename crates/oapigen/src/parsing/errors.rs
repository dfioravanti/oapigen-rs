use oas3::spec;
use proc_macro2::LexError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("could not parse the string to TokenStream.")]
    CannotParseString(#[from] LexError),
    #[error("could not resolve the reference with the passed spec.")]
    ReferenceNotFound(#[from] spec::RefError),
}
