//! This module contains all information pertaining to the functionality of counting the files and directories as they are iterated, and reporting the information gathered.
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
