# doist - Todoist CLI Deployment Guide for AI Assistants

This document contains all information needed for AI assistants to deploy and use the doist CLI tool.

## What is doist?

doist is an unofficial Todoist CLI client written in Rust that provides command-line access to Todoist task management. The latest version (0.3.4+) defaults to non-interactive mode, making it perfect for scripting and automation.

## Repository Information

- **Fork**: https://github.com/robbarry/doist (fork of chaosteil/doist)
- **Default branch**: `rob/patches` (not `main`)
- **Language**: Rust (edition 2024)
- **Key feature**: Defaults to non-interactive mode (no `-n` flag needed)

## Installation Instructions

### Local Installation (macOS/Linux)

```bash
# Clone repository
git clone https://github.com/robbarry/doist.git
cd doist
git checkout rob/patches

# Build and install
cargo build --release
cargo install --path . --force

# Verify
doist --version
```

### Remote Server Installation

For Ubuntu/Debian servers:

```bash
SERVER="your-server"

# Install Rust
ssh $SERVER "curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y"

# Install dependencies
ssh $SERVER "sudo apt-get update && sudo apt-get install -y pkg-config libssl-dev git"

# Clone and build
ssh $SERVER "git clone https://github.com/robbarry/doist.git ~/doist-cli"
ssh $SERVER "cd ~/doist-cli && git checkout rob/patches && source ~/.cargo/env && cargo build --release"

# Install to ~/.local/bin
ssh $SERVER "mkdir -p ~/.local/bin && cp ~/doist-cli/target/release/doist ~/.local/bin/"
ssh $SERVER 'grep -q ".local/bin" ~/.bashrc || echo "export PATH=\"\$HOME/.local/bin:\$PATH\"" >> ~/.bashrc'

# Verify
ssh $SERVER "~/.local/bin/doist --version"
```

## Authentication

Before using doist, authenticate with your Todoist API token:

```bash
# Get token from: https://todoist.com/app/settings/integrations
doist auth YOUR_API_TOKEN
```

## Critical Default Behavior

**IMPORTANT**: By default, `doist` or `doist list` shows **ONLY today's and overdue tasks**, not all tasks!

- To see ALL tasks: `doist list -f all`
- To see inbox: `doist list -f "#inbox"`
- To see next 7 days: `doist list -f "7 days"`

## Essential Commands

### Task Management
```bash
# List today's and overdue tasks (DEFAULT)
doist

# List ALL tasks
doist list -f all

# Add task to inbox (default project)
doist add "Task name"

# Add task with project, label, priority, and due date
doist add "Review PR" -P Work -L work -p 2 -d "today"

# View task details (get ID from list output)
doist view <task_id>

# Close/complete a task
doist close <task_id>

# Edit a task
doist edit <task_id>
```

### Project Management
```bash
doist projects list
doist projects add
doist projects view <project_id>
```

### Label Management
```bash
doist labels list
doist labels add
```

## Task Options

- `-d, --due <DUE>` - Due date (natural language: "today", "tomorrow", "next Monday")
- `-D, --desc <DESC>` - Detailed description
- `-p, --priority <PRIORITY>` - Priority (1=urgent, 2=high, 3=normal, 4=low)
- `-P, --project <project>` - Project (supports fuzzy matching)
- `-L, --label <label>` - Label (can use multiple times, fuzzy matching)
- `-S, --section <section>` - Section (fuzzy matching)

## Filter Examples

The `-f, --filter` option supports Todoist's natural language filters:

- `"all"` - ALL tasks (not the default!)
- `"today | overdue"` - Today's or overdue tasks (DEFAULT)
- `"#inbox"` - Inbox tasks only
- `"7 days"` - Tasks due in next 7 days
- `"no date"` - Tasks without due dates
- `"@work & @ready"` - Tasks with both labels
- `"p1 | p2"` - High priority tasks
- `"#Work"` - All tasks in Work project
- `"tomorrow"` - Tasks due tomorrow

## Interactive Mode

For complex operations, use interactive mode:
```bash
doist -i
```

This allows fuzzy search selection and multiple operations.

## Common Workflows

```bash
# Morning review
doist  # See today's tasks

# Weekly planning
doist list -f "7 days"

# Quick capture to inbox
doist add "Call dentist"

# Batch add tasks
doist add "Task 1" -P Work && doist add "Task 2" -P Work && doist

# Project focus
doist list -P Work

# Find tasks without dates
doist list -f "no date"
```

## Troubleshooting

### Build Issues
- Missing pkg-config: `apt install pkg-config` or `yum install pkgconfig`
- OpenSSL headers missing: `apt install libssl-dev` or `yum install openssl-devel`
- Rust too old: Update with `rustup update stable`

### Runtime Issues
- No tasks shown: Remember default is today/overdue only, use `-f all` for everything
- Authentication failed: Re-run `doist auth YOUR_TOKEN`
- Command not found: Ensure `~/.local/bin` or `~/.cargo/bin` is in PATH

## Development Notes

### Repository Structure
- `src/` - Rust source code
- `tests/commands/` - Integration tests
- `docs/REMOTE_INSTALL.md` - Detailed remote installation guide
- `CLAUDE.md` - Project-specific documentation

### Building from Source
```bash
cargo build           # Debug build
cargo build --release # Release build
cargo test           # Run tests
cargo fmt --all      # Format code
cargo clippy --all-targets -- -D warnings  # Lint
```

### Key Changes in robbarry/doist Fork
1. Default to non-interactive mode (no `-n` flag needed)
2. Flag conflicts: `-i` and `-n` are mutually exclusive
3. Updated documentation and examples

## Servers with doist Installed
- nonprofits
- alvin

Access via: `ssh <server>` then run `doist`

## Important Tips

1. **Always remember**: Default shows only today/overdue, not all tasks
2. **Use fuzzy matching**: Projects and labels support partial matches
3. **Natural language dates**: "tomorrow", "next Monday", "in 3 days" all work
4. **Quick capture**: `doist add "task"` goes to inbox by default
5. **Chain commands**: Use `&&` to run multiple commands in sequence
6. **Find task IDs**: IDs are shown in list output, needed for view/edit/close

## API Reference

Full Todoist API filters documentation: https://todoist.com/help/articles/introduction-to-filters

## Version Info

Current version: 0.3.4
Rust edition: 2024
Default behavior: Non-interactive mode