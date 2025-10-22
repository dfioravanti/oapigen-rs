use std::path::PathBuf;

pub enum DateTimeLibraries {
    Chrono,
    Jiff,
}

pub struct Libraries {
    pub datetime: DateTimeLibraries,
}

impl Default for Libraries {
    fn default() -> Self {
        Libraries {
            datetime: DateTimeLibraries::Chrono,
        }
    }
}

/// Config represents all the configuration options that can be set in the crate.
///
/// In particular, it allows to configure things like:
/// - Output path
/// - Which libraries to use to represent complex datatypes like datetime, etc
///
/// Current defaults:
/// - datetime: [Chrono](https://docs.rs/chrono/latest/chrono/)
#[derive(Default)]
pub struct Config {
    pub output_path: PathBuf,
    pub libraries: Libraries,
}
