# Contributing to Spindalis

Thank you for your interest in contributing to Spindalis! We welcome contributions,
whether you're fixing a bug, improving documentation, or adding new features.
This guide will help you get started with contributing to the project.

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [How to Contribute](#how-to-contribute)
- [Bug Reports](#bug-reports)
- [Feature Requests](#feature-requests)
- [Pull Requests](#pull-requests)
- [Code Style](#code-style)
- [License](#license)
- [Community](#community)

## Code of Conduct

By participating in this project, you agree to abide by our
[Code of Conduct](https://github.com/lignum-vitae/spindalis/blob/main/docs/CODE_OF_CONDUCT.md).
Please take a moment to familiarize yourself with it.

## How to Contribute

### Bug Reports

If you find a bug or unexpected behavior, please open an issue in the GitHub
repository. When reporting a bug, provide the following information:

- A description of the problem.
- Steps to reproduce the issue (if applicable).
- Any relevant error messages.
- The version of the library you're using.

### Feature Requests

If you have an idea for a new feature or enhancement, please open an issue
describing the feature and why you think it would be useful.
We encourage open discussions before starting to code a new feature.

### Pull Requests

To contribute code:

>[!Note]
>Detailed below is the process of adding the repo as an upstream repo through your
>Command Line Interface (CLI).
>However, GitHub allows you to sync your fork through their Web UI by navigating
>to the GitHub Page of your repo fork and clicking on the `Sync fork` button.
>GitHub also has its own CLI that allows you to use the command
>`gh repo sync owner/cli-fork -b BRANCH-NAME`.
>You can read more about that in the
>[GitHub Docs](https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/working-with-forks/syncing-a-fork)
>here.
>If you go this route, you should be able to skip steps 4 and 9, as well as omit
>`upstream/main` from step 5 below. Push your changes directly to your GitHub fork.
>The GitHub desktop app also provides a UI that makes creating, deleting, and editing
>branches on your fork easy.

#### 1. Open a new Issue following the above-mentioned guidelines

#### 2. Fork the repository to your own GitHub account

#### 3. Clone your fork locally:

```nginx
git clone https://github.com/YOUR_USERNAME/spindalis.git
```

#### 4. Keep your fork up to date with the main branch

```nginx
# Add the main Spindalis repo as upstream
git remote add upstream https://github.com/lignum-vitae/spindalis.git
# Get latest changes
git fetch upstream
# Verify your remotes
git remote
```

#### 5. Create a new branch for your changes:

##### Choose ONE of the following commands

```nginx
# Creates a new branch that stays in sync with the main repository
git checkout -b feature-name upstream/main

# Checks out existing branch if you already have a branch locally
git checkout feature-name
```

#### 6. Make your changes in your local repository

```nginx
# Make sure latest changes are taken from main BEFORE making changes
git pull origin main
```

#### 7. Run your changes in your local environment

This step can be achieved using [just](https://github.com/casey/just).
`Just` can be installed using `cargo install just`,
by using any of their listed package managers, or by `curl`.

Ensure that `just` is working correctly by running `just` in the shell from the
project directory. Windows users may need to check that `just` is in their `PATH`.

After installing `just`, all appropriate checks can be run using the command
`just check`.

```nginx
# Format your changes
cargo fmt

# Clear cache to avoid issuessuch as "failed to resolve" errors
cargo clean
cargo check

# Lint your work with Clippy
cargo clippy --all-targets --all-features

# Run your tests locally
cargo test
```

#### 8. Commit your changes with a descriptive commit message

```nginx
# Gets latest changes from main Spindalis project if you've set up an upstream branch as detailed above
git fetch upstream
# We recommend individually adding each file with modifications
git add <filename>
# Commit files after all files with modifications have been added
git commit -m "Add feature: description of change"
```

#### 🚨 Using "git add ." when staging changes

While `git add .` is convenient for adding all modified files, it can lead to
messy commits. Consider using it only when:

- You've reviewed all changes
- You're certain about each modification
- You've checked git status first
- Your .gitignore is properly configured

#### 9. Push your branch to your fork on GitHub:

```nginx
git push -f origin feature-name
```

#### 10. Open a Pull Request (PR) from your branch to the main branch of the original Spindalis repository on GitHub

- You may need to click the `compare across forks` link under the `Compare changes`
header that populates when you click `New pull request` to see your local repo fork.

#### 11. In your PR description, include:

- A summary of the changes.
- Any relevant issue numbers (e.g., fixes #123).
- Information about tests and validation.

## Code Style

We follow [that Rust style guide](https://doc.rust-lang.org/nightly/style-guide/)
for code formatting.
Please run `cargo fmt` before committing.

Some additional notes:

- Use meaningful variable and function names.
- Keep lines readable (Cargo defaults to 100 characters).
- Make sure to update documentation if your changes affect the usage or API.
- Rust has two forms of tests; integration tests, and unit tests. Add unit tests
in the same file as your changes for any new functionality that is self contained.
Add integration tests in the `tests/` folder for any changes that require functionality
from multiple areas within the Spindalis crate.

You can format the codebase by running this from the root:

```bash
cargo fmt
```

## License

By contributing to Spindalis, you agree that your contributions will be licensed
under the MIT License, as outlined in the [LICENSE](https://github.com/lignum-vitae/spindalis/blob/main/LICENSE)
file.

## Community

We encourage contributions from everyone, and we strive to maintain a welcoming
and inclusive community. If you have any questions, need help, or want to discuss
ideas, feel free to reach out via issues or the repository discussions.

Thank you for contributing to Spindalis! Your help improves the project for everyone!
