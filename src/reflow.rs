//! Paragraph extraction + the reflow pipeline.

use textwrap::{wrap_algorithms::WrapAlgorithm, Options as TwOptions, WordSeparator};

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
    let mut i = 0;
    while i < lines.len() {
        let line = strip_newline(lines[i]);

        // Blank line — preserve as-is.
        if line.trim().is_empty() {
            out.push_str(lines[i]);
            i += 1;
            continue;
        }

        // Detect this paragraph's prefix from the first line.
        let prefix = opts
            .forced_prefix
            .clone()
            .unwrap_or_else(|| detect_prefix(line));

        // Walk forward while the next line has the same prefix
        // and is not a paragraph break.
        let start = i;
        while i < lines.len() {
            let l = strip_newline(lines[i]);
            if l.trim().is_empty() {
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

fn strip_newline(s: &str) -> &str {
    s.strip_suffix('\n').unwrap_or(s)
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

    let stripped: Vec<&str> = lines
        .iter()
        .map(|l| l.strip_prefix(prefix).unwrap_or(*l))
        .collect();

    let prose: String = stripped
        .iter()
        .map(|l| l.trim())
        .collect::<Vec<_>>()
        .join(" ");

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
