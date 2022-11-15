use std::path::Path;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use path_no_alloc::with_paths;


pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("baseline make array", |b| b.iter(|| {
        let mut arr: [u8; 128] = [0; 128].map(black_box);
        black_box(&mut arr);
    }));


    c.bench_function("path_no_alloc", |b| b.iter(|| {
        let p1 = black_box("pathA");
        let p2 = black_box("pathB");

        with_paths! {
            path = p1 / p2 => black_box(path)
        };
    }));

    c.bench_function("path_join", |b| b.iter(|| {
        let p1 = black_box("pathA");
        let p2 = black_box("pathB");

        let path = Path::new(p1).join(p2);
        black_box(path.as_path());
    }));

    c.bench_function("path_no_alloc longer", |b| b.iter(|| {
        let p1 = black_box("some/longer/path/to/thing");
        let p2 = black_box("another/long/path/to/thing");

        with_paths! {
            path = p1 / p2 => black_box(path)
        };
    }));

    c.bench_function("path_join longer", |b| b.iter(|| {
        let p1 = black_box("some/longer/path/to/thing");
        let p2 = black_box("another/long/path/to/thing");

        let path = Path::new(p1).join(p2);
        black_box(path.as_path());
    }));

    c.bench_function("path_no_alloc longer x 2", |b| b.iter(|| {
        let p1 = black_box("some/longer/path/to/thing/some/longer/path/to/thing");
        let p2 = black_box("another/long/path/to/thing/another/long/path/to/thing");

        with_paths! {
            path = p1 / p2 => black_box(path)
        };
    }));

    c.bench_function("path_join longer x 2", |b| b.iter(|| {
        let p1 = black_box("some/longer/path/to/thing/some/longer/path/to/thing");
        let p2 = black_box("another/long/path/to/thing/another/long/path/to/thing");

        let path = Path::new(p1).join(p2);
        black_box(path.as_path());
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
