set shell := ["sh", "-c"]
set windows-shell := ["cmd.exe", "/c"]

ROOT_DIR := justfile_directory()
submodules := "spindalis spindalis_core spindalis_macros"

test_cmd := if os_family() == "windows" { \
    f"cmd.exe /c FOR %f IN ({{submodules}}) DO " + \
    f"cargo test --manifest-path {{ROOT_DIR}}/%f/Cargo.toml" \
} else { \
    f"for mod in {{submodules}}; do " + \
    f"cargo test --manifest-path {{ROOT_DIR}}/$mod/Cargo.toml; done" \
}

clippy_cmd := if os_family() == "windows" { \
    f"cmd.exe /c FOR %f IN ({{submodules}}) DO " + \
    "cargo clippy --all-targets --all-features " + \
    f"--manifest-path {{ROOT_DIR}}/%f/Cargo.toml" \
} else { \
    f"for mod in {{submodules}}; do " + \
    "cargo clippy --all-targets --all-features " + \
    f"--manifest-path {{ROOT_DIR}}/$mod/Cargo.toml; done" \
}

fmt_cmd := if os_family() == "windows" { \
    f"cmd.exe /c FOR %f IN ({{submodules}}) DO " + \
    f"cargo fmt --manifest-path {{ROOT_DIR}}/%f/Cargo.toml" \
} else { \
    f"for mod in {{submodules}}; do " + \
    f"cargo fmt --manifest-path {{ROOT_DIR}}/$mod/Cargo.toml; done" \
}


recipes:
    just -l

test TEST:
    cargo test --mainfest-path {{ROOT_DIR}}/{{TEST}}/Cargo.toml

[doc('Test all modules')]
test-all:
    {{test_cmd}}

lint LINT:
    cargo clippy --all-targets --all-features --mainfest-path {{ROOT_DIR}}/{{LINT}}/Cargo.toml

[doc('Lint all modules with clippy')]
lint-all:
    {{clippy_cmd}}

format FORMAT:
    cargo fmt --mainfest-path {{ROOT_DIR}}/{{FORMAT}}/Cargo.toml

[doc('Format all modules')]
format-all:
    {{fmt_cmd}}

latest:
    git pull origin main

check: latest format-all test-all lint-all
