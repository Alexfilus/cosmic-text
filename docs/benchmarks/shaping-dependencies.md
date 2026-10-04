# Shaping dependencies

Upgrade rustybuzz 0.14 to 0.20 and ttf-parser 0.21 to 0.25. The regression checks the space advance with simultaneous weight and optical-size variations.

## Provenance and dependencies

- Upstream version: `9e7a56f083db15f67510df4396351464df2e64bd (cosmic-text 0.14.2)`.
- Obscura source commits: `b4d11b4`.
- Prerequisite branches: None; independent branch from the version listed above.

## Reproduce

```sh
cargo nextest run --release -p cosmic-text --lib --test shaping_dependencies --features shape-run-cache
cargo nextest run --release -p cosmic-text --no-fail-fast --features shape-run-cache
BENCH_ITERS=1000 cargo bench -p cosmic-text --bench shaping_dependencies --features shape-run-cache
```

Run `git lfs pull` before the upstream image tests. The new 6 KiB variable-font fixture is stored directly with its OFL license and provenance; it does not require LFS.

## Validation

Local macOS arm64, Rust 1.98.1, release profile:

- Summary [   0.979s] 13 tests run: 13 passed, 0 skipped
- `shaping-dependencies: median=2262.5 ns/op min=2255.83 max=2360.42 samples=7 iterations=100 (warm font system)`

The benchmark reports seven samples after warm-up. The sample range is included;
these absolute timings are not a before/after speedup claim. Taffy scenarios include
fixture construction. Cosmic-text scenarios reuse a warm FontSystem.
Use a separate target directory per worktree, or clean this package between revisions,
to prevent stale same-version artifacts from another checkout.
