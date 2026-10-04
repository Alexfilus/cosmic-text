# Cluster letter spacing

Apply tracking once per shaped cluster, preserve base/mark positioning in RTL, and avoid adding tracking inside cursive joins while retaining word-space tracking.

## Provenance and dependencies

- Upstream version: `9e7a56f083db15f67510df4396351464df2e64bd (cosmic-text 0.14.2)`.
- Obscura source commits: `52d2d89`.
- Prerequisite branches: None; independent branch from the version listed above.

## Reproduce

```sh
cargo nextest run --release -p cosmic-text --lib --test cluster_letter_spacing --features shape-run-cache
cargo nextest run --release -p cosmic-text --no-fail-fast --features shape-run-cache
BENCH_ITERS=1000 cargo bench -p cosmic-text --bench cluster_letter_spacing --features shape-run-cache
```

Run `git lfs pull` before testing; the regression uses the upstream Noto font fixtures.

## Validation

Local macOS arm64, Rust 1.98.1, release profile:

- Summary [   1.050s] 17 tests run: 17 passed, 0 skipped
- `cluster-letter-spacing: median=4115 ns/op min=4074.16 max=5137.5 samples=7 iterations=100 (warm font system)`

The benchmark reports seven samples after warm-up. The sample range is included;
these absolute timings are not a before/after speedup claim. Taffy scenarios include
fixture construction. Cosmic-text scenarios reuse a warm FontSystem.
Use a separate target directory per worktree, or clean this package between revisions,
to prevent stale same-version artifacts from another checkout.
