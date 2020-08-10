use crate::NameGetter;
use std::path::Path;

/// Checks whether the file is hidden, by examining the first character of the file name to see if it is a period.
pub fn is_hidden(path: &Path) -> bool {
    &path.get_file_name()[..1] != "."
}
