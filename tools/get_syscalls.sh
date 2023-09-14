#!/usr/bin/env bash
# devela/tools/get_syscalls.sh
#
# Generates a rust file with consts for the syscalls of the current target.
# It is called by the `.github/workflows/get_syscalls.yml` manual CI action.

TARGET=$1
if [ -z "$TARGET" ]; then
	TARGET=localhost
fi

# make sure to install the required packages:
# apt-get update && apt-get install -y gcc

echo -e '#include <sys/syscall.h>' | cpp -dM \
	| grep '#define __NR_.*[0-9])\?$' \
	| sed 's/(__NR_SYSCALL_BASE + //' | sed 's/)//' \
	| cut -d' ' -f 2,3 | cut -d_ -f 4- \
	| sort -k 1,1 \
	| awk '{print "pub const " toupper($1) ": isize = " $2 ";"}' \
	> "syscalls_${TARGET}.rs"
