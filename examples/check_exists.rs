use std::path::{Path, PathBuf};

use path_no_alloc::with_paths;

fn check_exists<R1, R2>(root: R1, paths: impl IntoIterator<Item = R2>)
where
    R1: AsRef<Path>,
    R2: AsRef<Path>,
{
    for path in paths {
        with_paths!{
            path = root / path
        };

        if path.exists() {
            println!("{path:?} exists")
        } else {
            println!("{path:?} does not exist.")
        }
    }
}

fn main() {
    check_exists(
        "src",
        ["lib.rs", "tests.rs", "some-other-file.txt"]
    );
}
