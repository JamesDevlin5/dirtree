mod constants {
    pub const TAB: &'static str = "    ";
    pub const BAR: &'static str = "│   ";
    pub const TEE: &'static str = "├── ";
    pub const ELL: &'static str = "└── ";
}
mod counter {
    use std::fmt;
    use std::path::Path;
    pub struct Counter {
        dirs: usize,
        files: usize,
    }
    impl Counter {
        pub fn new() -> Self {
            Counter { dirs: 0, files: 0 }
        }
        pub fn accept(&mut self, p: &Path) {
            if p.is_dir() {
                self.dirs += 1;
            } else {
                self.files += 1;
            }
        }
    }
    impl fmt::Display for Counter {
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

fn walk(path: &Path, counter: &mut Counter) -> io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();
            counter.accept(&path);
            if path.is_dir() {
                walk(&path, counter)?;
            }
        }
    }
    Ok(())
}
fn main() -> io::Result<()> {
    let p = Path::new(".");
    println!("{}", p.display());
    let mut c = Counter::new();
    walk(&p, &mut c).unwrap();
    println!("{}", c);
    Ok(())
}
