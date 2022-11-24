use std::path::Path;

use crate::with_paths;

#[test]
fn test_with_paths() {
    let p1 = "hello";
    let p2 = "world";
    let p3 = "some/other/path";
    check_paths(p1, p2, p3);
}

#[test]
fn test_with_paths_2() {
    let p1 = "hello/";
    let p2 = "world";
    let p3 = "some/other/path";
    check_paths(p1, p2, p3);
}

#[test]
fn test_with_paths_empty_1() {
    let p1 = "hello";
    let p2 = "";
    let p3 = "some/other/path";
    check_paths(p1, p2, p3);
}

#[test]
fn test_with_paths_abs_1() {
    let p1 = "hello//";
    let p2 = "/world/";
    let p3 = "some/other/path//";
    check_paths(p1, p2, p3);
}

#[test]
fn test_with_paths_abs_2() {
    let p1 = "/hello//";
    let p2 = "/world/";
    let p3 = "some/other/path//";
    check_paths(p1, p2, p3);
}

#[test]
fn test_with_paths_abs_3() {
    let p1 = "//hello//";
    let p2 = "/world/";
    let p3 = "some/other/path//";
    check_paths(p1, p2, p3);
}

#[test]
fn test_with_paths_abs_4() {
    let p1 = "hello//";
    let p2 = "/world/";
    let p3 = "some/other/path";
    let p4 = "/actual_root"; // p4 should be selected as the actual root path

    let result = Path::new(p1).join(p2).join(p3).join(p4);
    with_paths! {
        path = p1 / p2 / p3 / p4
        => assert_eq!(path, result)
    }
}

#[test]
fn test_with_paths_overflow() {
    let p1 = "Call me Ishmael. Some years ago—never mind how long precisely—having little or no money in my purse";
    let p2 = "and nothing particular to interest me on shore";
    let p3 = "/some/root/path/for/good/measure/with/lots/of/slashes/precisely—having little or no money in my purse";

    assert!(p1.len() + p2.len() > 128);
    assert!(p1.len() + p3.len() > 128);
    assert!(p2.len() + p3.len() > 128);

    check_paths(p1, p2, p3)
}


