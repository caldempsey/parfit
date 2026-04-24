//! Paragraph extraction + the reflow pipeline.

use textwrap::{wrap_algorithms::WrapAlgorithm, Options as TwOptions, WordSeparator};

use crate::lang::Fence;
use crate::options::Options;
use crate::prefix::detect_prefix;

/// Reflow a block of text to the target width. The prefix (e.g.
/// `// `) is detected from the input and preserved on every
/// output line.
pub fn reflow(input: &str, opts: &Options) -> String {
    let lines: Vec<&str> = input.split_inclusive('\n').collect();
    if lines.is_empty() {
        return String::new();
    }

    let mut out = String::with_capacity(input.len());
    let mut fences = FenceStack::new(opts.fences);
    let mut i = 0;
    while i < lines.len() {
        let line = strip_newline(lines[i]);

        // Fenced region (Markdown ``` / ~~~). The stack tells us
        // whether this line is inside a fence and whether it is
        // itself a delimiter. Either way, the line passes through
        // unchanged.
        if fences.consume(line) {
            out.push_str(lines[i]);
            i += 1;
            continue;
        }

        // Blank line — preserve as-is.
        if line.trim().is_empty() {
            out.push_str(lines[i]);
            i += 1;
            continue;
        }

        // Ignore-marker line (Markdown headings, bullets,
        // blockquotes, table rows) — pass through verbatim.
        if opts.matches_ignore_marker(line) {
            out.push_str(lines[i]);
            i += 1;
            continue;
        }

        // Detect this paragraph's prefix from the first line.
        let prefix = opts
            .forced_prefix
            .clone()
            .unwrap_or_else(|| detect_prefix(line));

        // Walk forward while the next line has the same prefix,
        // is not a paragraph break, and is not a structural line
        // that must stay on its own.
        let start = i;
        while i < lines.len() {
            let l = strip_newline(lines[i]);
            if l.trim().is_empty() {
                break;
            }
            if opts.matches_ignore_marker(l) {
                break;
            }
            if fences.peek(l) {
                break;
            }
            if detect_prefix(l) != prefix && opts.forced_prefix.is_none() {
                break;
            }
            i += 1;
        }

        let para_lines: Vec<&str> = lines[start..i].iter().map(|l| strip_newline(l)).collect();

        emit_paragraph(&para_lines, &prefix, opts, &mut out);
    }

    out
}

/// A paragraph is a run of non-blank lines sharing a prefix.
/// Classify it and either pass it through or reflow it.
fn emit_paragraph(lines: &[&str], prefix: &str, opts: &Options, out: &mut String) {
    // Paragraphs whose prefix is pure whitespace of four or more
    // spaces are preformatted blocks (code examples in READMEs,
    // RFC-style indented verbatim, markdown code blocks). Pass
    // them through untouched so column alignment survives.
    let pure_indent = prefix.chars().all(char::is_whitespace) && prefix.chars().count() >= 4;
    if pure_indent {
        for l in lines {
            out.push_str(l);
            out.push('\n');
        }
        return;
    }

    // Directive detection runs against the ORIGINAL line (including
    // any leading whitespace and the comment marker) because the
    // default patterns — `//go:`, `// eslint-`, `#!/`, `// @ts-` —
    // all live on the comment marker itself.
    if lines.iter().any(|l| opts.matches_skip(l)) {
        for l in lines {
            out.push_str(l);
            out.push('\n');
        }
        return;
    }

    // Join the paragraph into one string: textwrap's optimal-fit
    // needs global access to all words to minimise badness.
    let mut prose = String::new();
    for l in lines {
        let trimmed = l.strip_prefix(prefix).unwrap_or(l).trim();
        if !prose.is_empty() {
            prose.push(' ');
        }
        prose.push_str(trimmed);
    }

    if prose.is_empty() {
        for l in lines {
            out.push_str(l);
            out.push('\n');
        }
        return;
    }

    let body_width = opts.width.saturating_sub(prefix.chars().count()).max(10);

    let tw = TwOptions::new(body_width)
        .wrap_algorithm(WrapAlgorithm::new_optimal_fit())
        .word_separator(WordSeparator::AsciiSpace)
        .break_words(false);

    for line in textwrap::wrap(&prose, &tw) {
        out.push_str(prefix);
        out.push_str(&line);
        out.push('\n');
    }
}

/// Stack-based matcher for paired delimiter regions — fenced code
/// blocks today, block comments in future languages. The shape is
/// the "valid parentheses" pattern (LeetCode 20): push on an open
/// marker, pop on the matching close, inspect the top to know the
/// current enclosing region, but operate at line granularity.
///
/// Per call: O(k) where k is the number of configured fences (two
/// for Markdown, zero for every other language). Amortized O(1).
struct FenceStack {
    configured: &'static [Fence],
    open: Vec<&'static Fence>,
}

impl FenceStack {
    fn new(configured: &'static [Fence]) -> Self {
        Self {
            configured,
            open: Vec::new(),
        }
    }

    /// Feed one line. Returns `true` if the line is part of a
    /// fenced region (either a delimiter or content inside).
    /// Mutates the stack on a matching open or close.
    fn consume(&mut self, line: &str) -> bool {
        let trimmed = line.trim_start();

        // Currently inside a region — look for its close.
        if let Some(top) = self.open.last().copied() {
            if trimmed.starts_with(top.close) {
                self.open.pop();
            }
            return true;
        }

        // Outside any region — look for a fresh open.
        for fence in self.configured {
            if trimmed.starts_with(fence.open) {
                self.open.push(fence);
                return true;
            }
        }

        false
    }

    /// Same predicate as [`consume`] but without mutating the
    /// stack. Helps the paragraph decide whether to
    /// stop collecting lines for the current paragraph.
    fn peek(&self, line: &str) -> bool {
        let trimmed = line.trim_start();
        if let Some(top) = self.open.last() {
            return trimmed.starts_with(top.close);
        }
        self.configured.iter().any(|f| trimmed.starts_with(f.open))
    }
}

fn strip_newline(s: &str) -> &str {
    s.strip_suffix('\n').unwrap_or(s)
}
