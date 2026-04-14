# crabtime-path-dep

Minimal Rust workspace used to reproduce and test a `crabtime` dependency setup.

It contains two local crates, `a` and `b`, and currently points to `mirsella/crabtime` on the `local-dep` branch.

## What This Repo Is

This is not an end-user library or application. It is a focused reproduction workspace meant to isolate dependency and proc-macro behavior with as little unrelated code as possible.

Crate `a` depends on both local crate `b` and the git-sourced `crabtime` dependency, which makes it useful for testing mixed path/git dependency graphs.

## Check the workspace

```bash
cargo check
```

## Build

```bash
cargo build
```

## Notes

- The interesting part is the dependency wiring in the root `Cargo.toml`.
- The implementation code is intentionally tiny.
- This repo is meant for reproducing behavior, not for shipping a reusable package.
