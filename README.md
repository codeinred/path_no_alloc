# path_no_alloc: An ergonomic library for stack-allocated paths

Recently I started work on a library that relies very heavily on path
manipulation. After dealing with functions littered with calls to Path.join, I
wanted something faster and more ergonomic. Surely there was a better way,
right?

Enter `path_no_alloc`. It's a simple library for joining and using paths while
avoiding allocation entirely for small paths (paths less than 128 bytes).

Usage is very simple. Inside `with_paths!`, paths are joined with the `/`
operator, if the total length of the paths are less than 128 bytes, the
operation will occur inside a stack-allocated buffer.

```rust
use path_no_alloc::with_paths;

let p1 = "hello";
let p2 = "world";

// Here, we create a variable `path` of type `&Path` that
// represents p1 joined to p2
with_paths! {
    path = p1 / p2
};

assert_eq!(path, std::path::Path::new("hello/world"));
```

`with_paths!` can also be used as a compound expression by appending statements
after the declaration using the `=>` operator:

```rust
use path_no_alloc::with_paths;

let p1 = "some/dir";
let p2 = "file.txt";

let file_exists: bool = with_paths! {
    path = p1 / p2 => path.exists()
};
```

You can have an unlimited number of statements inside a `with_paths!` block, and
you can also create and join as many paths as you want. Anything that implements
`AsRef<Path>` can be used in a declaration and joined to other paths.

```rust
use path_no_alloc::with_paths;
use std::path::{Path, PathBuf};

let p1 = Path::new("hello");
let p2 = "world".to_string();
let my_file = "file.txt";
let some_root = PathBuf::from("some/project/root");

let path_exists = with_paths! {
    path = p1 / p2,
    another_path = some_root / p1 / my_file,
    some_third_path = p1 / p2 / some_root / my_file

    =>

    assert_eq!(path, std::path::Path::new("hello/world"));
    assert_eq!(another_path, std::path::Path::new("some/project/root/hello/file.txt"));
    let path_exists = some_third_path.exists();
    println!("{some_third_path:?} exists? {path_exists}");

    path_exists
};
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
};

// Joining a relative path appends it
assert_eq!(relative, Path::new("my/working/dir/path/to/file.txt"));
// But joining an absolute path just results in the absolute path
assert_eq!(absolute, Path::new("/path/to/file.txt"));

// this is the same as the behavior of Path.join():
assert_eq!(relative, Path::new(working_dir).join(rel_path));
assert_eq!(absolute, Path::new(working_dir).join(abs_path));
```

## Minutae

### Performance

When tested on a set of 131072 random paths, using the `with_paths!` macro is
2-3.5 times faster than calling `Path.join()`, and this is under ideal
conditions `Path.join`. Allocation is fundamentally non-deterministic; it's
subject to contention from multiple threads; and it must be avoided altogether
in the context of certain latency-specific applications.

Using `with_paths!` will avoid allocation in a majority of cases under
real-world conditions (I assume dealing with paths greater than 128 characters
isn't common).

Even when it's necessary to do a system call (such as checking for the existence
of a file), using `with_paths!` can still result in a performance improvement of
20-30%.

(If these images are not displayed in the crate documentation, please view them
at
[github.com/codeinred/path_no_alloc](https://github.com/codeinred/path_no_alloc)

![](docs/benchmarks/join/report/lines.svg)

Shown above is a comparison for the time it takes to join two randomly selected
paths using `Path.join`, versus the time taken to join two paths with
`with_paths!`. The X axis represents the _average_ total length of two randomly
selected paths. Starting with an _average_ path length of 64, there is a
non-zero probability of the two randomly chosen paths having a combined length
greater than 128, resulting in an allocation occuring.

![](docs/benchmarks/join/report/violin.svg)

Shown above is a violin plot of the same data as in the line graph. The final
number in the benchmark ID corresponds to average total length of two randomly
selected paths.

### Have you tested edge cases?

Yes. All of the following edge cases are tested:

- joining 1 or more paths, with one being absolute
- joining 1 or more paths, with some or all of them being empty
- joining paths where the resulting path is exactly the size of the buffer

There is additional fuzzing, wherein combinations of 10 random paths are tested,
with some of the paths being potentially empty, or absolute, and with the length
of the paths potentially exceeding the size of the stack buffer.

See [tests.rs](src/tests.rs#L86)

### What happens if the paths don't fit in the stack buffer?

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

## Why I wrote `path_no_alloc`

Unfortunately unless your app is doing a _lot_ of path manipulations, using
`with_paths!` won't result in meaningful performance improvements on a global
level. Typically, the time taken to join two paths is dwarfed by the operations
done with the resulting path - file creation, IO, reading/writing, or other
interactions with the OS / filesystem.

I wrote this library mainly for my own curiosity, and because I found the
resulting interface to be nicer than calling `Path.join` everywhwere. It was my
first foray into uninitilaized memory and stack-allocated buffers in Rust, and I
can honestly say I learned a lot.

Writing path operations that avoided allocation was a challenge, and one that I
found interesting.

In languages like C++, stack-allocated buffers can be a headache, especially
since most of the time programmers _simply assume that the buffer is
sufficient_ - if the stack buffer is overrun, too bad! Rust's hygenic macros
enable the safe, performant use of stack-allocated buffers, with proper fallback
to heap-allocated memory if the buffer is overrun, **and** they allow you to do
it in a way that's actually _transparent to the programmer_. I incur no
additional mental overhead from using `with_paths!`, because the syntax is
cleaner and more straight-forward than `Path.join`!

With that said, I hope you might find this library useful, or at least
educational.

With love,

— Alecto
