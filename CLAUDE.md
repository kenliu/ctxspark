# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```bash
cargo build --release          # build optimized binary
cargo build                    # build debug binary
cargo run -- 65                # run without installing
cargo run -- --help            # show CLI help
./target/release/ctxspark 65  # run release binary directly
cargo install --path .         # install to ~/.cargo/bin
cargo test                     # run all tests
```

## Architecture

Single-file implementation in `src/main.rs`. Logic is split into two testable functions with `main()` wiring them to the CLI.

No dependencies. Release profile uses LTO, single codegen unit, and stripped symbols for minimum binary size and startup time.

**Data flow:**
1. `parse_args()` iterates `env::args()` manually into an `Args` struct (value + 6 color/threshold options)
2. `value` is validated to 0–99; thresholds validated (`mid < high`)
3. Block character selected: `BLOCKS[(value * 9 / 100).min(8)]`
4. Foreground color selected by comparing `value` against `mid_threshold` and `high_threshold`
5. Output printed to stdout as `{value}%[{ANSI-colored char}]` with no trailing newline

**ANSI encoding:** `\x1b[38;5;{fg}m\x1b[48;5;{bg}m{char}\x1b[0m` — 256-color fg + bg, then reset.

## Key defaults

| Option | Default | Meaning |
|---|---|---|
| `--low-color` | 82 | bright green |
| `--mid-color` | 226 | yellow |
| `--high-color` | 196 | red |
| `--mid-threshold` | 50 | low→mid boundary |
| `--high-threshold` | 80 | mid→high boundary |
| `--bg-color` | 254 | light grey background |
