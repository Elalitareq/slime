# Slime Browser — Design Spec

- **Project:** Slime
- **License:** MPL-2.0
- **Language:** Rust
- **Date:** 2026-05-14
- **Status:** Approved for planning
- **Author:** elalitareq + Claude (brainstorm session)

## 1. Intent

Slime is an open-source, cross-platform web browser built from scratch in
Rust. The browser engine (HTML parser, CSS, layout, paint, network) is
written from the ground up rather than reusing Chromium/WebKit/Gecko. The
project prioritizes learning, hackability, and community participation
over speed-to-feature-parity with mainstream browsers.

The license (MPL-2.0) prevents closed-source forks of Slime's own code
while remaining friendly to mixing with other licenses, matching the
posture of Firefox and Servo.

## 2. Non-Goals

- Daily-driver replacement for Chrome/Firefox at v0.1 — that is a v1.0
  ambition, years away
- Performance parity with Blink/Gecko/WebKit
- Bug-for-bug compatibility with mainstream browsers
- Mobile platforms in any phase before v1.0
- Bundled DRM, telemetry, account systems, sync

## 3. Architecture

Cargo workspace. Each subsystem is its own crate so contributors can
specialize and so each unit can be tested in isolation.

```
slime/                  # binary, app entry
├── slime-chrome        # window, tabs, URL bar (winit + softbuffer)
├── slime-engine        # render pipeline orchestrator
├── slime-html          # HTML5 tokenizer + tree builder
├── slime-css           # CSS parser + selector match + cascade
├── slime-dom           # DOM node tree (shared by engine, later JS)
├── slime-layout        # box tree, block + inline flow
├── slime-paint         # display list → pixels (tiny-skia)
└── slime-net           # URL, HTTP/1.1, TLS
```

### Data flow

```
URL → slime-net → bytes
bytes → slime-html → DOM (slime-dom)
DOM + stylesheets → slime-css → styled DOM
styled DOM → slime-layout → box tree
box tree → slime-paint → display list → pixels
pixels → slime-chrome → window
```

### Dependency policy

Engine internals (parser, layout, paint) prefer building from scratch for
learning value. External crates are acceptable for:

- Window + input: `winit`
- Software framebuffer: `softbuffer`
- 2D rasterization: `tiny-skia`
- Text shaping + layout: `cosmic-text`
- Image decoding: `image`
- HTTP + TLS: `hyper` + `rustls` (Phase A.5 for HTTPS)

Decisions on `html5ever` / `cssparser` vs from-scratch parsers are
deferred to planning. Default is from-scratch; switch if blocking
progress.

## 4. Phase A — Static Web MVP

### 4.1 In scope

**HTML elements:** doctype, `<html>`, `<head>`, `<body>`, `<title>`,
`<meta>`, headings `<h1>`–`<h6>`, `<p>`, `<div>`, `<span>`, `<a>`,
`<img>`, `<ul>`, `<ol>`, `<li>`, `<table>` (basic rows/cells, no
colspan/rowspan), `<form>` (GET only), `<input type="text|submit">`,
`<button>`, `<br>`, `<hr>`.

**CSS:**

- Selectors: type, class, id, descendant, `:hover`
- Box model: `margin`, `border`, `padding`, `width`, `height`
- Display: `block`, `inline`, `inline-block`, `none`
- Color: `color`, `background-color` (named + hex + `rgb()`)
- Font: `font-size`, `font-family`, `font-weight`
- Text: `text-align`

**Text:** UTF-8, basic line breaking, system fonts via `cosmic-text`.

**Images:** PNG, JPEG via `image` crate.

**Navigation:** click `<a>` → fetch + render. Back / forward / reload.

**Forms:** GET submit (encodes inputs into query string and navigates).

**Network:** HTTP first, then HTTPS via `rustls` (milestone A14).

**Chrome:** single window, multiple tabs, tab strip, URL bar, back /
forward / reload, status bar (URL on hover). No bookmarks, no history
UI, no settings.

### 4.2 Out of scope (Phase A explicit non-goals)

- JavaScript (Phase B)
- Flexbox, Grid (Phase B)
- Video, audio, canvas, SVG
- Cookies, localStorage, history persistence across runs
- Animations, transitions
- Web fonts (`@font-face`)
- Iframes
- Accessibility tree

### 4.3 Acceptance test sites

