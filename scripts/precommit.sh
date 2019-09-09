#!/bin/sh

set -e

(cd c-api && cargo fmt --all)
(cd fuzz && cargo fmt --all)
cargo fmt --all && git diff --name-only --cached | xargs git add