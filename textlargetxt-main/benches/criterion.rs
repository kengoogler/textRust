use criterion::{black_box, criterion_group, criterion_main, Criterion};
use textlargetxt::calculate; // 導入你的 `calculate` 函數

// 定義一個 benchmark 函數
fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("calculate", |b| {
        b.iter(|| calculate(black_box(1000.to_string())))
    });
}

// 使用 `criterion_group` 宏來創建一個 benchmark 測試組
criterion_group!(benches, criterion_benchmark);

// 使用 `criterion_main` 宏來定義主函數，並執行所有的 benchmark 測試組
criterion_main!(benches);