1. `http://example.com`
2. `http://info.cern.ch`
3. A static Wikipedia article (HTTPS gate — milestone A14)
4. An MDN HTML reference page (HTTPS gate)
5. Hand-written fixtures in `tests/fixtures/`

For each, Slime's render is compared (manually first, then via golden
images in CI) against Firefox's render. "Pass" = same content, same
boxes, acceptable typography drift.

### 4.4 Milestones

| #   | Milestone                            | Demo                                       |
| --- | ------------------------------------ | ------------------------------------------ |
| A0  | Workspace + CI green                 | empty crates, `cargo test` passes          |
| A1  | Window opens, blank canvas           | `winit` + `softbuffer` show black square   |
| A2  | Text renders                         | hardcoded "Hello" painted via `cosmic-text` |
| A3  | HTML parser (subset)                 | parse `<h1>hi</h1>` → DOM, debug-print     |
| A4  | CSS parser + cascade                 | apply `h1 { color: red }` to DOM           |
| A5  | Layout: block flow                   | nested divs stack vertically               |
| A6  | Layout: inline + text                | paragraphs wrap, headings size right       |
| A7  | Paint: backgrounds + borders         | colored boxes with borders                 |
| A8  | Local file load                      | `slime file:///path/test.html` renders     |
| A9  | HTTP fetch                           | `slime http://info.cern.ch` works          |
| A10 | Links clickable, history             | back / forward                             |
| A11 | Images (PNG, JPEG)                   | `<img>` renders                            |
| A12 | Tabs in chrome                       | multiple pages, switch                     |
| A13 | URL bar + reload                     | full chrome interaction                    |
| A14 | HTTPS via `rustls`                   | wikipedia.org no-JS page renders           |
| A15 | Forms GET submit                     | search box on test page works              |
| 🏁  | **A-Release: `v0.1.0`** tagged       | binaries for mac, linux, windows           |

## 5. Phase B — Dynamic Web

- B1: Flexbox layout
- B2: Embed QuickJS (small, MIT, easy bindings)
- B3: Minimal DOM bindings (`getElementById`, `addEventListener`,
  `innerHTML`)
- B4: `fetch()` API
- B5: Cookies, localStorage
- B6: More CSS (grid subset, transforms, basic animations)
- B7: SVG basic
- B8: Bookmarks, history, downloads in chrome
- 🏁 **B-Release: `v0.2.0`** — renders many real sites, breaks on SPAs

## 6. Phase C — Full Modern

- C1: Migrate to GPU rendering (`wgpu`)
- C2: Own JS engine (`slime-js`); replace QuickJS gradually
- C3: Async event loop, promises, microtasks
- C4: More web APIs (Canvas, WebSocket, IndexedDB subset)
- C5: Web fonts, video, audio
- C6: Workers, Service Workers
- C7: Extensions API (subset)
- 🏁 **C-Release: `v1.0.0`** — daily-driver capable

## 7. Repository Layout

```
slime/
├── Cargo.toml                  # workspace root
├── README.md
├── LICENSE                     # MPL-2.0 full text
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md          # Contributor Covenant 2.1
├── SECURITY.md
├── .github/
│   ├── workflows/
│   │   ├── ci.yml              # mac/linux/windows × stable
│   │   ├── lint.yml            # rustfmt + clippy -D warnings
│   │   └── release.yml         # tag → cross-platform binaries
│   ├── ISSUE_TEMPLATE/
│   │   ├── bug.yml
│   │   ├── feature.yml
│   │   └── rendering-bug.yml   # URL + screenshot required
│   ├── PULL_REQUEST_TEMPLATE.md
│   └── FUNDING.yml             # GitHub Sponsors + Open Collective
├── docs/
│   ├── ARCHITECTURE.md
│   ├── ROADMAP.md
│   └── superpowers/specs/      # this file lives here
├── crates/
│   ├── slime/
│   ├── slime-chrome/
│   ├── slime-engine/
│   ├── slime-html/
│   ├── slime-css/
│   ├── slime-dom/
│   ├── slime-layout/
│   ├── slime-paint/
│   └── slime-net/
├── tests/
│   └── fixtures/
├── examples/
│   └── render-page.rs
├── rustfmt.toml
├── clippy.toml
└── .editorconfig
```

## 8. CI + Tooling

- **CI matrix:** ubuntu-latest, macos-latest, windows-latest × stable
  Rust. Cache `~/.cargo` and `target/`.
