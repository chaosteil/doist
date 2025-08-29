# Repository Guidelines

## Project Structure & Module Organization
- `src/`: Rust crate with modules like `api/`, `tasks/`, `projects/`, `sections/`, `labels/`, and CLI wiring in `lib.rs`/`command.rs`. Entry binary: `src/bin/doist.rs`.
- `tests/commands/`: Integration tests using `assert_cmd`, `wiremock`, and `tokio`.
- `vhs/`: Terminal demo assets (for README gifs).
- `.github/`: CI workflows (fmt, clippy, build, test).

## Build, Test, and Development Commands
- Build: `cargo build` (release: `cargo build --release`).
- Run: `cargo run -- <args>` (e.g., `cargo run -- list --nointeractive`).
- Test: `cargo test` (verbose: `cargo test -- --nocapture`).
- Format: `cargo fmt --all` (CI enforces `cargo fmt --check`).
- Lint: `cargo clippy --all-targets -- -D warnings` (CI runs clippy).
- Release (maintainers): `cargo dist` and `cargo release -x <level>` per CONTRIBUTING.

## Coding Style & Naming Conventions
- Edition: Rust 2024; use `rustfmt` defaults (4-space indentation).
- Naming: modules/files `snake_case`; types/enums `PascalCase`; functions/vars `snake_case`; constants `SCREAMING_SNAKE_CASE`.
- CLI: keep concise flags and clap-visible aliases consistent with existing commands.

## Testing Guidelines
- Frameworks: `tokio` for async, `assert_cmd` for CLI, `wiremock` for HTTP, `assert_fs` for temp dirs.
- Layout: put new CLI/integration tests under `tests/commands/` and reuse helpers in `tests/commands/setup.rs` and `tests/commands/mocks.rs`.
- Patterns: prefer `#[tokio::test]` async tests; isolate I/O using `assert_fs::TempDir`; mock HTTP via `wiremock`.
- Run subsets: `cargo test list` (by name filter).

## Commit & Pull Request Guidelines
- Commits: imperative, concise subjects (e.g., "Add list expand flag"); reference issues/PRs when relevant.
- PRs: include what/why, CLI examples or screenshots for UX changes, and link related issues.
- Quality gate: ensure `cargo fmt`, `cargo clippy`, `cargo build`, and `cargo test` pass locally before pushing.

## Security & Configuration Tips
- Auth: `doist auth <TOKEN>` stores the Todoist token in the config dir (XDG). Never commit tokens.
- Config override for testing: pass `--config_prefix <dir>` (mirrors test harness behavior) to keep local testing isolated.

## Branching & Upstream Sync
- Default branch: work on `rob/patches`; keep `main` mirroring `upstream/main`.
- Set default (GitHub): `gh repo edit --default-branch rob/patches`.
- Add upstream once: `git remote add upstream https://github.com/chaosteil/doist.git`.
- Sync upstream: `git fetch upstream && git checkout main && git reset --hard upstream/main`.
- Update work branch: `git checkout rob/patches && git merge --no-ff main` (or `git rebase main` then `git push --force-with-lease`).
- New work: branch off `rob/patches` and PR back into `rob/patches`.

## Architecture Overview
- Entry points: binary `src/bin/doist.rs` calls into `lib.rs`, which parses CLI via Clap in `command.rs` (`Arguments`, `Commands`, `AuthCommands`).
- Config: `config::Config` loads/saves token and base URL (XDG dir via `dirs`/`xdg`). Tests use `--config_prefix` to sandbox configs.
- HTTP layer: `api` module builds a `reqwest` client with retry middleware (`reqwest-middleware`, `reqwest-retry`). DTOs serialize via `serde`/`serde_json`.
- Domains: `tasks/`, `projects/`, `sections/`, `labels/` each expose subcommand params and ops (`add`, `list`, `view`, etc.). Command routing is in `command.rs`.
- UX: interactive flows use `dialoguer` (fuzzy select) and `indicatif`; time handling via `chrono`/`chrono-tz`.
- Errors: `color-eyre::Result` and `thiserror` for typed errors.

## Feature Development Workflow
- Add a subcommand: define `Params` + handler in the relevant domain (e.g., `tasks/<op>.rs`), wire it in `AuthCommands` dispatch.
- Extend API: add request/response types under `api/`, use `serde` for (de)serialization, and reuse the shared client.
- Tests: add integration tests under `tests/commands/`, mocking HTTP with `wiremock` and spawning the binary via `assert_cmd` (`Tool::init`).
