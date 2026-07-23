use bc_assist::ballistic::config::SolverConfig;
use bc_assist::ballistic::drag::g7::G7;
use bc_assist::ballistic::trajectory::PointMassSolver;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn trajectory_solve_benchmark(c: &mut Criterion) {
    let solver = PointMassSolver::new(G7, SolverConfig::default());

    c.bench_function("g7_trajectory_300yd", |b| {
        b.iter(|| {
            black_box(solver.solve(2600.0, 300.0));
        })
    });
}

criterion_group!(benches, trajectory_solve_benchmark);
criterion_main!(benches);
