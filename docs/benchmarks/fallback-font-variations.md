# Fallback font variations

Resolve weight, optical size and italic variation requests on the actual fallback face rather than only the requested family.

## Provenance and dependencies

- Upstream version: `9e7a56f083db15f67510df4396351464df2e64bd (cosmic-text 0.14.2)`.
- Obscura source commits: `1781027`.
- Prerequisite branches: `feature-variable-font-instances`

## Reproduce

```sh
cargo nextest run --release -p cosmic-text --lib --test fallback_font_variations --features shape-run-cache
cargo nextest run --release -p cosmic-text --no-fail-fast --features shape-run-cache
BENCH_ITERS=1000 cargo bench -p cosmic-text --bench fallback_font_variations --features shape-run-cache
```

Run `git lfs pull` before the upstream image tests. The new 6 KiB variable-font fixture is stored directly with its OFL license and provenance; it does not require LFS.

## Validation

Local macOS arm64, Rust 1.98.1, release profile:

- Summary [   1.218s] 16 tests run: 16 passed, 0 skipped
- `fallback-font-variations: median=2824.59 ns/op min=2809.17 max=3012.08 samples=7 iterations=100 (warm font system)`

The benchmark reports seven samples after warm-up. The sample range is included;
these absolute timings are not a before/after speedup claim. Taffy scenarios include
fixture construction. Cosmic-text scenarios reuse a warm FontSystem.
Use a separate target directory per worktree, or clean this package between revisions,
to prevent stale same-version artifacts from another checkout.
