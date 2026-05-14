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
