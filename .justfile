set shell := ["sh", "-c"]
set windows-shell := ["cmd.exe", "/c"]

ROOT_DIR := justfile_directory()
submodules := "spindalis spindalis_core spindalis_macros"


recipes:
    just -l

test TEST:
    cargo test -p {{TEST}}

[doc('Test all modules')]
test-all:
    cargo test --workspace

lint LINT:
    cargo clippy --all-targets --all-features -p {{LINT}}

[doc('Lint all modules with clippy')]
lint-all:
    cargo clippy --workspace --all-targets --all-features

format FORMAT:
    cargo fmt -p {{FORMAT}}

[doc('Format all modules')]
format-all:
    cargo fmt --all

latest:
    git pull origin main

check: latest format-all test-all lint-all
