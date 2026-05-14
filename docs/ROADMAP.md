# Slime Roadmap

Mirrors the spec's milestone table. Update this when milestones land.

## Phase A — Static Web MVP (target: 4–8 months)

| #   | Status    | Milestone                            |
| --- | --------- | ------------------------------------ |
| A0  | ✅ done   | Workspace + CI green                 |
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
