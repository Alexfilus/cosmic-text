#[path = "../tests/support/css_line_breaking.rs"]
mod support;

fn main() {
    support::verify();
    let mut fs = support::fonts();
    let iterations = std::env::var("BENCH_ITERS")
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(100);
    assert!(iterations > 0);
    for _ in 0..10 {
        std::hint::black_box(support::scenario(&mut fs));
    }
    let mut samples = Vec::new();
    for _ in 0..7 {
        let start = std::time::Instant::now();
        for _ in 0..iterations {
            std::hint::black_box(support::scenario(&mut fs));
        }
        samples.push(start.elapsed().as_nanos() as f64 / f64::from(iterations));
    }
    samples.sort_by(f64::total_cmp);
    println!("css-line-breaking: median={} ns/op min={} max={} samples=7 iterations={iterations} (warm font system)", samples[3], samples[0], samples[6]);
}
