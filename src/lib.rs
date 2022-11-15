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
    let paths = paths.map(|p| p.as_os_str().as_bytes());

    let total_len: usize = paths.iter().map(|x| x.len()).sum();
    let total_len = total_len + N + 1;

    if total_len <= raw_buff.len() {
        let mut start = 0;

        for path in paths {
            let end = start + path.len();
            raw_buff[start..end].copy_from_slice(path);
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

#[macro_export]
macro_rules! with_paths {
    {
        $( $name:ident = $( $path:ident ) / + ),*
        => $( $statements:stmt );*
    } => {
        $(
            let mut arr: [u8; 128] = [0; 128];
            let mut buff = ::std::path::PathBuf::new();
            let $name = $crate::join_in_buff(&mut arr, &mut buff, [$($path.as_ref()),+]);
        )*

        $( $statements )*
    }
}
