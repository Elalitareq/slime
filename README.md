# Slime

A web browser, written from scratch in Rust. Open source under [MPL-2.0](LICENSE).

> **Status:** Pre-alpha. Phase A (static-web MVP) in progress. See [ROADMAP](docs/ROADMAP.md).

## What is Slime?

Slime is an experiment in building a usable web browser without reusing Chromium,
WebKit, or Gecko. Engine, layout, paint, network — all written from scratch. The
goal is a daily-driver browser that is small enough to understand and modify.

## Quickstart

```bash
git clone https://github.com/elalitareq/slime
cd slime
cargo run -p slime
```

## Try rendering a fixture page

```bash
cargo run --example render-page tests/fixtures/hello.html
```

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). New contributors: look for issues
labeled `good-first-issue`.

## Architecture

See [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md).

## License

[MPL-2.0](LICENSE).
