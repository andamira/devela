#!/bin/sh

RUSTDOCFLAGS='-Z unstable-options '--show-coverage cargo +nightly d -F _docs
