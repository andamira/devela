#!/bin/sh

# Resolve the directory of the script
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

# Reliably get the name of the crate corresponding to the current directory,
# even within a workspace or from any subdirectory.
crate_manifest_path=$(cargo locate-project --message-format plain)
crate_name=$(cargo metadata --format-version 1 --no-deps \
  | jq -r --arg manifest_path "$crate_manifest_path" \
  '.packages[] | select(.manifest_path == $manifest_path) | .name')

# Operation selector
operation="$1"

case "$operation" in
  # Output summary of dependencies and features
  summary)
    "$SCRIPT_DIR/manifest.sh" dependencies | rg '\[dependencies\]'
    "$SCRIPT_DIR/manifest.sh" features | rg '\[features\]'
    ;;

  # Retrieve and display dependencies
  dependencies)

    dependencies=$(cargo metadata --format-version 1 --no-deps \
      | jq -r ".packages[] | select(.name == \"$crate_name\") | .dependencies")

    # Extract required and optional dependency names
    required_names=$(echo "$dependencies" | \
      jq -r '[.[] | select(.optional == false and (.kind == null or .kind == "normal"))] | .[].name')
    optional_names=$(echo "$dependencies" | \
      jq -r '[.[] | select(.optional == true and (.kind == null or .kind == "normal"))] | .[].name')

    # Output dependency names
    echo "Required Dependencies:"
    echo "$required_names"
    echo "\nOptional Dependencies:"
    echo "$optional_names"

    # Count dependencies
    required=$(echo "$dependencies" | jq '[.[] | select(.optional == false and (.kind == null or .kind == "normal"))] | length')
    optional=$(echo "$dependencies" | jq '[.[] | select(.optional == true and (.kind == null or .kind == "normal"))] | length')
    total_non_dev=$((required + optional))

    # Output dependency counts
    echo "\n[dependencies] # $total_non_dev ($required required, $optional optional)"
    ;;

  # Retrieve and display features
  features)
    features=$(cargo metadata --format-version=1 --no-deps | jq ".packages[] | select(.name == \"$crate_name\") | .features | keys")

    # Output feature list
    echo "Features:"
    echo "$features"

    # Count features
    total=$(echo "$features" | jq length)
    visible=$(echo "$features" | rg '"' | rg -v '"_' | wc -l)
    hidden=$(echo "$features" | rg '"_' | wc -l)
    remaining=$((300 - total))

    # Output feature counts
    echo "\n[features] # $total/300 ($remaining remaining), $visible visible, $hidden hidden"
    ;;

  *)
    echo "Usage: $0 {summary|dependencies|features}"
    exit 1
    ;;
esac
