mod counter {
    use std::fmt;
    struct Counter {
        dirs: usize,
        files: usize,
    }
    impl Counter {
        fn new() -> Self {
            Counter { dirs: 0, files: 0 }
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
fn main() {
    println!("Hello, world!");
}
