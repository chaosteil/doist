# doist – AI Assistant Usage Guide

Concise instructions for LLMs to install, authenticate, and use the doist CLI safely and predictably.

## Repo & Defaults
- Fork: https://github.com/robbarry/doist (default branch: `rob/patches`).
- Language: Rust (Edition 2024).
- Default action: `list` tasks (non-interactive).
- Default filter: shows today | overdue (not all tasks).

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

## Core Commands
The default command is `list`. All commands below require authentication.

### Task Commands
```bash
# List tasks (default, non-interactive)
doist list -n
doist # same as above

# List all tasks
doist list -n -f all

# Add a task
doist add "Review PR" -P Work -L code -p 2 -d today

# Create a task interactively
doist create

# View, edit, close, or comment on a task by ID
doist view <task_id>
doist edit <task_id>
doist close <task_id>
doist comment <task_id> "This is a comment"
```

### Project Commands (`doist projects ...`)
Alias: `p`
```bash
# List projects
doist projects list

# Add, view, delete a project
doist projects add "New Project"
doist projects view <project_id>
doist projects delete <project_id>
```

### Label Commands (`doist labels ...`)
Alias: `lbl`
```bash
# List labels
doist labels list

# Add, delete a label
doist labels add "new-label"
doist labels delete <label_id>
```

### Section Commands (`doist projects sections ...`)
Alias: `s` (under `projects`)
```bash
# List sections in a project
doist projects sections --project-id <project_id> list

# Add, delete a section
doist projects sections --project-id <project_id> add "New Section"
doist projects sections --project-id <project_id> delete <section_id>
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

## Development
- Run: `cargo run -- list -n`.
- Test: `cargo test`.
- Format: `cargo fmt --all`.
- Lint: `cargo clippy --all-targets -- -D warnings`.
- Config sandbox for tests: add `--config_prefix <dir>` to isolate state.

