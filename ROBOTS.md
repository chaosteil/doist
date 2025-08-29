# doist – AI Assistant Usage Guide

Concise instructions for LLMs to install, authenticate, and use the doist CLI safely and predictably.

## Repo & Defaults
- Fork: https://github.com/robbarry/doist (default branch: `rob/patches`).
- Language: Rust (Edition 2024).
- Default filter: shows today | overdue (not all tasks).
- Current default mode: interactive. Use `-n/--nointeractive` for non‑interactive output. Note: a PR proposes flipping this default; rely on flags to be explicit.

## Install (local)
```bash
git clone https://github.com/robbarry/doist.git
cd doist && git checkout rob/patches
cargo build --release
cargo install --path . --force
doist --version
```

## Authenticate
```bash
# Get token at https://todoist.com/app/settings/integrations
doist auth <API_TOKEN>
```

## Core Commands (safe defaults)
```bash
# List (non‑interactive, suitable for piping)
doist list -n

# List all tasks (not default)
doist list -n -f all

# Interactive (fuzzy select, actions menu)
doist list -i

# Add tasks
doist add "Review PR" -P Work -L code -p 2 -d today

# View / close / edit by ID (from list output)
doist view <task_id>
doist close <task_id>
doist edit <task_id>
```

## Useful Flags
- `-f, --filter <query>`: Todoist filters (e.g., `all`, `"7 days"`, `"#inbox"`).
- `-P, --project <name>` / `--project_id <id>`: project (fuzzy by name).
- `-S, --section <name>` / `--section_id <id>`: section (fuzzy by name).
- `-L, --label <name>`: label (repeatable; fuzzy by name).
- `-n, --nointeractive`: print results; no prompts.
- `-i, --interactive`: continuous interactive loop.

## Behavior Notes
- Output defaults to “today | overdue”; pass `-f all` for everything.
- Non‑interactive prints a stable table (good for piping/searching).
- Interactive flows rely on a TTY; avoid in headless scripts.

## Troubleshooting (quick)
- No tasks shown: likely filter. Try `-f all`.
- Auth errors: re‑run `doist auth <API_TOKEN>`.
- PATH issues: ensure `~/.cargo/bin` (or install path) is on `PATH`.

## Development (brief)
- Run: `cargo run -- list -n`.
- Test: `cargo test`; Lint/format: `cargo clippy` / `cargo fmt --all`.
- Config sandbox for tests: add `--config_prefix <dir>` to isolate state.

