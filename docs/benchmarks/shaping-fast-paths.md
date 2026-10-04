# Shaping fast paths

Cache shape plans by face, direction, script, language and OpenType features. Varied faces retain their own uncached plans. The LTR fast path belongs to the allocation-reuse branch, not this change.

## Provenance and dependencies

- Upstream version: `9e7a56f083db15f67510df4396351464df2e64bd (cosmic-text 0.14.2)`.
- Obscura source commits: `ecd11e2`.
- Prerequisite branches: `feature-css-line-breaking`, `feature-shaping-dependencies`

## Reproduce

```sh
cargo nextest run --release -p cosmic-text --lib --test shaping_fast_paths --features shape-run-cache
cargo nextest run --release -p cosmic-text --no-fail-fast --features shape-run-cache
BENCH_ITERS=1000 cargo bench -p cosmic-text --bench shaping_fast_paths --features shape-run-cache
```

Run `git lfs pull` before the upstream image tests. The new 6 KiB variable-font fixture is stored directly with its OFL license and provenance; it does not require LFS.

## Validation

Local macOS arm64, Rust 1.98.1, release profile:

- Summary [   1.477s] 16 tests run: 16 passed, 0 skipped
- `shaping-fast-paths: median=7296.25 ns/op min=7106.67 max=7637.08 samples=7 iterations=100 (warm font system)`

The benchmark reports seven samples after warm-up. The sample range is included;
these absolute timings are not a before/after speedup claim. Taffy scenarios include
fixture construction. Cosmic-text scenarios reuse a warm FontSystem.
Use a separate target directory per worktree, or clean this package between revisions,
to prevent stale same-version artifacts from another checkout.

## Paired performance check

Three alternating before/after runs, each with seven samples and 1000 iterations.
The same harness and release compiler were used. Background compilation may add noise.

| Revision | Median ns/op | Sample min/max ns/op | Median peak RSS bytes |
|---|---:|---:|---:|
| base | 26337 | 25562 / 28018 | 7225344 |
| head | 12962 | 12162 / 13490 | 7143424 |

Baseline source commit: `80000d753b1ed0b1e4594d1d670e95fb5cfe851d`.
Paired timing disables `shape-run-cache` on both revisions to measure shape-plan construction rather than whole-run cache hits.

### Recreate the measured baseline

The source-only reverse patch below records the exact measured baseline even if the original local commit is not present in a fresh clone. Apply it only in a separate checkout; it intentionally restores the old behavior.

```sh
git worktree add --detach ../benchmark-baseline feature-shaping-fast-paths
cd ../benchmark-baseline
git apply --unidiff-zero docs/benchmarks/shaping-fast-paths.baseline.patch
BENCH_ITERS=1000 cargo bench -p cosmic-text --bench shaping_fast_paths
```
