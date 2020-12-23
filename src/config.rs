//! The `Config` structure defines the configuration options specified to the program.
//! The fields will be parsed from the external input, then passed to the walk function in this convenient structure that encapsulates them.
#[derive(Debug)]
pub struct Config {
    /// Whether to show hidden files (*-a*, *--all*).
    all_files: bool,
    /// Whether to exclusively show directories (*-d*).
    dirs_only: bool,
    /// Whether to show the full path, instead of solely the file name (*-f*).
    full_path: bool,
    /// The maximum depth to traverse the directory tree (*-L*).
    level: Option<usize>,
}

use clap::ArgMatches;
impl Config {
    pub fn from_matches(matches: &ArgMatches) -> Self {
        Config {
            all_files: matches.is_present("all_files"),
            dirs_only: matches.is_present("dirs_only"),
            full_path: matches.is_present("full_path"),
            level: matches
                .value_of("level")
                .and_then(|lvl| Some(lvl.parse::<usize>().unwrap())),
        }
    }

    pub fn create_pred(&self) -> &dyn Fn(&std::path::PathBuf) -> bool {
        &|f| true
    }
}
