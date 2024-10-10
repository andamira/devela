#!/bin/sh

# show all features, with nested contents
# cargo metadata --format-version=1 --no-deps | jq '.packages[] | select(.name == "devela") | .features'

# show all feature names
cargo metadata --format-version=1 --no-deps | jq '.packages[] | select(.name == "devela") | .features | keys'

# count all features
echo -n "total features: "
cargo metadata --format-version=1 --no-deps | jq '.packages[] | select(.name == "devela") | .features | length'

echo -n "hidden features: "
cargo metadata --format-version=1 --no-deps | jq '.packages[] | select(.name == "devela") | .features | keys' \
	| rg '"_' | wc -l
echo -n "visible features: "
cargo metadata --format-version=1 --no-deps | jq '.packages[] | select(.name == "devela") | .features | keys' \
	| rg '"' | rg -v '"_' | wc -l
