#!/bin/sh

# reliably gets the name of the crate corresponding to the current directory,
# even within a workspace or from any subdirectory,
crate_manifest_path=$(cargo locate-project --message-format plain)
crate_name=$(cargo metadata --format-version 1 --no-deps \
  | jq -r --arg manifest_path "$crate_manifest_path" \
  '.packages[] | select(.manifest_path == $manifest_path) | .name')

# Retrieve dependency counts
dependencies=$(cargo metadata --format-version 1 --no-deps \
  | jq -r ".packages[] | select(.name == \"$crate_name\") | .dependencies")

# show the names of the dependencies
required_names=$(echo "$dependencies" | \
	jq -r '[.[] | select(.optional == false and (.kind == null or .kind == "normal"))] | .[].name')
optional_names=$(echo "$dependencies" | \
	jq -r '[.[] | select(.optional == true and (.kind == null or .kind == "normal"))] | .[].name')
echo "Required Dependencies:"
echo "$required_names"
echo "\nOptional Dependencies:"
echo "$optional_names"

# count the dependencies:
required=$(echo "$dependencies" | jq '[.[] | select(.optional == false and (.kind == null or .kind == "normal"))] | length')
optional=$(echo "$dependencies" | jq '[.[] | select(.optional == true and (.kind == null or .kind == "normal"))] | length')
total_non_dev=$((required + optional))
# dev_dependencies=$(echo "$dependencies" | jq '[.[] | select(.kind == "dev")] | length')

echo "\n[dependencies] # $total_non_dev ($required required, $optional optional)"
