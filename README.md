# path_no_alloc: An ergonomic library for stack-allocated paths

Recently I started work on a library that relies very heavily on path
manipulation. After dealing with functions littered with calls to Path.join, I
wanted something faster and more ergonomic. Surely there was a better way,
right?

Enter `path_no_alloc`. It's a simple library for joining and using paths while
avoiding allocation entirely for small paths (paths less than 128 characters).

Usage is very simple. Given two paths, you can join them and use them in the
context of a `with_paths!` block:

```rust
use path_no_alloc::with_paths;

let p1 = "hello";
let p2 = "world";

with_paths! {
    // Here, we create a variable `path` of type `&Path` that
    // represents p1 joined to p2
    path = p1 / p2 =>

    assert_eq!(path, std::path::Path::new("hello/world"))
}
```

You can have an unlimited number of statements inside a `with_paths!` block, and
you can also create and join as many paths as you want. Additionally, a path may
be anything that implements `AsRef<Path>`:

```rust
use path_no_alloc::with_paths;
use std::path::{Path, PathBuf};

let p1 = Path::new("hello");
let p2 = "world".to_string();
let my_file = "file.txt";
let some_root = PathBuf::from("some/project/root");

with_paths! {
    // Here, we create a variable `path` of type `&Path` that
    // represents p1 joined to p2
    path = p1 / p2,
    another_path = some_root / p1 / my_file,
    some_third_path = p1 / p2 / some_root / my_file
    =>

    assert_eq!(path, std::path::Path::new("hello/world"));
    assert_eq!(another_path, std::path::Path::new("some/project/root/hello/file.txt"));

    let path_exists = some_third_path.exists();
    println!("{some_third_path:?} exists? {path_exists}");
}
```

Finally, paths joined in a `with_paths!` block behave identically to paths
joined with `Path.join`. This means that joining an absolute path to another
path truncates and just returns the absolute path:

```rust
use path_no_alloc::with_paths;
use std::path::Path;

let working_dir = "my/working/dir";
let rel_path = "path/to/file.txt";
let abs_path = "/path/to/file.txt";

with_paths! {
    relative = working_dir / rel_path,
    absolute = working_dir / abs_path
    =>

    // Joining a relative path appends it
    assert_eq!(relative, Path::new("my/working/dir/path/to/file.txt"));
    // But joining an absolute path just results in the absolute path
    assert_eq!(absolute, Path::new("/path/to/file.txt"));

    // this is the same as the behavior of Path.join():
    assert_eq!(relative, Path::new(working_dir).join(rel_path));
    assert_eq!(absolute, Path::new(working_dir).join(abs_path));
}
```

## Have you tested edge cases?

Yes. All of the following edge cases are tested:

- joining 1 or more paths, with one being absolute
- joining 1 or more paths, with some or all of them being empty
- joining paths where the resulting path is exactly the size of the buffer

There is additional fuzzing, wherein combinations of 10 random paths are tested,
with some of the paths being potentially empty, or absolute, and with the length
of the paths potentially exceeding the size of the stack buffer.

See [tests.rs](src/tests.rs#L86)

## What happens if the paths don't fit in the stack buffer?

If the paths don't fit in the stack buffer, then `with_paths!` will compute the
combined length of all paths; reserve a `PathBuf` with the appropriate size; and
join the paths using that `PathBuf`. This operation is entirely transparent to
the user, and when doing path manipulations, you'll still be doing them with a
`&Path`.

Note that this is _still_ more efficient than `Path::new(a).join(b)`, because
the typical call to `Path.join` will result in 2 allocations:

- first it will create a PathBuf containing `a`,
- then it will push `b` onto the pathbuf.

Unfortunately, `Path.join` does not allocate enough space upfront, resulting in
the second allocation.

I hope to submit a bug fix to the standard library regarding this issue.
