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

#[derive(Default)]
pub struct Config {
    pub output_path: PathBuf,
    pub libraries: Libraries,
}
