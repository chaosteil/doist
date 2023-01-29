#!/bin/sh
set -e

dir=$(mktemp -d)
cat >> "$dir/config.toml" << EOF
token = "AUTH_KEY"
url = "http://localhost:3000"
override_time = "$(cat ../tests/commands/fixtures/fetch_time)"
EOF
cargo run --quiet -- --config_prefix="$dir" "$@"
