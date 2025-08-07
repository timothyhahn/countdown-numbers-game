use countdown_numbers_game::MinimaxSolver;
use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;

fn bench_simple_solution(c: &mut Criterion) {
    c.bench_function("minimax simple [10,5] -> 15", |b| {
        b.iter(|| {
            let mut solver = MinimaxSolver::new();
            solver.solve(black_box(15), black_box(&[10, 5]))
        })
    });
}

fn bench_medium_solution(c: &mut Criterion) {
    c.bench_function("minimax medium [6,7,7,1,5,8] -> 327", |b| {
        b.iter(|| {
            let mut solver = MinimaxSolver::with_depth(8);
            solver.solve(black_box(327), black_box(&[6, 7, 7, 1, 5, 8]))
        })
    });
}

fn bench_complex_solution(c: &mut Criterion) {
    let mut group = c.benchmark_group("minimax complex");
    group.sample_size(10);
    group.bench_function("[50,25,3,1,10,7] -> 113", |b| {
        b.iter(|| {
            let mut solver = MinimaxSolver::with_depth(8);
            solver.solve(black_box(113), black_box(&[50, 25, 3, 1, 10, 7]))
        })
    });
    group.finish();
}

fn bench_no_solution(c: &mut Criterion) {
    c.bench_function("minimax no solution [10,5] -> 3", |b| {
        b.iter(|| {
            let mut solver = MinimaxSolver::new();
            solver.solve(black_box(3), black_box(&[10, 5]))
        })
    });
}

fn bench_unsolvable_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("minimax unsolvable");
    group.sample_size(10);
    group.bench_function("[1,2,3,4,5,6] -> 999", |b| {
        b.iter(|| {
            let mut solver = MinimaxSolver::with_depth(100);
            solver.solve(black_box(999), black_box(&[1, 2, 3, 4, 5, 6]))
        })
    });
    group.finish();
}

criterion_group!(
    benches,
    bench_simple_solution,
    bench_medium_solution,
    bench_complex_solution,
    bench_no_solution,
    bench_unsolvable_complex,
);
criterion_main!(benches);
