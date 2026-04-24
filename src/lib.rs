//! parfit — paragraph fit.
//!
//! Reflow prose in code comments to a target width using optimal-fit
//! line breaking. Lines that look like directives (`//go:generate`,
//! `// eslint-disable`, `#!/usr/bin/env bash`, `#[derive(…)]`, …)
//! pass through unchanged. Words that look like URLs never split.
//!
//! Two entry points:
//!
//! - [`reflow`]: treat the whole input as prose and wrap each
//!   paragraph. Used when the input is plain text (a README, an
//!   email, a comment block the caller has already extracted) or
//!   when no language context is available.
//! - [`reflow_source`]: given a source file and a [`Language`],
//!   reflow only the comment blocks and leave the code untouched.

mod directives;
mod prefix;
mod reflow;
mod source;

pub mod lang;
pub mod options;

pub use lang::Language;
pub use options::Options;
pub use reflow::reflow;
pub use source::reflow_source;
