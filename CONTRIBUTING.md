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
- `cargo fmt`, `cargo clippy --workspace --all-targets -- -D warnings`, and
  `cargo test --workspace` must pass before requesting review.

## Crate ownership

Each crate has a `README.md` describing its role. If you're not sure where a
change belongs, open an issue first and we'll route it.

## Discussion

GitHub Issues for bugs and features. Discord for everything else (link in the
project README once the server is set up).

## Code of Conduct

By participating you agree to follow our [Code of Conduct](CODE_OF_CONDUCT.md).
