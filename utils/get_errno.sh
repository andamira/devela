#!/usr/bin/env bash
# devela/utils/get_errno.sh
#
# Generates a rust file with consts for from errno.h on the current target.
# It is called by the `.github/workflows/get_errno.yml` manual CI action.

# the errno.h files should be the same in most linux platforms,
# you can quickly check if the files are equal with the following command:
#
# $ diff -q --from-file errno_*

TARGET=$1
if [ -z "$TARGET" ]; then
	TARGET=localhost
fi

# make sure to install the required packages:
# apt-get update && apt-get install -y gcc

echo -e '#include <sys/errno.h>' | cpp -dM \
	| grep "#define [A-Z].*[0-9]$" \
	| cut -d' ' -f 2,3 \
	| sort -k 1.1 \
	| awk '{print "pub const " toupper($1) ": isize = " $2 ";"}' \
	> "errno_${TARGET}.rs"

# sort by number instead:
# | sort -g -k 2.1 \
