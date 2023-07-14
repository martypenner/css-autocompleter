use autocompletion_engine_core::AutocompletionEngine;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn benchmark_get_completions(c: &mut Criterion) {
  let mut engine = AutocompletionEngine::new();

  let files = vec![
    "./__test__/test.atom.io.css".to_string(),
    "./__test__/test.atom.io.css".to_string(),
    "./__test__/test.atom.io.css".to_string(),
  ];

  c.bench_function("get_completions", |b| {
    b.iter(|| {
      engine.get_all_completions_for_files(black_box(files.clone()));
    })
  });
}

criterion_group!(benches, benchmark_get_completions);
criterion_main!(benches);