/// CHeck that every combination of 3 paths produces the expected result
fn check_paths(p1: impl AsRef<Path>, p2: impl AsRef<Path>, p3: impl AsRef<Path>) {
    with_paths! {
        path = p1 => assert_eq!(path, p1.as_ref())
    }
    with_paths! {
        path = p2 => assert_eq!(path, p2.as_ref())
    }
    with_paths! {
        path = p3 => assert_eq!(path, p3.as_ref())
    }
    with_paths! {
        path = p1 / p1 => assert_eq!(path, p1.as_ref().join(p1.as_ref()))
    }
    with_paths! {
        path = p1 / p2 => assert_eq!(path, p1.as_ref().join(p2.as_ref()))
    }
    with_paths! {
        path = p1 / p3 => assert_eq!(path, p1.as_ref().join(p3.as_ref()))
    }
    with_paths! {
        path = p2 / p1 => assert_eq!(path, p2.as_ref().join(p1.as_ref()))
    }
    with_paths! {
        path = p2 / p2 => assert_eq!(path, p2.as_ref().join(p2.as_ref()))
    }
    with_paths! {
        path = p2 / p3 => assert_eq!(path, p2.as_ref().join(p3.as_ref()))
    }
    with_paths! {
        path = p3 / p1 => assert_eq!(path, p3.as_ref().join(p1.as_ref()))
    }
    with_paths! {
        path = p3 / p2 => assert_eq!(path, p3.as_ref().join(p2.as_ref()))
    }
    with_paths! {
        path = p3 / p3 => assert_eq!(path, p3.as_ref().join(p3.as_ref()))
    }
    with_paths! {
        path = p1 / p1 / p1 => assert_eq!(path, p1.as_ref().join(p1.as_ref()).join(p1.as_ref()))
    }
    with_paths! {
        path = p1 / p1 / p2 => assert_eq!(path, p1.as_ref().join(p1.as_ref()).join(p2.as_ref()))
    }
    with_paths! {
        path = p1 / p1 / p3 => assert_eq!(path, p1.as_ref().join(p1.as_ref()).join(p3.as_ref()))
    }
    with_paths! {
        path = p1 / p2 / p1 => assert_eq!(path, p1.as_ref().join(p2.as_ref()).join(p1.as_ref()))
    }
    with_paths! {
        path = p1 / p2 / p2 => assert_eq!(path, p1.as_ref().join(p2.as_ref()).join(p2.as_ref()))
    }
    with_paths! {
        path = p1 / p2 / p3 => assert_eq!(path, p1.as_ref().join(p2.as_ref()).join(p3.as_ref()))
    }
    with_paths! {
        path = p1 / p3 / p1 => assert_eq!(path, p1.as_ref().join(p3.as_ref()).join(p1.as_ref()))
    }
    with_paths! {
        path = p1 / p3 / p2 => assert_eq!(path, p1.as_ref().join(p3.as_ref()).join(p2.as_ref()))
    }
    with_paths! {
        path = p1 / p3 / p3 => assert_eq!(path, p1.as_ref().join(p3.as_ref()).join(p3.as_ref()))
    }
    with_paths! {
        path = p2 / p1 / p1 => assert_eq!(path, p2.as_ref().join(p1.as_ref()).join(p1.as_ref()))
    }
    with_paths! {
        path = p2 / p1 / p2 => assert_eq!(path, p2.as_ref().join(p1.as_ref()).join(p2.as_ref()))
    }
    with_paths! {
        path = p2 / p1 / p3 => assert_eq!(path, p2.as_ref().join(p1.as_ref()).join(p3.as_ref()))
    }
    with_paths! {
        path = p2 / p2 / p1 => assert_eq!(path, p2.as_ref().join(p2.as_ref()).join(p1.as_ref()))
    }
    with_paths! {
        path = p2 / p2 / p2 => assert_eq!(path, p2.as_ref().join(p2.as_ref()).join(p2.as_ref()))
    }
    with_paths! {
        path = p2 / p2 / p3 => assert_eq!(path, p2.as_ref().join(p2.as_ref()).join(p3.as_ref()))
    }
    with_paths! {
        path = p2 / p3 / p1 => assert_eq!(path, p2.as_ref().join(p3.as_ref()).join(p1.as_ref()))
    }
    with_paths! {
        path = p2 / p3 / p2 => assert_eq!(path, p2.as_ref().join(p3.as_ref()).join(p2.as_ref()))
    }
    with_paths! {
        path = p2 / p3 / p3 => assert_eq!(path, p2.as_ref().join(p3.as_ref()).join(p3.as_ref()))
    }
    with_paths! {
        path = p3 / p1 / p1 => assert_eq!(path, p3.as_ref().join(p1.as_ref()).join(p1.as_ref()))
    }
    with_paths! {
        path = p3 / p1 / p2 => assert_eq!(path, p3.as_ref().join(p1.as_ref()).join(p2.as_ref()))
    }
    with_paths! {
        path = p3 / p1 / p3 => assert_eq!(path, p3.as_ref().join(p1.as_ref()).join(p3.as_ref()))
    }
    with_paths! {
        path = p3 / p2 / p1 => assert_eq!(path, p3.as_ref().join(p2.as_ref()).join(p1.as_ref()))
    }
    with_paths! {
        path = p3 / p2 / p2 => assert_eq!(path, p3.as_ref().join(p2.as_ref()).join(p2.as_ref()))
    }
    with_paths! {
        path = p3 / p2 / p3 => assert_eq!(path, p3.as_ref().join(p2.as_ref()).join(p3.as_ref()))
    }
    with_paths! {
        path = p3 / p3 / p1 => assert_eq!(path, p3.as_ref().join(p3.as_ref()).join(p1.as_ref()))
    }
    with_paths! {
        path = p3 / p3 / p2 => assert_eq!(path, p3.as_ref().join(p3.as_ref()).join(p2.as_ref()))
    }
    with_paths! {
        path = p3 / p3 / p3 => assert_eq!(path, p3.as_ref().join(p3.as_ref()).join(p3.as_ref()))
    }
}
