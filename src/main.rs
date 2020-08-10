/// Constants; The various character combinations describing the infrastructure that composes the tree-format output style.
mod constants {
    pub const TAB: &'static str = "    ";
    pub const BAR: &'static str = "│   ";
    pub const TEE: &'static str = "├── ";
    pub const ELL: &'static str = "└── ";
}

/// This module contains all information pertaining to the functionality of counting the files and directories as they are iterated, and reporting the information gathered.
mod counter {
    use std::fmt;
    use std::path::Path;

    /// Maintains a count of how many files and directories have been seen.
    /// Both fields are unsigned as there may not be a negative number of either.
    pub struct Counter {
        /// The number of directories traversed thus far.
        dirs: usize,
        /// The number of files traversed thus far.
        files: usize,
    }

    impl Counter {
        /// Creates a new `Counter` struct, and initializes both fields to zero.
        pub fn new() -> Self {
            Counter { dirs: 0, files: 0 }
        }

        /// Allows the counter to accept a path as an argument, and update itself appropriately.
        /// If the path is not a directory, it is automatically considered a file.
        pub fn accept(&mut self, p: &Path) {
            if p.is_dir() {
                self.dirs += 1;
            } else {
                self.files += 1;
            }
        }
    }

    /// Allows for printing the counter status to the console.
    impl fmt::Display for Counter {
        /// Displays the counter as a string, describing its interior information.
        ///
        /// ```
        /// let c = Counter { dirs: 2, files: 3 };
        /// assert_eq!(format!("{}", c), "2 directories, 3 files");
        /// ```
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{} directories, {} files", self.dirs, self.files)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_new() {
            let c = Counter::new();
            assert_eq!(c.dirs, 0);
            assert_eq!(c.files, 0);
        }

        #[test]
        fn test_display() {
            assert_eq!(format!("{}", Counter::new()), "0 directories, 0 files");
            assert_eq!(
                format!(
                    "{}",
                    Counter {
                        dirs: 10,
                        files: 10
                    }
                ),
                "10 directories, 10 files"
            );
            assert_eq!(
                format!(
                    "{}",
                    Counter {
                        dirs: 100,
                        files: 1
                    }
                ),
                "100 directories, 1 files"
            );
            assert_eq!(
                format!(
                    "{}",
                    Counter {
                        dirs: 512,
                        files: 1024
                    }
                ),
                "512 directories, 1024 files"
            );
            assert_eq!(
                format!("{}", Counter { dirs: 0, files: 1 }),
                "0 directories, 1 files"
            );
        }
    }
}
use counter::Counter;
use std::{fs, io, path::Path};

/// The `NameGetter` trait ensures the functionality of retrieving a file name, as a string, from some structure.
trait NameGetter {
    /// Gets the file name corresponding to this structure.
    fn get_file_name(&self) -> &str;
}

/// An implementation of the `NameGetter` trait for `Path`, allowing for retrieval of the final item on the path.
impl NameGetter for Path {
    /// Gets the name of the last item on the path.
    fn get_file_name(&self) -> &str {
        self.file_name()
            .expect("Could not get file name")
            .to_str()
            .expect("Could not convert file name to string")
    }
}

/// A convenient printing function, which prints the prefix, immediately followed by the seperator, immediately followed by the file name of the path
fn print_line(prefix: &str, seperator: &str, path: &Path) {
    println!("{}{}{}", prefix, seperator, path.get_file_name());
}

/// Traverses the provided path, counting the files and directories that are passed.
///
/// # Arguments
///
/// * `p` - The root path of the traversal
/// * `prefix` - The string preceding the file or directory name for each entry in this directory, animating the tree
/// * `counter` - The structure counting occurrences of files and directories
///
/// # Pseudocode
///
/// ```
/// Check if the path is a directory
///     If it is not, there are no children; bail
/// Read the child entries of the directory
/// For each child entry:
///     Increment the counter
///     If this is the last child of its parent:
///         Print the "ell" after the prefix, followed by the file name
///         If this entry is a directory, recur on the walking algorithm
///             The new prefix will have a "tab" appended
///     Otherwise:
///         Print the "tee" after the prefix, followed by the file name
///         If this entry is a directory, recur on the walking algorithm
///             The new prefix will have a "bar" appended
/// ```
fn walk(p: &Path, prefix: &str, counter: &mut Counter) -> io::Result<()> {
    // Ensure directory
    if p.is_dir() {
        // Read the children of the target path
        let mut path_iter = fs::read_dir(p)
            .expect("Could not read directory")
            // Unwrap the children, and extract their path
            .map(|e| e.expect("IO error during iteration of path").path())
            // Create a peekable iterator, for looking ahead
            .peekable();

        // Iterate the paths while there are still paths
        while let Some(next_path) = path_iter.next() {
            // Update the counter
            counter.accept(&next_path);

            // Calculate the seperator and string to append to the prefix
            let (seperator, new_prefix) = match path_iter.peek() {
                Some(_) => (constants::TEE, constants::BAR),
                None => (constants::ELL, constants::TAB),
            };

            // Print the constructed line of the tree
            print_line(prefix, seperator, &next_path);
            // (Attempt to) traverse this child
            walk(&next_path, &format!("{}{}", prefix, new_prefix), counter)?;
        }
    }
    // Success
    Ok(())
}

/// This module contains code relevant to argument parsing.
/// The crate `clap` is used as a framework to handle parsing reliably.
/// All arguments and the app itself are modularly divided into functions that customize their output.
mod config {
    use clap::{App, Arg};

    /// The `Opts` structure defines the options specified to the program.
    /// The fields will be parsed from the external input, then passed to the walk function in this convenient structure that encapsulates them.
    pub struct Opts {
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
}

fn main() -> io::Result<()> {
    let matches = config::get_config().get_matches();
    let p = Path::new(matches.value_of("Target Directory").unwrap_or("."));
    let all_files = matches.is_present("all_files");
    println!("{}", p.display());
    let mut c = Counter::new();
    walk(&p, "", &mut c).unwrap();
    println!("{}", c);
    Ok(())
}
