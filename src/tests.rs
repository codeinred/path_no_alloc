use std::path::{Path, PathBuf};

use rand::{distributions::Uniform, prelude::Distribution};

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

#[test]
fn test_fuzz() {
    let mut rng = rand::thread_rng();
    const SAMPLE_SIZE: usize = 100000;
    const SAMPLES: usize = 100_000;

    let options = b"abcdef/";

    let length_dist = Uniform::from(0..24);
    let opt_dist = Uniform::from(0..options.len());

    let mut paths: Vec<String> = vec![];
    paths.reserve(SAMPLE_SIZE);

    for _ in 0..SAMPLE_SIZE {
        let len = length_dist.sample(&mut rng);

        let s: String = opt_dist
            .sample_iter(&mut rng)
            .take(len)
            .map(|i| options[i] as char)
            .collect();
        paths.push(s);
    }

    let path_dist = Uniform::from(0..paths.len()).map(|i| paths[i].as_str());

    let mut n127 = 0;
    let mut result = PathBuf::new();
    for _ in 0..SAMPLES {
        let p0 = path_dist.sample(&mut rng);
        let p1 = path_dist.sample(&mut rng);
        let p2 = path_dist.sample(&mut rng);
        let p3 = path_dist.sample(&mut rng);
        let p4 = path_dist.sample(&mut rng);
        let p5 = path_dist.sample(&mut rng);
        let p6 = path_dist.sample(&mut rng);
        let p7 = path_dist.sample(&mut rng);
        let p8 = path_dist.sample(&mut rng);
        let p9 = path_dist.sample(&mut rng);

        result.clear();
        result.push(p0);
        result.push(p1);
        result.push(p2);

        with_paths! {
            path =p0/p1/p2 => assert_eq!(path, result)
        }

        result.push(p3);
        result.push(p4);
        result.push(p5);

        with_paths! {
            path =p0/p1/p2/p3/p4/p5 => assert_eq!(path, result)
        }

        result.push(p6);
        result.push(p7);
        result.push(p8);
        result.push(p9);

        let len = result.as_os_str().len();
        if len >= 127 {
            n127 += 1;
        }
        with_paths! {
            path =p0/p1/p2/p3/p4/p5/p6/p7/p8/p9 => assert_eq!(path, result)
        }
    }
    let p = n127 as f64 / SAMPLES as f64;
    println!("Portion >= 127: {p}")
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
