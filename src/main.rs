mod config;
mod constants;
mod counter;
mod opts;
mod util;

use config::Config;
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
    /// An inner function, enabling walking a path when it is known for certain that the path is a directory.
    /// This is an invariant that must hold, or the call to `fs::read_dir()` will fail.
    /// The point of this nesting is to reduce the number of calls to `is_dir()`, reducing the number of systems calls overall.
    fn walk_dir(p: &Path, prefix: &str, counter: &mut Counter) -> io::Result<()> {
        // Read the children of the target path
        let mut path_iter = fs::read_dir(p)
            .expect("Could not read directory")
            // Unwrap the children, and extract their path
            .map(|e| e.expect("IO error during iteration of path").path())
            // Filter hidden files
            .filter(|f| util::is_hidden(f))
            // Create a peekable iterator, for looking ahead
            .peekable();

        // Iterate the paths while there are still paths
        while let Some(next_path) = path_iter.next() {
            // Calculate the seperator and string to append to the prefix
            let (seperator, new_prefix) = match path_iter.peek() {
                Some(_) => (constants::TEE, constants::BAR),
                None => (constants::ELL, constants::TAB),
            };

            // Print the constructed line of the tree
            print_line(prefix, seperator, &next_path);

            // Traverse this child if it is a directory
            if next_path.is_dir() {
                counter.inc_dirs();
                walk(&next_path, &format!("{}{}", prefix, new_prefix), counter)?;
            } else {
                counter.inc_files();
            }
        }
        // Success
        Ok(())
    }
    // Check directory
    if p.is_dir() {
        // Walk the directory
        walk_dir(&p, prefix, counter)
    } else {
        // Exit
        Ok(())
    }
}

fn main() -> io::Result<()> {
    let matches = opts::get_config().get_matches();
    let p = Path::new(matches.value_of("Directory").unwrap_or("."));
    let conf = Config::from_matches(&matches);
    println!("{}", p.display());
    let mut c = Counter::new();
    walk(&p, "", &mut c).unwrap();
    println!("{}", c);
    Ok(())
}
