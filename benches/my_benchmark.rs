use criterion::{black_box, criterion_group, criterion_main, Criterion};
use headercmp::{is_header_const, is_header_hash};

const MESSAGE: &str = "타임 스탬프 파일 이름   오류  메시지";

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("is_header hash version", |b| {
        b.iter(|| is_header_hash(black_box(MESSAGE)))
    });

    c.bench_function("is_header const version", |b| {
        b.iter(|| is_header_const(black_box(MESSAGE)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