- **CI steps:** `cargo build --workspace`,
  `cargo test --workspace`,
  `cargo clippy --workspace --all-targets -- -D warnings`,
  `cargo fmt --all --check`.
- **Release:** Tag `vX.Y.Z` triggers cross-platform binary build and
  GitHub Release. Binaries: macOS (universal), Linux x86_64 + aarch64,
  Windows x86_64.
- **Golden image diffs:** `slime-paint` integration tests render fixture
  pages and diff against committed reference PNGs (allow small
  per-pixel delta).
- **Versioning:** SemVer, pre-1.0 (`0.x.y`) until Phase C ships.

## 9. Community + Growth Infra

### 9.1 Automated updates

| Tool                                | Job                                                                                                          |
| ----------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| GitHub Actions → Discord webhook    | post on new release, merged PR, first-time contributor's PR, milestone closed                                |
| `slime-bot` (small service)         | weekly digest in `#progress`: commits this week, contributors, open issues, milestone % done. Pulls GH API.   |
| Render-bot                          | comments on PRs touching layout/paint with golden-image diffs vs Firefox screenshots                          |
| `release-drafter` GH Action         | auto-builds `CHANGELOG.md` from PR labels                                                                     |
| Cross-poster (X / Mastodon / Bsky)  | optional, same webhook                                                                                       |
| `mdbook` dev-blog                   | deployed to GitHub Pages; one post per milestone; RSS for subscribers                                        |

Hosting: GH Actions cron or fly.io free tier. Target operating cost
~$0/month until scale demands otherwise.

### 9.2 Discord server structure

- `#announcements` (bot-only)
- `#progress-feed` (bot: commits / PRs)
- `#general`
- `#help` (users running Slime)
- `#contributors`
- `#design` (UX / chrome decisions)
- `#good-first-issue` (bot mirrors GH label)
- `#rendering-bugs` (paste URL + screenshot)

Role bot: users self-select `crate:html`, `crate:css`, etc.; get pinged
on issues for that crate.

### 9.3 Sponsorship + funding

| Channel                 | Setup                                                                  |
| ----------------------- | ---------------------------------------------------------------------- |
| GitHub Sponsors         | `.github/FUNDING.yml`; tier rewards (logo in README, Discord role)     |
| Open Collective         | optional, easier for org sponsors                                       |
| Polar.sh                | issue bounties — sponsors pay to prioritize specific bugs              |
| `thanks.dev`, `drips`   | passive split with upstream crate maintainers                          |
| Corporate sponsors page | `SPONSORS.md` with logos                                               |
| Grants                  | NLnet, Sovereign Tech Fund, Internet Society — apply after A0–A5 demo |

### 9.4 Growth flywheel

1. Each milestone (A1, A2, …) = blog post + Discord drop + tweet with
   screenshot/video
2. Demo videos drive sharing — invest in good GIFs/clips per milestone
3. Hacker News submission on `v0.1.0`, not before — needs visual hook
4. Optional weekly Show & Tell streams (Twitch / YouTube)
5. Track stars + contributors with README badges and `repobeats` graph

### 9.5 Onboarding contributors

- `CONTRIBUTING.md` has 5-minute "first PR" path: clone → `cargo run
  --example render-page tests/fixtures/hello.html`
- Bot welcomes each new PR with a checklist
- "Mentor of the week" rotation triages new contributors
- Streamed pair-coding sessions on hard bugs (optional)

## 10. Open Decisions (resolve in planning)

- **HTML parser:** from-scratch vs `html5ever`. Default: from-scratch
  for Phase A learning value; escape hatch to `html5ever` if blocking.
- **CSS parser:** same question, same default.
- **HTTP stack:** `hyper`+`rustls` vs custom. Default: use `hyper` +
  `rustls`; rewriting HTTP is not the interesting part of this project.
- **Text rendering:** `cosmic-text` (recommended) vs `fontdue` + custom
  shaping.
- **Font fallback strategy** for Phase A.

These decisions belong in the implementation plan (next step), not in
this spec.

## 11. Acceptance Criteria for the Spec Itself

This spec is "done" when:

- Architecture is committed to (Section 3)
- Phase A scope is locked (Section 4)
- Phase B/C exist as direction, not commitment (Sections 5–6)
- Repo layout + CI + license are decided (Sections 7–8)
- Community plan exists with concrete first steps (Section 9)
- Open decisions are named, not hidden (Section 10)

All six are met. Spec ready for implementation planning.
