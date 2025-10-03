use std::hint::black_box;
use criterion::{
    criterion_group, criterion_main, BenchmarkId, Criterion, Throughput, SamplingMode
};
use rust_censure::{Censor, CensorLang};

fn get_bench_cases() -> [(&'static str, &'static str); 3] {
    [
        ("short_dirty", "dumb ass 20"),
        ("short_clean", "hello friend 20"),
        ("long_dirty",  "fuck bitch whore, you dumb ass, foo bar baz; repeated: fuck bitch whore 20"),
    ]
}

fn bench_censor(c: &mut Criterion) {
    let en = Censor::new(CensorLang::En).unwrap();

    let cases = get_bench_cases();

    let mut group = c.benchmark_group("censor_en_clean_line");
    group.sampling_mode(SamplingMode::Flat); // stable timing for fast funcs

    for (name, text) in cases {
        group.throughput(Throughput::Bytes(text.len() as u64)); // throughput MB/s

        group.bench_with_input(BenchmarkId::from_parameter(name), text, |b, input| {
            b.iter(|| {
                let out = en.clean_line(black_box(&input.clone())); // deliberately clone input to simulate real-world use
                black_box(out);
            })
        });
    }

    group.finish();
}

fn bench_censor_pre_compiled(c: &mut Criterion) {
    let en = Censor::new(CensorLang::En).unwrap();
    en.precompile_all_patterns(); // force pre-compile all patterns

    let cases = get_bench_cases();

    let mut group = c.benchmark_group("censor_en_clean_line");
    group.sampling_mode(SamplingMode::Flat); // stable timing for fast funcs

    for (name, text) in cases {
        group.throughput(Throughput::Bytes(text.len() as u64)); // throughput MB/s

        group.bench_with_input(BenchmarkId::from_parameter(name), text, |b, input| {
            b.iter(|| {
                let out = en.clean_line(black_box(&input.clone())); // deliberately clone input to simulate real-world use
                black_box(out);
            })
        });
    }

    group.finish();
}

criterion_group!(benches, bench_censor, bench_censor_pre_compiled);
criterion_main!(benches);
