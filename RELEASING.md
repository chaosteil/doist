# Releasing

A tag `vX.Y.Z` on main starts the release workflow
(`.github/workflows/release.yml`). The workflow runs the checks, builds
the binaries with GoReleaser and publishes them. You do the one-time
setup once, and the release steps for each version.

## One-time setup

### The Homebrew tap token: `HOMEBREW_TAP_TOKEN`

The repo has this secret. Do these steps again before the token expires.

GoReleaser writes `Casks/doist.rb` to `chaosteil/homebrew-tap`. The
default `GITHUB_TOKEN` of the workflow cannot write to another repo
([GoReleaser](https://goreleaser.com/customization/homebrew_casks/#github-actions)).

1. On GitHub, make a fine-grained personal access token
   ([GitHub docs](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens#creating-a-fine-grained-personal-access-token)).
2. Set "Repository access" to "Only select repositories", and select
   `chaosteil/homebrew-tap`.
3. Set the repository permission "Contents" to "Read and write".
4. Write down the expiry date. Before this date, make a new token and
   set the secret again.
5. Save the token as a secret of doist
   ([gh secret set](https://cli.github.com/manual/gh_secret_set)):

   ```sh
   gh secret set HOMEBREW_TAP_TOKEN --repo chaosteil/doist
   ```

### crates.io Trusted Publishing

The release workflow publishes to crates.io with Trusted Publishing. With
Trusted Publishing, GitHub Actions gets a short-lived token from
crates.io, and the repo keeps no crates.io secret
([crates.io docs](https://crates.io/docs/trusted-publishing)). The crate
`doist` is on crates.io, thus you only add the trusted publisher.

1. Log in to <https://crates.io> with GitHub.
2. Open the "Settings" tab of the crate `doist`. Add a trusted publisher
   with these values:
   - Publisher: GitHub
   - Repository owner: `chaosteil`
   - Repository name: `doist`
   - Workflow filename: `release.yml`
   - Environment: empty

After this setup, the `crates-io` job of the release workflow publishes
each new version. If crates.io already has the version of `Cargo.toml`,
the job does nothing.

## Release steps

The steps use jj. The git commands are below the list.

1. Select the new version `X.Y.Z` ([SemVer](https://semver.org)).
2. Start a new change on main:

   ```sh
   jj git fetch
   jj new main -m "chore(release): vX.Y.Z"
   ```

3. Set `version = "X.Y.Z"` in `Cargo.toml`.
4. Run `cargo build`. Cargo writes the new version to `Cargo.lock`.
5. Run the checks of the CI:

   ```sh
   cargo build
   cargo nextest run
   cargo clippy --all-targets -- -D warnings
   cargo fmt --check
   ```

6. Merge the change to main, with a pull request or a push.
7. Make sure that `main` points to the release commit:

   ```sh
   jj git fetch
   jj log -r main
   ```

8. Tag the release commit, and push the tag:

   ```sh
   jj tag set vX.Y.Z -r main
   jj git push --tag vX.Y.Z
   ```

In a git-only clone, do step 2 with `git switch -c release-vX.Y.Z
origin/main`, commit with `git commit -am "chore(release): vX.Y.Z"`, and
do step 8 with git:

```sh
git fetch origin
git tag vX.Y.Z origin/main
git push origin vX.Y.Z
```

The repo does not use cargo-release, because cargo-release stops on the
detached HEAD of a colocated jj repo.

## What the workflow does

The tag starts three jobs:

- `checks` runs `ci.yml`: the checks on Linux and on macOS.
- `goreleaser` starts after `checks`. It builds the binaries for
  `x86_64-unknown-linux-musl`, `aarch64-unknown-linux-musl`,
  `x86_64-apple-darwin`, `aarch64-apple-darwin` and
  `x86_64-pc-windows-gnu`. Then it makes the GitHub release with the
  archives, the `.deb` and `.rpm` packages, `checksums.txt` and the
  changelog. Then it pushes `Casks/doist.rb` to
  `chaosteil/homebrew-tap`.
- `crates-io` starts after `checks`. It publishes the crate to crates.io,
  if crates.io does not have the version yet.

A tag with a prerelease part, for example `v0.5.0-rc.1`, makes a GitHub
prerelease. This tag does not change the Homebrew tap. The `crates-io`
job publishes a prerelease version too.

## Make sure that the release works

1. Watch the workflow run:

   ```sh
   gh run watch --repo chaosteil/doist
   ```

2. Make sure that the release has the files:

   ```sh
   gh release view vX.Y.Z --repo chaosteil/doist
   ```

3. Make sure that Homebrew installs the new version:

   ```sh
   brew update
   brew upgrade --cask chaosteil/tap/doist
   doist --version
   ```

4. Make sure that <https://crates.io/crates/doist> shows the new
   version.
5. Make sure that cargo-binstall finds the archive:

   ```sh
   cargo binstall --dry-run doist
   ```

If a job fails, read its log. If GoReleaser made a part of the release,
delete the GitHub release before you run the job again:

```sh
gh release delete vX.Y.Z --repo chaosteil/doist
gh run rerun <run-id> --failed --repo chaosteil/doist
```

## Homebrew and the unsigned binary

The macOS binaries are not signed and not notarized. macOS sets a
quarantine flag on a downloaded file, and stops an unsigned binary that
has this flag. The cask removes the flag after the install, with
`hooks.post.install_steps`
([GoReleaser](https://goreleaser.com/customization/homebrew_casks/#signing-and-notarizing)).

Homebrew 5.0.0 deprecated casks without code signing. Homebrew also
said that it disables the casks of `Homebrew/homebrew-cask` that fail
Gatekeeper checks in September 2026
([Homebrew 5.0.0](https://brew.sh/2025/11/12/homebrew-5.0.0/)).
Gatekeeper is the macOS check of signed apps. This rule is for the
official cask repo, and `chaosteil/homebrew-tap` is a third-party tap.

If `brew install --cask chaosteil/tap/doist` stops to install or start
doist on macOS, sign and notarize the binaries
([GoReleaser](https://goreleaser.com/customization/sign/notarize/)).
