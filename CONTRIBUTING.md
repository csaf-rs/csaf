# Contributing to csaf-rs

Thank you for your interest in contributing! This document explains how to participate effectively in this project.

## Table of Contents

- [Getting Started](#getting-started)
- [Reporting Issues](#reporting-issues)
- [Suggesting Features](#suggesting-features)
- [Contributing Code](#contributing-code)
- [Pull Request Process](#pull-request-process)
- [Development Setup](#development-setup)
- [Coding Standards](#coding-standards)
- [Commit Message Guidelines](#commit-message-guidelines)

---

## Getting Started

1. **Fork** the repository on GitHub.
2. **Clone** your fork locally:
   ```bash
   git clone https://github.com/<your-username>/csaf.git
   cd csaf
   git submodule update --init --recursive
   ```
3. Set up the upstream remote:
   ```bash
   git remote add upstream https://github.com/csaf-rs/csaf.git
   ```
4. Follow the [Development Setup](#development-setup) instructions below.

---

## Reporting Issues

Before opening a new issue, please:

1. **Search existing issues** [open and closed](https://github.com/csaf-rs/csaf/issues?q=is%3Aissue%20(state%3Aopen%20OR%20state%3Aclosed)) to avoid duplicates.
2. **Use the correct template** — bug reports and feature requests have separate templates on GitHub.

A good bug report includes:
- A clear and descriptive title.
- Steps to reproduce the problem.
- Expected vs. actual behavior.
- CSAF document or minimal JSON snippet (if applicable).
- Your Rust version (`rustc --version`) and OS.
- Any relevant error messages or stack traces.

> [!IMPORTANT]
> **Do not open a pull request to fix a bug or add a feature without first opening a corresponding issue.** This allows maintainers to discuss the approach before implementation work begins.
> For minor fixes, like small typo corrections, a direct pull request is fine.

---

## Suggesting Features

Enhancement requests are welcome. Open an issue and describe:

- The problem you are trying to solve.
- Your proposed solution or API.
- Alternatives you have considered.
- Whether you are willing to implement it yourself.

Maintainers will confirm whether the feature fits the project scope before any implementation starts.

---

## Contributing Code

### Workflow

1. **Open or find an issue** that describes the change you want to make. If none exists, create one first (see above).
2. **Get the issue assigned** to you or leave a comment indicating you plan to work on it, to avoid duplicate effort.
3. **Create a branch** from `main` with a descriptive name:
   ```bash
   git checkout -b fix/issue-123-wrong-cvss-score
   # or
   git checkout -b feat/issue-456-ssvc-validation
   ```
4. **Implement your changes**, following the [Coding Standards](#coding-standards).
5. **Open a Pull Request** that references the issue (see [Pull Request Process](#pull-request-process)).

### Scope of Changes

- Keep pull requests focused. One logical change per PR makes review easier and history cleaner.
- Do not include unrelated refactors, formatting fixes, or dependency bumps in a feature or bug-fix PR.
- If you spot unrelated issues while working, open a separate issue or PR for them.

---

## Pull Request Process

### Requirements

- **Every PR must reference at least one issue** using a [closing keyword](https://docs.github.com/en/issues/tracking-your-work-with-issues/linking-a-pull-request-to-an-issue) in the PR description:
  ```
  Closes #123
  Fixes/Resolves #456
  ```
  PRs without a linked issue will not be reviewed until the corresponding issue is created and linked.
  Exception for small typo fixes.

- The PR title should follow the same format as [commit messages](#commit-message-guidelines).

- All CI checks must pass before a PR is eligible for review:
  - `cargo fmt --all --check`
  - `cargo clippy --all-targets -- -D warnings`
  - `cargo test --verbose`

- **Auto-generated files must not be manually edited.** If your change requires regenerating schema types or test harnesses, run the appropriate generator and include the generated output in the same PR:
  ```bash
  cargo run -p type-generator
  ```

- Commits authored by AI are not allowed (see [AI usage](#ai-usage).).

### Review Process

1. At least **one maintainer approval** is required to merge.
2. Address all review comments. Resolve threads only after the suggested change has been applied (or a counter-proposal agreed upon).
3. Prefer `--rebase` over merge commits when updating your branch against `main`.
4. Squash fixup commits before requesting a final review, unless the commit history tells a meaningful story.

### Draft PRs

Open a **Draft PR** early if you want feedback on your approach before the implementation is complete. Mark it ready for review only when all requirements above are met.

---

## Development Setup

```bash
# Sync submodules and asset files (required after checkout and after submodule updates)
git submodule update --remote
./update_assets.sh

# Format check
cargo fmt --all --check -- -l

# Lint
cargo clippy --all-targets -- -D warnings

# Run all tests
cargo test

# Regenerate schema types (after schema changes)
cargo run -p type-generator

```

Minimum Rust version: **1.88.0** (see `rust-version` in `Cargo.toml`).
Otherwise use **stable** as the CI pipeline also uses it.

---

## Coding Standards

- Follow idiomatic Rust style. Run `cargo fmt` before every commit.
- All `cargo clippy -- -D warnings` lints must pass.
- New validation tests must follow the existing naming convention (`test_6_1_XX.rs`) and use the generated test harnesses.
- Implement `TestValidator<CommonSecurityAdvisoryFramework>` for both the 2.0 and 2.1 types where applicable.
- Collect all validation errors rather than failing fast — return `Err(Vec<ValidationError>)`, not a single error.
- Use the `CsafTrait` / `VulnerabilityTrait` / `ProductTreeTrait` abstractions for version-agnostic logic.
- Avoid `unwrap()` and `expect()` in library code; reserve them for tests and clearly impossible cases.
- Do not introduce `unsafe` code without prior discussion in an issue.
- Update `validation.rs` to enable a new validation.
- Update the README to indicate the implementation state.

---

## Commit Message Guidelines

Use the [Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>(<scope>): <short summary>

[optional body]

[optional footer(s)]
```

**Types:** `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`, `ci`
**Scopes:** should be cargo projects, but are optional, i.e. `csaf-rs`, `csaf-converter`, `csaf-ffi`, `csaf-result-json`, `csaf-validator`

**Examples:**
```
feat(csaf-ffi): add GO bindings
fix(csaf-rs): correct instance_path for nested product group references
docs: update development setup in CONTRIBUTING.md
chore: regenerate schema types after CSAF 2.1 schema update
```

---

## AI usage

We can't be bothered to merge what you didn't bother to write. If a PR is made that is obviously AI we'll probably close without response, unless it's a small change that is easy to review. Of course in that case, why not write it yourself?
Additionally, coding agents are not allowed to make commits and you have to remove or rewrite those commits.

---

## Questions?

If you are unsure about anything, open a [discussion](https://github.com/csaf-rs/csaf/discussions) or ask in the relevant issue. We are happy to help.
