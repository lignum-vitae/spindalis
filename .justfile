set shell := ["sh", "-c"]
set windows-shell := ["cmd.exe", "/c"]

ROOT_DIR := justfile_directory()
submodules := "spindalis spindalis_core spindalis_macros"

test_cmd := "cargo test --workspace"
clippy_cmd := "cargo clippy --workspace --all-targets --all-features"
fmt_cmd := "cargo fmt --all"


recipes:
    just -l

test TEST:
    cargo test -p {{TEST}}

[doc('Test all modules')]
test-all:
    {{test_cmd}}

lint LINT:
    cargo clippy --all-targets --all-features -p {{LINT}}

[doc('Lint all modules with clippy')]
lint-all:
    {{clippy_cmd}}

format FORMAT:
    cargo fmt -p {{FORMAT}}

[doc('Format all modules')]
format-all:
    {{fmt_cmd}}

latest:
    git pull origin main

check: latest format-all test-all lint-all
