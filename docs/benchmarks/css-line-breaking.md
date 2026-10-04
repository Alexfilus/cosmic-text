# Css line breaking

Add optional per-span CSS word-break and overflow-wrap policies, preserving shaped clusters and the distinction between emergency wrapping and min-content.

## Provenance and dependencies

- Upstream version: `9e7a56f083db15f67510df4396351464df2e64bd (cosmic-text 0.14.2)`.
- Obscura source commits: `71193fa`.
- Prerequisite branches: `feature-fallback-font-variations`

## Reproduce

```sh
cargo nextest run --release -p cosmic-text --lib --test css_line_breaking --features shape-run-cache
cargo nextest run --release -p cosmic-text --no-fail-fast --features shape-run-cache
BENCH_ITERS=1000 cargo bench -p cosmic-text --bench css_line_breaking --features shape-run-cache
```

Run `git lfs pull` before the upstream image tests. The new 6 KiB variable-font fixture is stored directly with its OFL license and provenance; it does not require LFS.

## Validation

Local macOS arm64, Rust 1.98.1, release profile:

- Summary [   1.474s] 16 tests run: 16 passed, 0 skipped
- `css-line-breaking: median=11615.83 ns/op min=9857.09 max=12058.75 samples=7 iterations=100 (warm font system)`

The benchmark reports seven samples after warm-up. The sample range is included;
these absolute timings are not a before/after speedup claim. Taffy scenarios include
fixture construction. Cosmic-text scenarios reuse a warm FontSystem.
Use a separate target directory per worktree, or clean this package between revisions,
to prevent stale same-version artifacts from another checkout.
