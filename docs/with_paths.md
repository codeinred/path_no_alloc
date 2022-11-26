`with_paths!` allows paths to be joined with small path optimization: if the
total length of all joined paths is less tahn 128, no PathBuf will be allocated.

There are two ways to use it:

```rust
# use path_no_alloc::with_paths;
// Declaration mode
let p1 = "path";
let p2 = "to";
let p3 = "thing";

// Declares a variable `my_path` of type &Path
with_paths! {
    my_path = p1 / p2 / p3
};

let path_exists = my_path.exists();
```

```rust
# use path_no_alloc::with_paths;
// Expression mode
let p1 = "path";
let p2 = "to";
let p3 = "thing";

// Declares a variable my_path of type &Path,
// only usable within this block
let my_result = with_paths! {
    my_path = p1 / p2 / p3
    => my_path.exists()
};
```

# Syntax

There are two modes of use for `with_paths!` - declaration
mode, and expression mode.

## Declaration mode

Declares paths usable in outer scope. Here, `with_paths!` simply expands to a
set of statements, and any path variables it declares are usable outside the
macro:

```rust,compile_fail
// This example illustrates syntax

with_paths! {
    <declarations>
};

<statements>
```

### Checking Existence with Declaration Mode

```rust
use path_no_alloc::with_paths;

let p1 = "some_dir";
let p2 = "file.txt";
with_paths! {
    path = p1 / p2
};

// path can be used after it's declared. It's just a variable.
let exists = path.exists();
```

## Expression mode

Declarations followed by statements. Here, `with_paths!` encapsulates a block.
Paths defined by this mode are not usable outside the block.

```rust,compile_fail
// This example illustrates syntax

let my_path_computation = with_paths! { <declarations> => <statements> };
```

### Checking Existence with Expression Mode

```rust
use path_no_alloc::with_paths;

let p1 = "some_dir";
let p2 = "file.txt";
let exists: bool = with_paths! {
    my_path = p1 / p2 => my_path.exists()
};

// `my_path` only exists in the context of the block. It's not in scope anymore.
```

# Constraints

You can use arbitrary objects that are convertible to a Path via `.as_ref()`:

```rust
use path_no_alloc::with_paths;
use std::path::Path;

// Check that a file exists in the given path
fn has_file<P: AsRef<Path>, Q: AsRef<Path>>(path: P, file: Q) -> bool {
    with_paths! {
        full_path = path / file => full_path.exists()
    }
}
```

You can also join arbitrary numbers of paths together:

```rust
use std::path::Path;
use path_no_alloc::with_paths;

let p1 = "path";
let p2 = "to";
let p3 = "thing";

with_paths! {
    path1 = p1 / p2 / p3,
    path2 = p3 / p2 / p1
    => println!("Path1 = {path1:?}, Path2 = {path2:?}")
}
```

This will print:

```output
Path1 = "path/to/thing", Path2 = "thing/to/path"
```

Paths joined in the context of `with_paths!` act as though they've been joined
via `Path.join`. In other words, joining paths A and B will just produce B in
the case that B is absolute:

```rust
use path_no_alloc::with_paths;
use std::path::Path;

let p1 = "some/path";
let p2 = "/absolute/path";

// declare my_path, to be used in following statements
with_paths! {
    my_path = p1 / p2
}

println!("{my_path:?} should equal \"/absolute/path\"");
assert_eq!(my_path, Path::new(p1).join(p2));
assert_eq!(my_path, Path::new("/absolute/path"));
```
