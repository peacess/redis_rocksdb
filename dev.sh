#!/usr/bin/env bash

_DIR=$(dirname $(realpath "$0"))

cd $_DIR

#. ./sh/pid.sh

set -ex

if ! hash watchexec 2>/dev/null; then
cargo install watchexec-cli
fi


cargo build || true

RUST_BACKTRACE=1 watchexec \
  -w src/ \
  -w examples/ \
  -c -r --exts rs,toml \
  -- "cargo +nightly test"
