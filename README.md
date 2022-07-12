# todoist CLI Client

An unofficial [todoist](https://todoist.com/) API CLI client written in Rust.

## About

This is an unofficial todoist CLI that focuses on being easy to use. It is
currently not feature complete, but covers some basic common use-cases and adds
more as we go along.

## Installation

TODO

## How to use

### Auth

First, set up your API token. Go into your todoist settings, go to
`Integrations` and copy out the `API token`. Plug it into the tool:

```bash
todoist auth MY_TOKEN
```

Now you're authenticated and can use the other functions of the tool.

### List tasks

Listing tasks and then working with them interactively is the recommended way to
work with the CLI.

By default the list view shows todays tasks:

```bash
todoist list
# Altenatively: `todoist l`
```

You can use interactive mode to interactively work with your tasks:

```bash
todoist list --interactive
# Alternatively: `todoist l -i`
```

This will allow you to type parts of the output until you select the task you
want to work with. Selecting will allow you to select various other subcommands,
like closing, changing due dates or even editing tasks.

### Adding tasks

A quick way to add a task is:

```bash
todoist add "Do the laundry" --due "tomorrow"
# Alternatively: `todoist a "Do the laundry" -d tomorrow`
```

### Help

Feel free to browse the help output for more help:

```bash
todoist help
```
