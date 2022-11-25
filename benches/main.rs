use rand::distributions::{Distribution, Uniform};
use std::{path::Path};

use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
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
        group.bench_with_input(BenchmarkId::new("with_paths!", i), &i, |b, i| {
            b.iter(|| {
                let p1 = p1_slices[*i];
                let p2 = p2_slices[*i];

                with_paths! {
                    path = p1 / p2 => black_box(path)
                };
            })
        });

        group.bench_with_input(BenchmarkId::new("Path.join", i), &i, |b, i| {
            b.iter(|| {
                let p1 = p1_slices[*i];
                let p2 = p2_slices[*i];

                black_box(Path::new(p1).join(p2))
            })
        });
    }
}

pub fn join_random(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    const SAMPLE_SIZE: usize = 1 << 17;

    let options = b"abcdef/";
    let opt_dist = Uniform::from(0..options.len());

    let mut group = c.benchmark_group("join");

    for mean_len in (1..12).map(|i| i * 10) {
        let length_dist = Uniform::from(0..mean_len);

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

        let mut i1 = 0;
        let mut i2 = 0;
        group.bench_function(BenchmarkId::new("with_paths!", mean_len), |b| {
            b.iter(|| {
                let p1 = &paths[i1];
                let p2 = &paths[i2];
                with_paths! {
                    path = p1 / p2 => black_box(path)
                };
                i1 = (i1 + 1) % SAMPLE_SIZE;
                i2 = (i2 + 2) % SAMPLE_SIZE;
            })
        });

        i1 = 0; i2 = 0;
        group.bench_function(BenchmarkId::new("Path.join", mean_len), |b| {
            b.iter(|| {
                let p1 = &paths[i1];
                let p2 = &paths[i2];
                black_box(Path::new(p1).join(p2));
                i1 = (i1 + 1) % SAMPLE_SIZE;
                i2 = (i2 + 2) % SAMPLE_SIZE;
            })
        });
    }
}

pub fn exists_random(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    const SAMPLE_SIZE: usize = 1 << 17;

    let options = b"abcdef/";
    let opt_dist = Uniform::from(0..options.len());

    let mut group = c.benchmark_group("exists");

    for mean_len in (1..12).map(|i| i * 10) {
        let length_dist = Uniform::from(0..mean_len);

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

        let mut i1 = 0;
        let mut i2 = 0;
        group.bench_function(BenchmarkId::new("with_paths!", mean_len), |b| {
            b.iter(|| {
                let p1 = &paths[i1];
                let p2 = &paths[i2];
                with_paths! {
                    path = p1 / p2 => black_box(path.exists())
                };
                i1 = (i1 + 1) % SAMPLE_SIZE;
                i2 = (i2 + 2) % SAMPLE_SIZE;
            })
        });

        i1 = 0; i2 = 0;
        group.bench_function(BenchmarkId::new("Path.join", mean_len), |b| {
            b.iter(|| {
                let p1 = &paths[i1];
                let p2 = &paths[i2];
                black_box(Path::new(p1).join(p2).exists());
                i1 = (i1 + 1) % SAMPLE_SIZE;
                i2 = (i2 + 2) % SAMPLE_SIZE;
            })
        });
    }
}

criterion_group!(benches, criterion_benchmark);
criterion_group!(random, join_random, exists_random);
criterion_main!(benches, random);
