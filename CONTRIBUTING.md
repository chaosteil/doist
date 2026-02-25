# Contributing to doist

## Contributing

Contributions are accepted and desired, thank you for taking your time to
contriubute! There are lots of ways you can contribute:

### Feature Requests

If you have an idea, just make an issue, and if it makes sense, we'll happily
consider it!

### Pull Requests

You're very welcome to make some PRs yourself. For now the only prerequisite is
that the linter should pass before any serious attempt to review it is done.

## Releasing

The project is set up with goreleaser and expects cargo-release to manage
releases:

1. Figure out what the next version is according to semver
2. Write something up for the release notes (check out `cargo release changes`)
   for ideas.
3. Run `cargo release -x <level>` (probably `patch`)
