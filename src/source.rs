//! Source-aware reflow: walk a source file line by line, pick out
//! contiguous line-comment blocks, feed each block through
//! [`reflow`], and copy every non-comment line through verbatim.
//!
//! Block comments (`/* … */`, Python docstrings) are *not*
//! recognised yet; they pass through as code. Line comments cover
//! the prose in most modern codebases, and block comments have
//! enough edge cases (string literals, nested delimiters,
//! docstring vs string disambiguation) to warrant a real lexer.

use crate::lang::{Language, Spec};
use crate::options::Options;
use crate::reflow::reflow;

/// Reflow comment blocks inside a source file, leaving code alone.
/// When `language` is [`Language::Text`] this falls back to plain
/// [`reflow`].
pub fn reflow_source(input: &str, language: Language, opts: &Options) -> String {
    if language == Language::Text {
        return reflow(input, opts);
    }

    let spec = language.spec();
    if spec.line_markers.is_empty() {
        return reflow(input, opts);
    }

    let lines: Vec<&str> = input.split_inclusive('\n').collect();
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    while i < lines.len() {
        if is_comment_line(strip_newline(lines[i]), &spec) {
            let start = i;
            while i < lines.len() && is_comment_line(strip_newline(lines[i]), &spec) {
                i += 1;
            }
            let block: String = lines[start..i].concat();
            out.push_str(&reflow(&block, opts));
        } else {
            out.push_str(lines[i]);
            i += 1;
        }
    }
    out
}

fn strip_newline(s: &str) -> &str {
    s.strip_suffix('\n').unwrap_or(s)
}

/// A line qualifies as a comment line when, after any leading
/// whitespace, its first non-whitespace run starts with one of
/// the language's line-comment markers.
fn is_comment_line(line: &str, spec: &Spec) -> bool {
    let trimmed = line.trim_start();
    spec.line_markers.iter().any(|m| trimmed.starts_with(m))
}
