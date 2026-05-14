# Slime Browser — Plan 1: Bootstrap (Milestone A0)

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Stand up the Slime Cargo workspace, all empty engine crates, full OSS infrastructure (license, docs, CI, templates), and prove green local + CI builds. End state: an empty, contributor-ready, MPL-2.0 Rust browser project that compiles, tests, lints, and formats cleanly on macOS, Linux, and Windows.

**Architecture:** Cargo workspace with 9 member crates. Each crate is a stub library exposing a single `it_compiles` smoke test. GitHub Actions workflows enforce build, test, clippy, and rustfmt across a 3-OS matrix on every push and PR.

**Tech Stack:** Rust (stable), Cargo workspaces, GitHub Actions, MPL-2.0, Contributor Covenant 2.1.

**Spec source:** `docs/superpowers/specs/2026-05-14-slime-browser-design.md`

---

## Working directory

All commands assume CWD = `/Users/elalitareq/Documents/projects/my_browser`. Engineer should `cd` there before starting.

```bash
cd /Users/elalitareq/Documents/projects/my_browser
```

## File map

Files created in this plan (relative to repo root):

- `Cargo.toml` — workspace root manifest
- `.gitignore`
- `rust-toolchain.toml`
- `rustfmt.toml`
- `clippy.toml`
- `.editorconfig`
- `LICENSE` — MPL-2.0 full text
- `README.md`
- `CLAUDE.md` — project context auto-loaded by Claude Code in future sessions
- `CONTRIBUTING.md`
- `CODE_OF_CONDUCT.md`
- `SECURITY.md`
- `docs/ARCHITECTURE.md`
- `docs/ROADMAP.md`
- `crates/slime/{Cargo.toml,src/main.rs}` — binary
- `crates/slime-{chrome,engine,html,css,dom,layout,paint,net}/{Cargo.toml,src/lib.rs}` — 8 library stubs
- `tests/fixtures/hello.html`
- `examples/render-page.rs`
- `.github/workflows/{ci,lint,release}.yml`
- `.github/ISSUE_TEMPLATE/{bug,feature,rendering-bug}.yml`
- `.github/PULL_REQUEST_TEMPLATE.md`
- `.github/FUNDING.yml`

---

### Task 1: Cargo workspace skeleton + .gitignore

**Files:**
- Create: `Cargo.toml`
- Create: `.gitignore`

- [ ] **Step 1: Create the workspace `Cargo.toml`**

Path: `Cargo.toml`

```toml
[workspace]
resolver = "2"
members = [
    "crates/slime",
    "crates/slime-chrome",
    "crates/slime-engine",
    "crates/slime-html",
    "crates/slime-css",
    "crates/slime-dom",
    "crates/slime-layout",
    "crates/slime-paint",
    "crates/slime-net",
]

[workspace.package]
version = "0.0.1"
edition = "2021"
rust-version = "1.75"
license = "MPL-2.0"
repository = "https://github.com/elalitareq/slime"
homepage = "https://github.com/elalitareq/slime"
authors = ["Slime contributors"]

[workspace.lints.rust]
unsafe_code = "forbid"

[workspace.lints.clippy]
pedantic = { level = "warn", priority = -1 }
nursery = { level = "warn", priority = -1 }
module_name_repetitions = "allow"
missing_errors_doc = "allow"
missing_panics_doc = "allow"

[profile.release]
lto = "thin"
codegen-units = 1
```

- [ ] **Step 2: Create `.gitignore`**

Path: `.gitignore`

```gitignore
/target
**/*.rs.bk
Cargo.lock
.DS_Store
*.swp
.idea/
.vscode/
*.iml
/coverage
```

(Note: we keep `Cargo.lock` out of git for now because the root binary is the only consumer; switch to committed lockfile if/when we publish to crates.io.)

- [ ] **Step 3: Verify the workspace is recognized**

Run:
```bash
cargo metadata --format-version=1 --no-deps >/dev/null && echo OK
```

