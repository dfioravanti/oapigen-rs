use thiserror::Error;

#[derive(Error, Debug)]
pub enum ParsingError {
    #[error("the following properties are incompatible with each other: {0:?}")]
    IncompatibleProperties(#[from] anyhow::Error),
}
