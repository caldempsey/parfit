# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/caldempsey/parfit/compare/v0.3.0...v0.4.0) - 2026-04-24

### Added

- *(source)* recognize and reflow block comments; extract caller-above-callee helpers
- *(lang)* add block_comments field to Spec for paired-delimiter comment regions
- *(reflow)* stack-based fence matcher + ignore_marker passthrough
- *(lang)* add Markdown language with ignore_markers and fences

### Fixed

- *(walker)* use io::Error::other per clippy's io_other_error lint

### Other

- re-enable release workflow on push to dev
- *(githooks)* add pre-commit hook running cargo fmt check
- cargo fmt and clippy empty-line-after-doc fix
- *(readme)* note block_comments as inverse of fences and add 2x2 marker matrix
- *(block-comments)* cover rust javadoc, go single-line, c multi-line
- mirror pushes to sr.ht on every branch and tag update
- pause releases on workflow_dispatch while foundations settle
- *(readme)* document markdown mode; credit LeetCode 20 for the stack pattern
- *(markdown)* worked examples for headings, fences, bullets, blockquotes, tables
- *(readme)* add hitchhiker's example and self-dogfood note
- *(contributing)* describe conventional commits and the release-plz flow
- add release-plz workflow with auto fast-forward of rel
- add GitHub Actions workflow for build, test, fmt, and clippy
- apply cargo fmt
