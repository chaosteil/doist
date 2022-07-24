# doist - Todoist CLI Client

[![Crates.io](https://img.shields.io/crates/v/doist)](https://crates.io/crates/doist)
[![GitHub Workflow Status](https://img.shields.io/github/workflow/status/chaosteil/doist/CI/main)](https://github.com/chaosteil/doist/actions)

An unofficial [Todoist](https://todoist.com/) API CLI client written in Rust.

## About

<p align="center">
  <img width="600" src="https://raw.githubusercontent.com/chaosteil/doist/main/doist.svg">
</p>

This is an unofficial Todoist CLI that focuses on being easy to use. It is
currently not feature complete, but covers some basic common use-cases and adds
more as we go along.

## Installation

Currently it's installed only via cargo install from source:

```bash
cargo install doist
```

More options coming eventually.

## How to use

### Auth

First, set up your API token. Go into your Todoist settings, go to
`Integrations` and copy out the `API token`. Plug it into the tool:

```bash
doist auth MY_TOKEN
```

Now you're authenticated and can use the other functions of the tool.

### List tasks

Listing tasks and then working with them interactively is the recommended way to
work with the CLI.

By default the list view shows todays tasks and lets you work with them:

```bash
doist list
# Alternatively: `doist l`
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

### Adding tasks

A quick way to add a task is:

```bash
doist add "Do the laundry" --due "tomorrow"
# Alternatively: `doist a "Do the laundry" -d tomorrow`
```

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

### Help

Feel free to browse the help output for more help:

```bash
doist help
```
