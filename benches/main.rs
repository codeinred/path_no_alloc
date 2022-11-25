use std::path::{Path, PathBuf};

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use path_no_alloc::with_paths;

pub fn array_from_idx<const N: usize, T>(f: impl FnMut(usize) -> T) -> [T; N] {
    let mut indices = [0; N];

    for i in 0..N {
        indices[i] = i;
    }

    indices.map(f)
}
pub fn criterion_benchmark(c: &mut Criterion) {

    let p1 = "Call me Ishmael. Some years ago - never mind how long precisely - having little or no money in my purse, and nothing particular to interest me on shore";
    let p2 = "I thought I would sail about a little and see the watery part of the world. It is a way I have of driving off the spleen and regulating the circulation.";

    let p1_slices = array_from_idx::<129, _>(|i| &p1[..i]);
    let p2_slices = array_from_idx::<129, _>(|i| &p2[..i]);

    let mut group = c.benchmark_group("Path join");


    for i in 1..128 {
        group.bench_with_input(BenchmarkId::new("with_paths!", i), &i, |b, i| b.iter(|| {
            let p1 = p1_slices[*i];
            let p2 = p2_slices[*i];

            with_paths! {
                path = p1 / p2 => black_box(path)
            };
        }));

        group.bench_with_input(BenchmarkId::new("Path.join", i), &i, |b, i| b.iter(|| {
            let p1 = p1_slices[*i];
            let p2 = p2_slices[*i];

            black_box(Path::new(p1).join(p2))
        }));

    }
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
