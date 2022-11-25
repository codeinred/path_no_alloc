#[cfg(test)]
mod tests;

use std::{
    ffi::OsStr,
    mem::MaybeUninit,
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
    raw_buff: &'a mut [MaybeUninit<u8>],
    path_buff: &'a mut Option<PathBuf>,
    paths: [&Path; N],
) -> &'a Path {
    // Find the start index of the first path that's not absulote, from the end
    // this will be the start of our join. If there's no absolute path, we start
    // with the first one

    use std::iter::zip;

    let mut byte_paths: [MaybeUninit<&[u8]>; N] =
        unsafe { std::mem::MaybeUninit::uninit().assume_init() };

    let mut total_len = 0;
    let paths: &[&[u8]] = {
        let mut start_idx = N;
        for i in (0..N).rev() {
            let bytes = paths[i].as_os_str().as_bytes();
            if bytes.len() == 0 {
                continue;
            }
            total_len += bytes.len() + 1;
            start_idx -= 1;
            byte_paths[start_idx].write(bytes);
            if bytes[0] == b'/' {
                break;
            }
        }
        unsafe { core::mem::transmute(&byte_paths[start_idx..]) }
    };
    // If the total length is zero, there's nothing to join, so we can return an empty
    // path
    if total_len == 0 {
        return "".as_ref();
    }

    // If they fit in the raw buffer, we'll join the paths in the raw buffer.
    // Otherwise, we'll put them into the pathbuf.
    if total_len <= raw_buff.len() {
        let mut start = 0;

        for bytes in paths.iter().cloned() {
            let end = start + bytes.len();
            for (i, b) in zip(start..end, bytes.iter().cloned()) {
                raw_buff[i].write(b);
            }
            raw_buff[end].write(b'/');
            start = end + 1;
        }
        // Add a null terminator instead of a slash at the end
        let end_idx = start - 1;
        raw_buff[end_idx].write(b'\0');

        let result = unsafe { std::mem::transmute(&raw_buff[..end_idx]) };
        OsStr::from_bytes(result).as_ref()
    } else {
        let path_buff = if let Some(path) = path_buff {
            path
        } else {
            path_buff.insert(PathBuf::new())
        };
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
    path_buff: &'a mut Option<PathBuf>,
    paths: [&Path; N],
) -> &'a Path {
    // I want to keep the function signature the same between windows and not-windows
    // but this variable isn't used. So I assign it to a variable named _
    // in order to indicate to the compiler that it's not currently used
    let _ = raw_buff;
    let paths = paths.map(|p| p.as_os_str());

    let total_len: usize = paths.iter().map(|x| x.len()).sum();
    let total_len = total_len + N + 1;

    let path_buff = if let Some(path) = path_buff {
        path
    } else {
        path_buff.insert(PathBuf::new())
    };

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
            let mut arr: [std::mem::MaybeUninit<u8>; 128] = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
            let mut buff = None;
            let $name = $crate::join_in_buff(&mut arr, &mut buff, [$($path.as_ref()),+]);
        )*

        $( $statements )*
    }
}