Expected output: `OK` (no error; member dirs don't exist yet but workspace section is valid; `cargo metadata` may warn but exit 0). If `cargo metadata` errors because members are missing, that's fine — Task 3 creates them. In that case, defer this check to after Task 3.

- [ ] **Step 4: Commit**

```bash
git add Cargo.toml .gitignore
git commit -m "chore: initialize Cargo workspace skeleton"
```

---

### Task 2: License (MPL-2.0)

**Files:**
- Create: `LICENSE`

- [ ] **Step 1: Fetch the canonical MPL-2.0 text**

Run:
```bash
curl -sSL https://www.mozilla.org/media/MPL/2.0/index.815ca599c9df.txt -o LICENSE
```

Verify the file is non-empty and starts with `Mozilla Public License Version 2.0`:
```bash
head -1 LICENSE
```
Expected: `Mozilla Public License Version 2.0`

If `curl` is unavailable or the URL changes, fall back to copy-pasting the MPL-2.0 text from <https://www.mozilla.org/en-US/MPL/2.0/>.

- [ ] **Step 2: Commit**

```bash
git add LICENSE
git commit -m "chore: add MPL-2.0 license"
```

---

### Task 3: Empty crate stubs with smoke tests

Each crate gets a `Cargo.toml` and a single source file with one passing test. The binary (`slime`) gets a `main.rs` that prints a version string.

**Files:**
- Create: `crates/slime/Cargo.toml`
- Create: `crates/slime/src/main.rs`
- Create: `crates/slime-chrome/{Cargo.toml,src/lib.rs,README.md}`
- Create: `crates/slime-engine/{Cargo.toml,src/lib.rs,README.md}`
- Create: `crates/slime-html/{Cargo.toml,src/lib.rs,README.md}`
- Create: `crates/slime-css/{Cargo.toml,src/lib.rs,README.md}`
- Create: `crates/slime-dom/{Cargo.toml,src/lib.rs,README.md}`
- Create: `crates/slime-layout/{Cargo.toml,src/lib.rs,README.md}`
- Create: `crates/slime-paint/{Cargo.toml,src/lib.rs,README.md}`
- Create: `crates/slime-net/{Cargo.toml,src/lib.rs,README.md}`

- [ ] **Step 1: Write the binary crate manifest and entry point**

Path: `crates/slime/Cargo.toml`
```toml
[package]
name = "slime"
description = "Slime browser — main binary"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
authors.workspace = true

[lints]
workspace = true

[[bin]]
name = "slime"
path = "src/main.rs"

[dependencies]
```

Path: `crates/slime/src/main.rs`
```rust
fn main() {
    println!("Slime browser v{}", env!("CARGO_PKG_VERSION"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {}
}
```

- [ ] **Step 2: Write each of the 8 library stubs**

For each library crate `<name>` in the list below, create three files. The eight crate names are:

```
slime-chrome
slime-engine
slime-html
slime-css
slime-dom
slime-layout
slime-paint
slime-net
```

For each `<name>`, create:

Path: `crates/<name>/Cargo.toml`
```toml
[package]
name = "<name>"
description = "Part of the Slime browser project"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
authors.workspace = true

[lints]
workspace = true

[dependencies]
```

Path: `crates/<name>/src/lib.rs`
```rust
//! <name>
//!
//! Placeholder crate. Implementation to be filled in by future plans.

#[cfg(test)]
mod tests {
    #[test]
    fn it_compiles() {}
}
```

Path: `crates/<name>/README.md`
```markdown
# <name>

Part of the [Slime browser](../../README.md) project. Placeholder crate; see
the project [ROADMAP](../../docs/ROADMAP.md) for which milestone fills this in.

## Role

_To be written when this crate gains real functionality._

## License

[MPL-2.0](../../LICENSE).
```

Replace `<name>` with the actual crate name in every file.

- [ ] **Step 3: Verify the whole workspace builds**

Run:
```bash
cargo build --workspace
```
Expected: builds 9 crates, exits 0.

- [ ] **Step 4: Verify all tests pass**

Run:
```bash
cargo test --workspace
```
Expected: 9 `it_compiles` tests pass, 0 failed.

- [ ] **Step 5: Verify the binary runs**

Run:
```bash
cargo run -p slime
```
Expected: prints `Slime browser v0.0.1`.

- [ ] **Step 6: Commit**

```bash
git add crates/
git commit -m "feat: scaffold empty engine crates with smoke tests"
```

---

### Task 4: Top-level docs (README, CLAUDE.md, ARCHITECTURE, ROADMAP)

**Files:**
- Create: `README.md`
- Create: `CLAUDE.md`
- Create: `docs/ARCHITECTURE.md`
- Create: `docs/ROADMAP.md`

- [ ] **Step 1: Write `README.md`**

Path: `README.md`
```markdown
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
```

- [ ] **Step 2: Write `CLAUDE.md`**

This file is automatically loaded by Claude Code each session. Keep it tight — it pays input-token cost every conversation.

Path: `CLAUDE.md`
```markdown
# Slime — Claude Code project context

Project: web browser written from scratch in Rust. License: MPL-2.0.

## Authoritative documents

- Spec: `docs/superpowers/specs/2026-05-14-slime-browser-design.md`
- Roadmap: `docs/ROADMAP.md`
- Architecture: `docs/ARCHITECTURE.md`
- Current plans: `docs/superpowers/plans/`

Read these before answering architecture or scope questions.

## Repo layout

Cargo workspace. Engine code lives in `crates/slime-*`. Binary is `crates/slime`.
Each crate has its own `README.md` describing its role.

## Conventions

- `unsafe_code = "forbid"` workspace-wide. If unsafe becomes necessary, raise it
  in an issue first.
- Clippy `pedantic` and `nursery` lints are on, warnings only. Don't silence
  individual lints without a comment justifying it.
- TDD: write the failing test first, then implementation.
- Frequent, small commits. Conventional Commits (`feat:`, `fix:`, `chore:`,
  `docs:`, `test:`).
- `cargo fmt` and `cargo clippy --workspace -- -D warnings` must pass before
  every PR.

## Tools

- Use the Serena MCP server for symbol lookups across crates — cheaper than
  grep on a multi-crate workspace.
- Use Context7 MCP for live docs of `winit`, `cosmic-text`, `tiny-skia`,
  `hyper`, `rustls`, etc. before reaching for memory.

## Out-of-scope reminders

Until tagged `v0.2.0` we do **not** implement: JavaScript, Flexbox, Grid,
animations, web fonts, iframes, video, audio, canvas, cookies, localStorage.
See spec §4.2 for the full Phase A non-goals list.
```

- [ ] **Step 3: Write `docs/ARCHITECTURE.md`**

Path: `docs/ARCHITECTURE.md`
```markdown
# Slime Architecture

Mirrors the spec at `superpowers/specs/2026-05-14-slime-browser-design.md`.

## Crate graph

```
slime (binary)
 ├── slime-chrome   window, tabs, URL bar
 ├── slime-engine   render pipeline orchestrator
 │    ├── slime-html    HTML5 parser  ─► slime-dom
 │    ├── slime-css     CSS parser + cascade
 │    ├── slime-dom     shared node tree
 │    ├── slime-layout  box tree, block + inline flow
 │    └── slime-paint   display list ► pixels
 └── slime-net      URL, HTTP/1.1, TLS
```

## Data flow (one page load)

1. User enters URL in `slime-chrome`.
2. `slime-net` fetches bytes (HTTP/HTTPS).
3. `slime-html` tokenizes + builds DOM (`slime-dom`).
4. `slime-css` parses stylesheets and runs the cascade.
5. `slime-layout` builds a box tree from the styled DOM.
6. `slime-paint` walks the box tree and rasterizes into a framebuffer.
7. `slime-chrome` blits the framebuffer to the window.

## Why these boundaries?

- Each crate has one job and one set of tests. New contributors can specialize.
- Engine internals depend only on the layer below them — no upward references.
- Swapping out a layer (e.g. CPU paint for GPU paint in Phase C) means changing
  one crate, not the whole engine.
```

- [ ] **Step 4: Write `docs/ROADMAP.md`**

Path: `docs/ROADMAP.md`
```markdown
# Slime Roadmap

Mirrors the spec's milestone table. Update this when milestones land.

## Phase A — Static Web MVP (target: 4–8 months)

| #   | Status    | Milestone                            |
| --- | --------- | ------------------------------------ |
| A0  | 🟡 active | Workspace + CI green                 |
| A1  | ⬜        | Window opens, blank canvas           |
| A2  | ⬜        | Text renders                         |
| A3  | ⬜        | HTML parser (subset)                 |
| A4  | ⬜        | CSS parser + cascade                 |
| A5  | ⬜        | Layout: block flow                   |
| A6  | ⬜        | Layout: inline + text                |
| A7  | ⬜        | Paint: backgrounds + borders         |
| A8  | ⬜        | Local file load                      |
| A9  | ⬜        | HTTP fetch                           |
| A10 | ⬜        | Links clickable, history             |
| A11 | ⬜        | Images (PNG, JPEG)                   |
| A12 | ⬜        | Tabs in chrome                       |
| A13 | ⬜        | URL bar + reload                     |
| A14 | ⬜        | HTTPS via rustls                     |
| A15 | ⬜        | Forms GET submit                     |
| 🏁  | ⬜        | **v0.1.0** — tagged release          |

## Phase B — Dynamic Web

Embedded QuickJS, basic DOM bindings, flexbox, cookies, bookmarks, history UI.
Targets `v0.2.0`.

## Phase C — Full Modern

Own JS engine, GPU rendering (`wgpu`), workers, more web APIs. Targets `v1.0.0`.
```

- [ ] **Step 5: Verify markdown renders (smoke check)**

Run:
```bash
ls README.md CLAUDE.md docs/ARCHITECTURE.md docs/ROADMAP.md
```
Expected: all four paths listed.

- [ ] **Step 6: Commit**

```bash
git add README.md CLAUDE.md docs/ARCHITECTURE.md docs/ROADMAP.md
git commit -m "docs: add README, CLAUDE context, architecture, and roadmap"
```

---

### Task 5: Community docs (CONTRIBUTING, CoC, SECURITY)

**Files:**
- Create: `CONTRIBUTING.md`
- Create: `CODE_OF_CONDUCT.md`
- Create: `SECURITY.md`

- [ ] **Step 1: Write `CONTRIBUTING.md`**

Path: `CONTRIBUTING.md`
```markdown
# Contributing to Slime

Thanks for your interest in Slime. Contributions of any size are welcome.

## 5-minute first PR path

1. `git clone https://github.com/elalitareq/slime && cd slime`
2. `cargo run --example render-page tests/fixtures/hello.html` — should print a
   placeholder line (real rendering lands in later milestones).
3. Pick an issue labeled `good-first-issue`.
4. Open a PR. CI will run on macOS, Linux, and Windows.

## Workflow

- Fork → branch → PR. Keep PRs focused; one logical change per PR.
- Conventional Commits: `feat:`, `fix:`, `chore:`, `docs:`, `test:`, `refactor:`.
- Write tests with TDD: failing test first, then implementation.
- `cargo fmt`, `cargo clippy --workspace -- -D warnings`, and
  `cargo test --workspace` must pass before requesting review.

## Crate ownership

Each crate has a `README.md` describing its role. If you're not sure where a
change belongs, open an issue first and we'll route it.

## Discussion

GitHub Issues for bugs and features. Discord for everything else (link in the
project README once the server is set up).

## Code of Conduct

By participating you agree to follow our [Code of Conduct](CODE_OF_CONDUCT.md).
```

- [ ] **Step 2: Fetch the Contributor Covenant 2.1**

Run:
```bash
curl -sSL https://www.contributor-covenant.org/version/2/1/code_of_conduct/code_of_conduct.md \
  -o CODE_OF_CONDUCT.md
head -1 CODE_OF_CONDUCT.md
```
Expected: line starts with `# Contributor Covenant Code of Conduct` (the canonical title).

If curl fails, copy from <https://www.contributor-covenant.org/version/2/1/code_of_conduct/>.

- [ ] **Step 3: Add contact email to CoC**

Edit `CODE_OF_CONDUCT.md`. Find the placeholder line that contains
`[INSERT CONTACT METHOD]` and replace it with `elalitareq@gmail.com`.

Run:
```bash
grep -F "[INSERT CONTACT METHOD]" CODE_OF_CONDUCT.md
```
Expected: no output (placeholder is gone).

- [ ] **Step 4: Write `SECURITY.md`**

Path: `SECURITY.md`
```markdown
# Security Policy

## Supported versions

Slime is pre-1.0 and only the latest tagged release is supported.

## Reporting a vulnerability

Email **elalitareq@gmail.com** with `[SLIME SECURITY]` in the subject line.
Please do not open public GitHub issues for security reports.

We will:

1. Acknowledge receipt within 7 days.
2. Confirm or dispute the issue within 30 days.
3. Coordinate a fix and disclosure timeline with you.

If you do not get a response in 14 days, please escalate by opening a private
discussion on GitHub Security Advisories.
```

- [ ] **Step 5: Commit**

```bash
git add CONTRIBUTING.md CODE_OF_CONDUCT.md SECURITY.md
git commit -m "docs: add contributing guide, code of conduct, and security policy"
```

---

### Task 6: Toolchain + formatting + editor config

**Files:**
- Create: `rust-toolchain.toml`
- Create: `rustfmt.toml`
- Create: `clippy.toml`
- Create: `.editorconfig`

- [ ] **Step 1: Pin the Rust toolchain**

Path: `rust-toolchain.toml`
```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
profile = "minimal"
```

- [ ] **Step 2: Configure rustfmt**

Path: `rustfmt.toml`
```toml
edition = "2021"
max_width = 100
imports_granularity = "Crate"
group_imports = "StdExternalCrate"
newline_style = "Unix"
use_small_heuristics = "Max"
```

- [ ] **Step 3: Configure clippy thresholds**

Path: `clippy.toml`
```toml
cognitive-complexity-threshold = 30
too-many-arguments-threshold = 8
type-complexity-threshold = 250
```

- [ ] **Step 4: Add `.editorconfig`**

Path: `.editorconfig`
```ini
root = true

[*]
indent_style = space
indent_size = 4
end_of_line = lf
charset = utf-8
trim_trailing_whitespace = true
insert_final_newline = true

[*.{yml,yaml,toml,md}]
indent_size = 2

[Makefile]
indent_style = tab
```

- [ ] **Step 5: Verify formatting and lints pass on the existing stubs**

Run:
```bash
cargo fmt --all --check
```
Expected: exit 0, no output. If it fails, run `cargo fmt --all` and re-check, then investigate which stub file was off-spec.

Run:
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
Expected: exit 0. If clippy complains about a stub, fix it (typically by adding `#![allow(...)]` is the wrong move — read the lint and address it properly).

- [ ] **Step 6: Commit**

```bash
git add rust-toolchain.toml rustfmt.toml clippy.toml .editorconfig
git commit -m "chore: pin toolchain and configure rustfmt, clippy, editorconfig"
```

---

### Task 7: GitHub Actions CI (build + test matrix)

**Files:**
- Create: `.github/workflows/ci.yml`

- [ ] **Step 1: Write the CI workflow**

Path: `.github/workflows/ci.yml`
```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

concurrency:
  group: ci-${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

env:
  CARGO_TERM_COLOR: always
  RUSTFLAGS: "-D warnings"

jobs:
  build-and-test:
    name: build+test (${{ matrix.os }})
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust toolchain
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy

      - name: Cache cargo registry + target
        uses: Swatinem/rust-cache@v2

      - name: cargo build
        run: cargo build --workspace --all-targets

      - name: cargo test
        run: cargo test --workspace --all-targets
```

- [ ] **Step 2: Verify YAML syntax locally**

Run:
```bash
python3 -c "import yaml,sys; yaml.safe_load(open('.github/workflows/ci.yml')); print('OK')"
```
Expected: `OK`.

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/ci.yml
git commit -m "ci: add build+test matrix for macOS, Linux, Windows"
```

---

### Task 8: GitHub Actions lint workflow (rustfmt + clippy)

**Files:**
- Create: `.github/workflows/lint.yml`

- [ ] **Step 1: Write the lint workflow**

Path: `.github/workflows/lint.yml`
```yaml
name: Lint

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

concurrency:
  group: lint-${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

env:
  CARGO_TERM_COLOR: always

jobs:
  rustfmt:
    name: rustfmt
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt
      - run: cargo fmt --all --check

  clippy:
    name: clippy
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy
      - uses: Swatinem/rust-cache@v2
      - run: cargo clippy --workspace --all-targets -- -D warnings
```

- [ ] **Step 2: Verify YAML syntax**

Run:
```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/lint.yml')); print('OK')"
```
Expected: `OK`.

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/lint.yml
git commit -m "ci: add rustfmt and clippy lint workflow"
```

---

### Task 9: Release workflow (tag-triggered binaries)

**Files:**
- Create: `.github/workflows/release.yml`

This workflow only runs when a tag matching `v*` is pushed. Until the first
tag (`v0.1.0`, Plan 7) it has no effect. Add it now so it's already wired up
when the milestone lands.

- [ ] **Step 1: Write the release workflow**

Path: `.github/workflows/release.yml`
```yaml
name: Release

on:
  push:
    tags: ["v*"]

permissions:
  contents: write

jobs:
  build:
    name: build (${{ matrix.target }})
    runs-on: ${{ matrix.os }}
    strategy:
      fail-fast: false
      matrix:
        include:
          - { os: macos-latest,   target: x86_64-apple-darwin,        archive: tar.gz }
          - { os: macos-latest,   target: aarch64-apple-darwin,       archive: tar.gz }
          - { os: ubuntu-latest,  target: x86_64-unknown-linux-gnu,   archive: tar.gz }
          - { os: ubuntu-latest,  target: aarch64-unknown-linux-gnu,  archive: tar.gz }
          - { os: windows-latest, target: x86_64-pc-windows-msvc,     archive: zip }
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}
      - uses: Swatinem/rust-cache@v2
      - name: Build release binary
        run: cargo build --release --locked --target ${{ matrix.target }} -p slime
      - name: Package
        shell: bash
        run: |
          name="slime-${GITHUB_REF_NAME}-${{ matrix.target }}"
          mkdir "$name"
          if [ "${{ matrix.archive }}" = "zip" ]; then
            cp "target/${{ matrix.target }}/release/slime.exe" "$name/"
            cp README.md LICENSE "$name/"
            7z a "$name.zip" "$name"
          else
            cp "target/${{ matrix.target }}/release/slime" "$name/"
            cp README.md LICENSE "$name/"
            tar czf "$name.tar.gz" "$name"
          fi
      - uses: softprops/action-gh-release@v2
        with:
          files: |
            slime-*.tar.gz
            slime-*.zip
```

Note: `aarch64-unknown-linux-gnu` cross-compile on `ubuntu-latest` will fail
without a cross toolchain. That's acceptable for now — when the first real
release approaches (Plan 7), revisit by either adding `cross` or dropping that
target. Tracking issue: file as `good-first-issue` once the repo is public.

- [ ] **Step 2: Verify YAML syntax**

Run:
```bash
python3 -c "import yaml; yaml.safe_load(open('.github/workflows/release.yml')); print('OK')"
```
Expected: `OK`.

- [ ] **Step 3: Commit**

```bash
git add .github/workflows/release.yml
git commit -m "ci: add tag-triggered release workflow for cross-platform binaries"
```

---

### Task 10: Issue templates

**Files:**
- Create: `.github/ISSUE_TEMPLATE/bug.yml`
- Create: `.github/ISSUE_TEMPLATE/feature.yml`
- Create: `.github/ISSUE_TEMPLATE/rendering-bug.yml`
- Create: `.github/ISSUE_TEMPLATE/config.yml`

- [ ] **Step 1: Write the bug template**

Path: `.github/ISSUE_TEMPLATE/bug.yml`
```yaml
name: Bug report
description: Something is broken
labels: [bug, needs-triage]
body:
  - type: textarea
    id: what-happened
    attributes:
      label: What happened?
      description: A clear description of the bug.
    validations:
      required: true
  - type: textarea
    id: repro
    attributes:
      label: Steps to reproduce
    validations:
      required: true
  - type: textarea
    id: expected
    attributes:
      label: Expected behavior
    validations:
      required: true
  - type: input
    id: version
    attributes:
      label: Slime version
      placeholder: "v0.0.1 (commit abc1234)"
    validations:
      required: true
  - type: dropdown
    id: os
    attributes:
      label: Operating system
      options: [macOS, Linux, Windows, Other]
    validations:
      required: true
```

- [ ] **Step 2: Write the feature template**

Path: `.github/ISSUE_TEMPLATE/feature.yml`
```yaml
name: Feature request
description: Suggest something new
labels: [enhancement, needs-triage]
body:
  - type: textarea
    id: problem
    attributes:
      label: Problem this would solve
    validations:
      required: true
  - type: textarea
    id: proposal
    attributes:
      label: Proposed solution
  - type: textarea
    id: alternatives
    attributes:
      label: Alternatives considered
  - type: dropdown
    id: phase
    attributes:
      label: Which roadmap phase does this belong to?
      options: ["Phase A (static MVP)", "Phase B (dynamic web)", "Phase C (modern)", "Unsure"]
    validations:
      required: true
```

- [ ] **Step 3: Write the rendering-bug template**

Path: `.github/ISSUE_TEMPLATE/rendering-bug.yml`
```yaml
name: Rendering bug
description: A page renders differently from a reference browser
labels: [rendering-bug, needs-triage]
body:
  - type: input
    id: url
    attributes:
      label: URL or fixture path
      description: A URL or `tests/fixtures/...` path that reproduces the bug.
    validations:
      required: true
  - type: textarea
    id: slime-screenshot
    attributes:
      label: Slime's rendering
      description: Drag in a screenshot or paste an image.
    validations:
      required: true
  - type: textarea
    id: reference-screenshot
    attributes:
      label: Reference rendering (Firefox or Chrome)
      description: Drag in a screenshot of the same page in Firefox or Chrome.
    validations:
      required: true
  - type: textarea
    id: what-differs
    attributes:
      label: What looks wrong?
    validations:
      required: true
  - type: dropdown
    id: subsystem
    attributes:
      label: Suspected crate (best guess; we'll re-route if wrong)
      options: [slime-html, slime-css, slime-dom, slime-layout, slime-paint, slime-engine, slime-net, slime-chrome, unsure]
    validations:
      required: true
```

- [ ] **Step 4: Write the issue chooser config**

Path: `.github/ISSUE_TEMPLATE/config.yml`
```yaml
blank_issues_enabled: false
contact_links:
  - name: Discussion
    url: https://github.com/elalitareq/slime/discussions
    about: Questions, ideas, and brainstorming. Not bugs.
```

- [ ] **Step 5: Verify all templates parse**

Run:
```bash
python3 -c "
import yaml, pathlib
for p in pathlib.Path('.github/ISSUE_TEMPLATE').glob('*.yml'):
    yaml.safe_load(p.read_text())
    print('OK', p)
"
```
Expected: 4 `OK` lines.

- [ ] **Step 6: Commit**

```bash
git add .github/ISSUE_TEMPLATE/
git commit -m "chore: add issue templates (bug, feature, rendering, config)"
```

---

### Task 11: PR template + FUNDING.yml

**Files:**
- Create: `.github/PULL_REQUEST_TEMPLATE.md`
- Create: `.github/FUNDING.yml`

- [ ] **Step 1: Write the PR template**

Path: `.github/PULL_REQUEST_TEMPLATE.md`
```markdown
## What does this PR do?

<!-- One or two sentences. -->

## Linked issues

<!-- "Closes #123" or "Refs #123". -->

## Changes

<!-- Bullet list of crates touched and what changed in each. -->

## Tests

- [ ] Added or updated tests
- [ ] `cargo test --workspace` passes locally
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes locally
- [ ] `cargo fmt --all --check` passes locally

## Roadmap

<!-- Which milestone (e.g. A3, A14) is this part of? -->

## Notes for reviewers

<!-- Anything tricky, anything you're unsure about. -->
```

- [ ] **Step 2: Write FUNDING.yml**

Path: `.github/FUNDING.yml`
```yaml
github: [elalitareq]
# open_collective: slime-browser   # uncomment once registered
# polar: slime                     # uncomment once registered
```

- [ ] **Step 3: Verify FUNDING.yml parses**

Run:
```bash
python3 -c "import yaml; yaml.safe_load(open('.github/FUNDING.yml')); print('OK')"
```
Expected: `OK`.

- [ ] **Step 4: Commit**

```bash
git add .github/PULL_REQUEST_TEMPLATE.md .github/FUNDING.yml
git commit -m "chore: add PR template and FUNDING.yml"
```

---

### Task 12: Example binary + fixture

The example proves the workspace can build a binary against the library crates,
and gives contributors a one-command entry point as promised in the README.

**Files:**
- Create: `examples/render-page.rs`
- Create: `tests/fixtures/hello.html`
- Modify: `crates/slime/Cargo.toml` — add `[[example]]` section

- [ ] **Step 1: Write the fixture**

Path: `tests/fixtures/hello.html`
```html
<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <title>Hello, Slime</title>
  </head>
  <body>
    <h1>Hello from Slime</h1>
    <p>If you can read this, the fixture loaded.</p>
  </body>
</html>
```

- [ ] **Step 2: Write the example binary**

Path: `examples/render-page.rs`
```rust
//! Stub example. Future milestones fill this in to actually render the page.
//! For now it just confirms the workspace links, reads the file, and prints
//! a summary.

use std::{env, fs, process};

fn main() {
    let mut args = env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("usage: render-page <html-file>");
        process::exit(2);
    };

    let bytes = match fs::read(&path) {
        Ok(b) => b,
        Err(err) => {
            eprintln!("failed to read {path}: {err}");
            process::exit(1);
        }
    };

    println!("loaded {} ({} bytes)", path, bytes.len());
    println!("(rendering not yet implemented; see ROADMAP milestones A1–A8)");
}
```

- [ ] **Step 3: Wire the example up in `crates/slime/Cargo.toml`**

The example lives in the repo-root `examples/` dir, but Cargo finds examples
per-package. Move it under the binary crate so `cargo run --example` works.

Actually, simpler: keep `examples/` at the repo root by listing it in the
binary crate's manifest. Replace the existing `crates/slime/Cargo.toml` with:

Path: `crates/slime/Cargo.toml`
```toml
[package]
name = "slime"
description = "Slime browser — main binary"
version.workspace = true
edition.workspace = true
rust-version.workspace = true
license.workspace = true
repository.workspace = true
homepage.workspace = true
authors.workspace = true

[lints]
workspace = true

[[bin]]
name = "slime"
path = "src/main.rs"

[[example]]
name = "render-page"
path = "../../examples/render-page.rs"

[dependencies]
```

- [ ] **Step 4: Run the example**

Run:
```bash
cargo run -p slime --example render-page -- tests/fixtures/hello.html
```
Expected output:
```
loaded tests/fixtures/hello.html (<some-number> bytes)
(rendering not yet implemented; see ROADMAP milestones A1–A8)
```

- [ ] **Step 5: Commit**

```bash
git add tests/fixtures/hello.html examples/render-page.rs crates/slime/Cargo.toml
git commit -m "feat(slime): add render-page example and hello fixture"
```

---

### Task 13: Final local green-build verification

No new files. Confirm the whole project is in a clean, green state before
pushing to GitHub. Each command's exit code must be 0.

- [ ] **Step 1: `cargo build`**

Run:
```bash
cargo build --workspace --all-targets
```
Expected: exit 0.

- [ ] **Step 2: `cargo test`**

Run:
```bash
cargo test --workspace --all-targets
```
Expected: exit 0; `it_compiles` × 9 plus any new tests, all passing.

- [ ] **Step 3: `cargo clippy`**

Run:
```bash
cargo clippy --workspace --all-targets -- -D warnings
```
Expected: exit 0, no warnings.

- [ ] **Step 4: `cargo fmt --check`**

Run:
```bash
cargo fmt --all --check
```
Expected: exit 0, no output.

- [ ] **Step 5: `git status` clean**

Run:
```bash
git status --short
```
Expected: no output.

- [ ] **Step 6: Tag the bootstrap milestone**

Annotated tag so it shows up in `git tag -n`:

```bash
git tag -a milestone-A0 -m "A0: workspace + CI + OSS infra complete"
```

Verify:
```bash
git tag -n
```
Expected: line beginning with `milestone-A0`.

---

### Task 14: Remote repo + push (manual user steps)

This task is **not auto-executable** because it requires the user to create a
GitHub repository and authenticate. The engineer should hand this off to the
user with the exact steps below.

- [ ] **Step 1: User creates the GitHub repo**

User runs (or does in the GitHub web UI):
```bash
gh repo create elalitareq/slime --public \
  --description "A web browser written from scratch in Rust." \
  --homepage "https://github.com/elalitareq/slime" \
  --license MPL-2.0 \
  --source . --remote origin --push
```

If `gh` is not installed, the user creates the repo in the web UI, then:
```bash
git remote add origin git@github.com:elalitareq/slime.git
git push -u origin main
git push origin milestone-A0
```

- [ ] **Step 2: User enables branch protection on `main`**

In the GitHub repo settings, under "Branches" → "Branch protection rules" for
`main`, require:
- Pull request before merging
- Status checks must pass: `build+test (ubuntu-latest)`, `build+test (macos-latest)`, `build+test (windows-latest)`, `rustfmt`, `clippy`
- Conversation resolution required
- No direct pushes (enforce admins optional)

- [ ] **Step 3: User configures Discussions + Sponsors**

In repo settings:
- Enable Discussions
- Enable Sponsors (links the `.github/FUNDING.yml` entry)
- Set Code of Conduct (auto-detected from `CODE_OF_CONDUCT.md`)

- [ ] **Step 4: Verify CI runs green on the first push**

Open the Actions tab on GitHub. Wait for both `CI` and `Lint` workflows to
finish. Both must be green.

If any job is red, hand back to the engineer with the failure log; do not
proceed to Plan 2 with red CI.

- [ ] **Step 5: Add README CI badges**

After CI runs once, append these badges to the top of `README.md`:

```markdown
[![CI](https://github.com/elalitareq/slime/actions/workflows/ci.yml/badge.svg)](https://github.com/elalitareq/slime/actions/workflows/ci.yml)
[![Lint](https://github.com/elalitareq/slime/actions/workflows/lint.yml/badge.svg)](https://github.com/elalitareq/slime/actions/workflows/lint.yml)
[![License: MPL-2.0](https://img.shields.io/badge/License-MPL_2.0-brightgreen.svg)](LICENSE)
```

Commit:
```bash
git add README.md
git commit -m "docs: add CI and license badges to README"
git push
```

---

## Plan exit criteria

Plan 1 is **done** when:

- All 14 tasks above are checked off.
- The GitHub repo exists, is public, and is licensed MPL-2.0.
- The `milestone-A0` tag is pushed.
- CI is green on `main` for all three OS jobs plus both lint jobs.
- README on GitHub shows green CI and lint badges.
- The `docs/ROADMAP.md` A0 row is updated from `🟡 active` to `✅ done`.

After this plan, Plan 2 (`docs/superpowers/plans/...-p2-window-and-text.md`)
covers milestones A1 and A2 — opening a window and rendering text. Plan 2 is
written from scratch when Plan 1 ships.

---

## Self-review notes

- **Spec coverage:** §3 (architecture: crates exist), §7 (repo layout), §8
  (CI matrix, fmt, clippy, release), §9.1/9.5 (templates, PR welcome, FUNDING),
  §10 (open decisions deferred to Plan 2+) — all addressed in Plan 1 to the
  extent that Plan 1's scope (A0 = bootstrap) requires.
- **Placeholder scan:** none. The example binary is intentionally a stub but
  is fully implemented; future plans replace its body, not its existence.
- **Type consistency:** crate names match across `Cargo.toml`, README, docs,
  and architecture diagrams (`slime-chrome`, `slime-engine`, `slime-html`,
  `slime-css`, `slime-dom`, `slime-layout`, `slime-paint`, `slime-net`, plus
  binary `slime`).
- **Deferred to user:** repo creation, branch protection, badges. Called out
  explicitly in Task 14.
