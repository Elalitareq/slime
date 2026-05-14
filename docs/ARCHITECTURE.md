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
