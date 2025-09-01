#!/bin/sh

RUSTDOCFLAGS='-Z unstable-options '--show-coverage cargo +nightly nd -F _docs
