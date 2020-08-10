//! This module contains code relevant to argument parsing.
//! The crate `clap` is used as a framework to handle parsing reliably.
//! All arguments and the app itself are modularly divided into functions that customize their output.
use clap::{App, Arg};

/// The `Config` structure defines the configuration options specified to the program.
/// The fields will be parsed from the external input, then passed to the walk function in this convenient structure that encapsulates them.
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

/// Public getter method for acquiring the app, along with all arguments attached.
pub fn get_config<'a, 'b>() -> App<'a, 'b> {
    get_app()
        .arg(target_dir_arg())
        .arg(all_files_arg())
        .arg(dirs_only_arg())
        .arg(full_path_arg())
        .arg(level_arg())
}

/// Creates the app itself; the tree API.
/// Information relating to the project, help, and arguments are created here.
fn get_app<'a, 'b>() -> App<'a, 'b> {
    App::new("Tree")
        .about("list contents of directories in a tree-like format.")
        .after_help("Tree is a recursive directory listing program that produces a depth indented listing of files. With no arguments, tree lists the files in the current directory. When directory arguments are given, tree lists all the files and/or directories found in the given directories each in turn. Upon completion of listing all files/directories found, tree returns the total number of files and/or directories listed.")
}

/// Creates the target directory argument.
/// This argument defines the root directory where the file listing should begin.
/// This argument will default to the current working directory if no other path is provided.
fn target_dir_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("Directory").default_value(".")
}

/// Creates the show all files argument.
/// The presence of this argument indicates that all files should be printed, regardless of whether they are hidden.
fn all_files_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("all_files")
            .short("a")
            .long("all")
            .help("All files are printed. By default tree does not print hidden files (those beginning with a dot `.') In no event does tree print the file system construct `.' (current directory) and `..' (previous directory).")
}

/// Creates the directories only argument.
/// The presence of this argument indicates that only directories should be printed, not files.
fn dirs_only_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("dirs_only")
        .short("d")
        .help("List directories only.")
}

/// Creates the full path prefix argument.
/// The presence of this argument indicates that the full path of every file should be printed, not just the file name.
fn full_path_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("full_path")
        .short("-f")
        .help("Prints the full path prefix for each file.")
}

/// Creates the maximum depth level argument.
/// The value indicated by this argument is the maximum depth of the directory tree that the printing should be continued.
fn level_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("level")
        .short("L")
        .takes_value(true)
        .value_name("level")
        .help("Max display depth of the directory tree.")
}
