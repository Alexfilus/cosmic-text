# Shaping allocation reuse

Intern shape-run attributes, reuse scratch storage, bypass bidi analysis for provably pure LTR input, and preserve painting/wrapping of words longer than 16384 glyphs. The timing workload uses shorter text so both revisions perform equivalent useful work.

## Provenance and dependencies

- Upstream version: `9e7a56f083db15f67510df4396351464df2e64bd (cosmic-text 0.14.2)`.
- Obscura source commits: `5366f6a`.
- Prerequisite branches: `feature-shaping-fast-paths`

## Reproduce

```sh
cargo nextest run --release -p cosmic-text --lib --test shaping_allocation_reuse --features shape-run-cache
cargo nextest run --release -p cosmic-text --no-fail-fast --features shape-run-cache
BENCH_ITERS=1000 cargo bench -p cosmic-text --bench shaping_allocation_reuse --features shape-run-cache
```

Run `git lfs pull` before the upstream image tests. The new 6 KiB variable-font fixture is stored directly with its OFL license and provenance; it does not require LFS.

## Validation

Local macOS arm64, Rust 1.98.1, release profile:

- Summary [   0.726s] 19 tests run: 19 passed, 0 skipped
- `shaping-allocation-reuse: median=62024.16 ns/op min=58720 max=62635.41 samples=7 iterations=100 (warm font system)`

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
| base | 134226 | 131901 / 137714 | 7782400 |
| head | 61778 | 60741 / 62315 | 7503872 |

Baseline source commit: `4d8d70ff62d24c2fcf89390df4611e76d8423801`.
Paired timing enables `shape-run-cache` on both revisions and passes `--skip-verification` to both timing executables, because the separate long-word regression fails on the baseline and would distort peak RSS. Correctness is checked independently.

### Recreate the measured baseline

The source-only reverse patch below records the exact measured baseline even if the original local commit is not present in a fresh clone. Apply it only in a separate checkout; it intentionally restores the old behavior.

```sh
git worktree add --detach ../benchmark-baseline feature-shaping-allocation-reuse
cd ../benchmark-baseline
git apply --unidiff-zero docs/benchmarks/shaping-allocation-reuse.baseline.patch
BENCH_ITERS=1000 cargo bench -p cosmic-text --bench shaping_allocation_reuse --features shape-run-cache -- --skip-verification
```
