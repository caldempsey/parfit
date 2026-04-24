//! Markdown-mode worked examples. Each test is a concrete
//! before / after that doubles as documentation for what parfit
//! will and will not touch in a Markdown file.

use parfit::{reflow_source, Language, Options};

fn md(input: &str) -> String {
    reflow_source(input, Language::Markdown, &Options::new(68))
}

#[test]
fn heading_passes_through_prose_wraps() {
    let input = "\
# Getting started

parfit is a reflow tool for code comments that wraps long prose at a sensible width while leaving machine-readable directives alone.
";
    let expected = "\
# Getting started

parfit is a reflow tool for code comments that wraps long prose at a
sensible width while leaving machine-readable directives alone.
";
    assert_eq!(md(input), expected);
}

#[test]
fn fenced_code_block_passes_through_surrounding_prose_wraps() {
    let input = "\
Install parfit with cargo, which is the simplest and most portable way to get the binary onto a development machine.

```
cargo install parfit
parfit --version
```

And then you are ready to start reflowing comments in your editor or at the shell.
";
    let expected = "\
Install parfit with cargo, which is the simplest and most portable
way to get the binary onto a development machine.

```
cargo install parfit
parfit --version
```

And then you are ready to start reflowing comments in your editor or
at the shell.
";
    assert_eq!(md(input), expected);
}

#[test]
fn bullet_list_passes_through_surrounding_prose_wraps() {
    let input = "\
parfit supports several different code languages out of the box, each with their own comment markers and pragma conventions.

- Go (`//` with directive support for `//go:generate`)
- Rust (`//`, `///`, `//!`)
- Python (`#` with awareness of `# type:` and `# noqa`)

Any other extension falls through to plain-text paragraph reflow which is the appropriate default for plain documentation and other prose content.
";
    let expected = "\
parfit supports several different code languages out of the box,
each with their own comment markers and pragma conventions.

- Go (`//` with directive support for `//go:generate`)
- Rust (`//`, `///`, `//!`)
- Python (`#` with awareness of `# type:` and `# noqa`)

Any other extension falls through to plain-text paragraph reflow
which is the appropriate default for plain documentation and other
prose content.
";
    assert_eq!(md(input), expected);
}

#[test]
fn blockquote_passes_through_even_when_long() {
    let input = "\
Inspired by par (1993):

> par was a brilliant little paragraph reformatter that inspired a lot of tools after it including the one you are currently reading about.

parfit carries the same niche forward with better defaults.
";
    let expected = "\
Inspired by par (1993):

> par was a brilliant little paragraph reformatter that inspired a lot of tools after it including the one you are currently reading about.

parfit carries the same niche forward with better defaults.
";
    assert_eq!(md(input), expected);
}

#[test]
fn table_rows_pass_through() {
    let input = "\
A tiny table, which absolutely must survive reflow unscathed so users can read the columns and their contents without any damage done.

| column one | column two |
|------------|------------|
| a          | b          |

That is the whole table; nothing more to say about it.
";
    let expected = "\
A tiny table, which absolutely must survive reflow unscathed so
users can read the columns and their contents without any damage
done.

| column one | column two |
|------------|------------|
| a          | b          |

That is the whole table; nothing more to say about it.
";
    assert_eq!(md(input), expected);
}

#[test]
fn three_consecutive_fences_balance_like_valid_parentheses() {
    // Three fence markers in a row: open, close, open. The stack
    // pattern handles this — middle fence closes the first block,
    // third fence opens a new one that never closes. All lines
    // still pass through because they are delimiters or inside
    // an open fence.
    let input = "```\n```\n```\n";
    assert_eq!(md(input), input);
}
