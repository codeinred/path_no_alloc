#[cfg(test)]
mod tests;

use std::{
    ffi::OsStr,
    path::{Path, PathBuf},
};

#[cfg(target_family = "unix")]
use std::os::unix::ffi::OsStrExt;

/// Joins two paths. If the paths fit inside the given buffer,
/// uses the buffer. Otherwise, uses the given pathbuff.
///
/// Returns a Path referencing whichever one was used.
#[cfg(target_family = "unix")]
pub fn join_in_buff<'a, const N: usize>(
    raw_buff: &'a mut [u8],
    path_buff: &'a mut PathBuf,
    paths: [&Path; N],
) -> &'a Path {
    // Find the start index of the first path that's not absulote, from the end
    // this will be the start of our join. If there's no absolute path, we start
    // with the first one
    let start_idx = paths.iter().rposition(
        |p| p.is_absolute()
    ).unwrap_or(0);

    // Now we whittle down the space of paths we're joining to just the ones we
    // care about. Also, from here on out we're working with byte strings.
    let paths = paths.map(|p| p.as_os_str().as_bytes());
    let paths = &paths[start_idx..];

    // Compute the amount of space required to store all the paths
    let total_len = paths.iter().map(|x| 1 + x.len()).sum();

    // If they fit in the raw buffer, we'll join the paths in the raw buffer.
    // Otherwise, we'll put them into the pathbuf.
    if total_len <= raw_buff.len() {
        let mut start = 0;

        for bytes in paths {
            let end = start + bytes.len();
            raw_buff[start..end].copy_from_slice(bytes);
            raw_buff[end] = b'/';
            start = end + 1;
        }
        // Add a null terminator instead of a slash at the end
        raw_buff[start - 1] = b'\0';

        OsStr::from_bytes(&raw_buff[..(start - 1)]).as_ref()
    } else {
        path_buff.clear();
        path_buff.reserve(total_len);

        for path in paths {
            path_buff.push(OsStr::from_bytes(path))
        }

        path_buff.as_path()
    }
}

#[cfg(not(target_family = "unix"))]
pub fn join_in_buff<'a, const N: usize>(
    raw_buff: &'a mut [u8],
    path_buff: &'a mut PathBuf,
    paths: [&Path; N],
) -> &'a Path {
    // I want to keep the function signature the same between windows and not-windows
    // but this variable isn't used. So I assign it to a variable named _
    // in order to indicate to the compiler that it's not currently used
    let _ = raw_buff;
    let paths = paths.map(|p| p.as_os_str());

    let total_len: usize = paths.iter().map(|x| x.len()).sum();
    let total_len = total_len + N + 1;

    path_buff.clear();
    path_buff.reserve(total_len);

    for path in paths {
        path_buff.push(path)
    }

    path_buff.as_path()
}

/// Allows paths to be joined with small path optimization:
/// if the total length of all joined paths is less tahn 128,
/// no PathBuf will be allocated.
///
/// You can use arbitrary objects that are convertible to a Path
/// via `.as_ref()`:
///
/// ```rust
/// use path_no_alloc::with_paths;
/// use std::path::Path;
///
/// /// Check that a file exists in the given path
/// fn has_file<P: AsRef<Path>, Q: AsRef<Path>>(path: P, file: Q) -> bool {
///     with_paths! {
///         full_path = path / file => full_path.exists()
///     }
/// }
/// ```
///
/// You can also join arbitrary numbers of paths together:
///
/// ```rust
/// use std::path::Path;
/// use path_no_alloc::with_paths;
///
/// let p1 = "path";
/// let p2 = "to";
/// let p3 = "thing";
///
/// with_paths! {
///     path1 = p1 / p2 / p3,
///     path2 = p3 / p2 / p1
///     => println!("Path1 = {path1:?}, Path2 = {path2:?}")
/// }
/// ```
///
/// This will print:
///
/// ```output
/// Path1 = "path/to/thing", Path2 = "thing/to/path"
/// ```
///
/// Paths joined in the context of `with_paths!` act as though they've been joined
/// via `Path.join`. In other words, joining paths A and B will just produce B
/// in the case that B is absolute:
///
/// ```rust
/// use path_no_alloc::with_paths;
/// use std::path::Path;
///
/// let p1 = "some/path";
/// let p2 = "/absolute/path";
///
/// with_paths! {
///     path = p1 / p2
///     =>
///     println!("{path:?} should equal \"/absolute/path\"");
///     assert_eq!(path, Path::new(p1).join(p2));
///     assert_eq!(path, Path::new("/absolute/path"));
/// }
/// ```
#[macro_export]
macro_rules! with_paths {
    {
        $( $name:ident = $( $path:ident ) / + ),*
        => $( $statements:stmt );* $(;)?
    } => {
        $(
            let mut arr: [u8; 128] = [0; 128];
            let mut buff = ::std::path::PathBuf::new();
            let $name = $crate::join_in_buff(&mut arr, &mut buff, [$($path.as_ref()),+]);
        )*

        $( $statements )*
    }
}
