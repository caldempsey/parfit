//! Source-aware reflow: walk a source file line by line, pick out
//! contiguous line-comment runs and C-family block-comment regions,
//! feed each region through [`reflow`], and copy every other line
//! through verbatim.
//!
//! Python docstrings (`""" … """`) are *not* recognised yet — they
//! are strings, not comments, and disambiguating a top-level
//! docstring from `x = """..."""` needs a real lexer. Line and
//! block comments cover the prose in most modern codebases.

use crate::lang::{Fence, Language, Spec};
use crate::options::Options;
use crate::reflow::reflow;

/// Reflow comment blocks inside a source file, leaving code alone.
/// When `language` is [`Language::Text`] this falls back to plain
/// [`reflow`]. Markdown is handled by applying the language's
/// ignore / fence markers to [`Options`] and running the whole
/// input through [`reflow`]: structural lines (headings, bullets,
/// tables) pass through verbatim, fenced code blocks pass through
/// verbatim, and the prose in between reflows normally.
pub fn reflow_source(input: &str, language: Language, opts: &Options) -> String {
    if language == Language::Text {
        return reflow(input, opts);
    }

    let spec = language.spec();

    if language == Language::Markdown {
        let opts = opts.clone().with_spec(spec);
        return reflow(input, &opts);
    }

    if spec.line_markers.is_empty() && spec.block_comments.is_empty() {
        return reflow(input, opts);
    }

    // Splits input into lines, classifies each one as line-comment,
    // block-comment-open, or code, gathers contiguous comment
    // regions, reflows them, and pushes everything else back out
    // unchanged.
    let lines: Vec<&str> = input.split_inclusive('\n').collect();
    let mut out = String::with_capacity(input.len());
    let mut i = 0;
    while i < lines.len() {
        let stripped = strip_newline(lines[i]);

        if is_line_comment(stripped, &spec) {
            let end = end_of_line_comment_run(&lines, i, &spec);
            out.push_str(&reflow(&lines[i..end].concat(), opts));
            i = end;
        } else if let Some(block) = find_block_comment_open(stripped, &spec) {
            let end = end_of_block_comment(&lines, i, block);
            out.push_str(&reflow(&lines[i..end].concat(), opts));
            i = end;
        } else {
            out.push_str(lines[i]);
            i += 1;
        }
    }
    out
}

/// A line qualifies as a line-comment line when, after any leading
/// whitespace, its first non-whitespace run starts with one of
/// the language's line-comment markers.
fn is_line_comment(line: &str, spec: &Spec) -> bool {
    let trimmed = line.trim_start();
    spec.line_markers.iter().any(|m| trimmed.starts_with(m))
}

/// Scan forward from `start` while lines keep qualifying as
/// line-comments; return the exclusive end index of the run.
fn end_of_line_comment_run(lines: &[&str], start: usize, spec: &Spec) -> usize {
    let mut i = start;
    while i < lines.len() && is_line_comment(strip_newline(lines[i]), spec) {
        i += 1;
    }
    i
}

/// If `line` (after leading whitespace) starts with one of the
/// language's block-comment opens, return the matching delimiter
/// pair.
fn find_block_comment_open(line: &str, spec: &Spec) -> Option<Fence> {
    let trimmed = line.trim_start();
    spec.block_comments
        .iter()
        .find(|b| trimmed.starts_with(b.open))
        .copied()
}

/// Scan forward from `start` (a line that opens a block comment)
/// until the matching close delimiter appears; return the
/// exclusive end index of the region, including the closing line.
/// Handles the single-line case (open and close on the same line)
/// and an unterminated region (scans to end of input).
fn end_of_block_comment(lines: &[&str], start: usize, block: Fence) -> usize {
    let first = strip_newline(lines[start]);
    let leading = first.len() - first.trim_start().len();
    let after_open = &first[leading + block.open.len()..];
    if after_open.contains(block.close) {
        return start + 1;
    }
    let mut i = start + 1;
    while i < lines.len() && !strip_newline(lines[i]).contains(block.close) {
        i += 1;
    }
    if i < lines.len() {
        i + 1
    } else {
        i
    }
}

fn strip_newline(s: &str) -> &str {
    s.strip_suffix('\n').unwrap_or(s)
}
