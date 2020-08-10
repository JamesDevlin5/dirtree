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
    if p.is_dir() {
        let mut path_iter = fs::read_dir(p)
            .expect("Could not read directory")
            .map(|e| e.expect("IO error during iteration of path").path())
            .peekable();
        while path_iter.peek().is_some() {
            let next_path = path_iter.next().expect("Path iterator error");
            counter.accept(&next_path);
            match path_iter.peek() {
                Some(_) => {
                    print_line(prefix, constants::TEE, &next_path);
                    if next_path.is_dir() {
                        let mut new_prefix = String::from(prefix);
                        new_prefix.push_str(constants::BAR);
                        walk(&next_path, &new_prefix, counter)?;
                    }
                }
                None => {
                    print_line(prefix, constants::ELL, &next_path);
                    if next_path.is_dir() {
                        walk(
                            &next_path,
                            &format!("{}{}", prefix, constants::TAB),
                            counter,
                        )?;
                    }
                }
            }
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let p = Path::new(".");
    println!("{}", p.display());
    let mut c = Counter::new();
    walk(&p, "", &mut c).unwrap();
    println!("{}", c);
    Ok(())
}
