//! Worked examples — each test is a real before-and-after that
//! doubles as user-facing documentation. When you change parfit's
//! behaviour, this file shows which scenarios shifted and how.

use parfit::{reflow, reflow_source, Language, Options};

// -------------------------------------------------------------
// Plain-text / comment-block mode (reflow)
// -------------------------------------------------------------

#[test]
fn petunias_paragraph_showcase() {
    let input = "\
// Curiously enough, the only thing that went through the mind of the bowl of petunias as it fell was 'Oh no, not again.' Many people have speculated that if we knew exactly why the bowl of petunias had thought that we would know a lot more about the nature of the universe than we do now.
";
    let expected = "\
// Curiously enough, the only thing that went through the mind of
// the bowl of petunias as it fell was 'Oh no, not again.' Many
// people have speculated that if we knew exactly why the bowl of
// petunias had thought that we would know a lot more about the
// nature of the universe than we do now.
";
    assert_eq!(reflow(input, &Options::new(68)), expected);
}

#[test]
fn preserves_paragraph_breaks() {
    let input = "\
// First paragraph is long enough to wrap at a sensible width of sixty-eight columns when it runs through parfit here.
//
// Second paragraph is similarly long and stands on its own because of the blank comment line in between the two.
";
    let expected = "\
// First paragraph is long enough to wrap at a sensible width of
// sixty-eight columns when it runs through parfit here.
//
// Second paragraph is similarly long and stands on its own because
// of the blank comment line in between the two.
";
    assert_eq!(reflow(input, &Options::new(68)), expected);
}

#[test]
fn urls_are_never_split() {
    let input = "\
// See https://example.com/a/very/long/url/that/should/not/be/split for more background on why this matters to us.
";
    let expected = "\
// See https://example.com/a/very/long/url/that/should/not/be/split
// for more background on why this matters to us.
";
    assert_eq!(reflow(input, &Options::new(68)), expected);
}

#[test]
fn preserves_indented_comment_prefix() {
    // NB: Rust's `"\` line-continuation escape eats leading
    // whitespace on the next line, which would silently strip
    // the four-space indent on line one. Build the strings with
    // explicit newlines instead.
    let input = "    // A deeply indented comment that is long enough \
                 to require wrapping while preserving the four-space \
                 indent on every line.\n";
    let expected = concat!(
        "    // A deeply indented comment that is long enough to require\n",
        "    // wrapping while preserving the four-space indent on every\n",
        "    // line.\n",
    );
    assert_eq!(reflow(input, &Options::new(68)), expected);
}

#[test]
fn rustdoc_triple_slash_prefix() {
    let input = "\
/// A rustdoc comment that is plenty long enough to require wrapping while preserving the triple-slash prefix on every continuation line.
";
    let expected = "\
/// A rustdoc comment that is plenty long enough to require wrapping
/// while preserving the triple-slash prefix on every continuation
/// line.
";
    assert_eq!(reflow(input, &Options::new(68)), expected);
}

#[test]
fn rfc_style_indented_block_passes_through() {
    let input = "\
A short intro paragraph that flows normally across lines when parfit wraps it to sixty-eight columns.

    a four-space indented line    intentional double spaces    here
    another indented line with    columnar    alignment
";
    let expected = "\
A short intro paragraph that flows normally across lines when parfit
wraps it to sixty-eight columns.

    a four-space indented line    intentional double spaces    here
    another indented line with    columnar    alignment
";
    assert_eq!(reflow(input, &Options::new(68)), expected);
}

// -------------------------------------------------------------
// Source-file mode (reflow_source) — code never touched
// -------------------------------------------------------------

#[test]
fn go_source_wraps_doc_comments_code_untouched() {
    let input = "\
package foo

// Foo does something that is described in quite a lot of detail here because it has a long doc comment written by someone thorough.
func Foo() int {
    return 42
}
";
    let expected = "\
package foo

// Foo does something that is described in quite a lot of detail
// here because it has a long doc comment written by someone
// thorough.
func Foo() int {
    return 42
}
";
    assert_eq!(reflow_source(input, Language::Go, &Options::new(68)), expected);
}

#[test]
fn go_source_passes_go_generate_directive_through() {
    let input = "\
package foo

//go:generate stringer -type=Kind
func Bar() {}
";
    // Directive line passes through unchanged even though it is
    // long enough to trigger a wrap.
    assert_eq!(reflow_source(input, Language::Go, &Options::new(68)), input);
}

#[test]
fn python_source_wraps_hash_comment_code_untouched() {
    let input = "\
def compute(x: int) -> int:
    # This function doubles the input because that is what the calling code expects and we documented it so here we are.
    return x * 2
";
    let expected = "\
def compute(x: int) -> int:
    # This function doubles the input because that is what the
    # calling code expects and we documented it so here we are.
    return x * 2
";
    assert_eq!(
        reflow_source(input, Language::Python, &Options::new(68)),
        expected
    );
}

#[test]
fn shell_source_preserves_shebang_wraps_comment() {
    let input = "\
#!/usr/bin/env bash
# This script does the thing described at length here because someone wrote a verbose comment and we want parfit to wrap it but not the shebang above.
set -euo pipefail
";
    let expected = "\
#!/usr/bin/env bash
# This script does the thing described at length here because
# someone wrote a verbose comment and we want parfit to wrap it but
# not the shebang above.
set -euo pipefail
";
    assert_eq!(
        reflow_source(input, Language::Shell, &Options::new(68)),
        expected
    );
}

#[test]
fn typescript_source_passes_ts_ignore_through() {
    let input = "\
// @ts-ignore: a deliberate override because the upstream types are wrong in this obscure edge case we hit exactly once.
const x: unknown = doThing();
";
    // @ts-ignore is a directive; parfit leaves it long rather than
    // wrapping, because wrapping would break its semantics.
    assert_eq!(
        reflow_source(input, Language::JavaScript, &Options::new(68)),
        input
    );
}

#[test]
fn javascript_source_passes_eslint_disable_through() {
    let input = "\
// eslint-disable-next-line no-console — we genuinely want the console here because the CLI ships its output through process.stdout and this is the cleanest place to do it.
console.log(\"hello\");
";
    assert_eq!(
        reflow_source(input, Language::JavaScript, &Options::new(68)),
        input
    );
}
