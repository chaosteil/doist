# doist - Todoist CLI Client

[![Crates.io](https://img.shields.io/crates/v/doist)](https://crates.io/crates/doist)
[![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/chaosteil/doist/ci.yml?branch=main)](https://github.com/chaosteil/doist/actions)

An unofficial [Todoist](https://todoist.com/) API CLI client written in Rust.

## About

<p align="center">
  <img width="1200" src="https://raw.githubusercontent.com/chaosteil/doist/main/vhs/doist.gif">
</p>

This is an unofficial Todoist CLI that focuses on being easy to use. It is
currently not feature complete, but covers some basic common use-cases and adds
more as we go along.

## Installation

### Install from crates.io

Currently it's installed only via cargo install:

```bash
# Install Rust first: https://www.rust-lang.org/tools/install
cargo install doist
# Binary now available in $HOME/.cargo/bin/doist
```

### Build from source

```bash
# Install Rust first: https://www.rust-lang.org/tools/install
git clone git@github.com:chaosteil/doist
cd doist
cargo build --release
./target/release/doist
```

More options coming eventually.

## How to use

### Auth

First, set up your API token. Go into your [Todoist settings](https://todoist.com/app/settings/integrations),
go to `Integrations` and copy out the `API token`. Plug it into the tool:

```bash
doist auth MY_TOKEN
```

Now you're authenticated and can use the other functions of the tool.

### List tasks

Listing tasks and then working with them interactively is the recommended way to
work with the CLI.

By default the list view shows todays tasks and lets you work with them:

```bash
doist
# Alternatively: `doist list` or `doist l`.
```

This will allow you to type parts of the output until you select the task you
want to work with (fuzzy search). Selecting will allow you to select various
other subcommands, like closing, changing due dates or even editing tasks.

You can also disable interactive mode to pipe use the output somewhere else:

```bash
doist list --nointeractive
# Alternatively: `doist l -n`
```

By default all interactive commands have a filter applied to show the most
relevant tasks. See the
[documentation](https://todoist.com/help/articles/introduction-to-filters) to
see what inputs are accepted. To then use the filter, add it to the command
parameters:

```bash
doist list --filter "all"
# Alternatively: `doist l -f all`
```

### Interactive (continuous) mode

To continue to use `doist list` with the currently applied filters after each
action (so you can close multiple tasks one after the other for example), a
super-interactive (continuous) mode is also available. This makes the experience
closer to the official app.

```bash
doist list --interactive
# Alternatively: `doist -i`
```

To close out of this mode, press `ESC` during the main list selection.

### Adding tasks

A quick way to add a task is:

```bash
doist add "Do the laundry" --desc "I always forget" --due "tomorrow"
# Alternatively: `doist a "Do the laundry" -D "I always forget" -d tomorrow`
```

Only the task name is required, everything else will assume a default of
*nothing*.

### Interactive task creation

Another way to fully interactively create tasks is:

```bash
doist create
# Alternatively: `doist A`
```

Which will prompt you for the task name and then give you an interactive menu
where you can fill in the details as necessary.

### More about tasks

It's also possible to provide the task with a priority:

```bash
doist add "Party hard" --priority 1
# Alternatively: `doist a "Party hard" -p1`
```

There are several other things you can do to add richer information to a task.
All inputs can be partially provided and will fuzzy match to the closest name
you probably had in mind:

```bash
# Adding project information
doist add "Party hard" --project "personal"
# Alternatively: `doist a "Party hard" -P personal`
```

```bash
# Adding section information. Will automatically attach to the correct project,
# but setting the project will narrow it down.
doist add "Party hard" --section "weekend"
# Alternatively: `doist a "Party hard" -S weekend`
doist add "Party hard" --project personal --section weekend
# Alternatively: `doist a "Party hard" -P personal -S weekend`
```

```bash
# Multiple labels can be provided when creating tasks as well
doist add "Party hard" --label dance --label happy
# Alternatively: `doist a "Party hard" -L dance -L happy`
```

Instead of providing names to be matched, you can also directly provide their
API IDs if you use this tool for automated tooling.

### Closing tasks

A quick way to close one of todays tasks is:

```bash
doist close
# Alternatively: `doist c`
```

And then fuzzy finding the task you want to close. Submitting the ID directly
also works if you're more comfortable with that:

```bash
doist close "BIG_ID_FROM_API"
# Alternatively: `doist c BIG_ID_FROM_API`
```

### View details

To view details of tasks and the comments asssociated with a task:

```bash
doist view
# Alternatively: `doist v`
```

This accepts the same parameters as `doist list` for task selection.

#### Configuration setup

### Disable colors

If you're not a fan of emojis or colors, you can disable all doist-induced
colors by setting the environment variable `NO_COLOR`:

```bash
NO_COLOR=1 doist
```

### Custom default filter

If you don't like the default filter of `(today | upcoming)`, you can set a
different default filter in the `~/.config/doist/config.toml` like this:

```toml
default_filter="all"
``````

See the [Todoist article on filtering](https://todoist.com/help/articles/introduction-to-filters)
for more information.

### Help

Feel free to browse the help output for more help:

```bash
doist help
```
