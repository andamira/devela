#!/bin/sh

# reliably gets the name of the crate corresponding to the current directory,
# even within a workspace or from any subdirectory,
crate_manifest_path=$(cargo locate-project --message-format plain)
crate_name=$(cargo metadata --format-version 1 --no-deps \
  | jq -r --arg manifest_path "$crate_manifest_path" \
  '.packages[] | select(.manifest_path == $manifest_path) | .name')

# show the names of all the features
cargo metadata --format-version=1 --no-deps | jq ".packages[] | select(.name == \"$crate_name\") | .features | keys"

# show the feature count (for the manifest)
total=$(cargo metadata --format-version=1 --no-deps | jq ".packages[] | select(.name == \"$crate_name\") | .features | length")
visible=$(cargo metadata --format-version=1 --no-deps | jq ".packages[] | select(.name == \"$crate_name\") | .features | keys" \
	| rg '"' | rg -v '"_' | wc -l)
hidden=$(cargo metadata --format-version=1 --no-deps | jq ".packages[] | select(.name == \"$crate_name\") | .features | keys" \
	| rg '"_' | wc -l)

remaining=$((300 - total))

echo "[features] # $total/300 ($remaining remaining), $visible visible, $hidden hidden"
