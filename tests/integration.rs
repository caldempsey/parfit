use parfit::{reflow, Options};

#[test]
fn wraps_a_long_go_doc_comment() {
    let input = "\
// parfit is a codebase-aware comment reflow tool that wraps long \
lines at a sensible width while leaving machine-readable directives alone.
";
    let out = reflow(input, &Options::new(60));
    for line in out.lines() {
        assert!(line.chars().count() <= 60, "line too long: {:?}", line);
        assert!(line.starts_with("// "));
    }
}

#[test]
fn preserves_multiple_paragraphs() {
    let input = "// first paragraph here.\n\n// second paragraph.\n";
    let out = reflow(input, &Options::new(40));
    assert_eq!(out, input);
}

#[test]
fn leaves_go_generate_untouched() {
    let input = "//go:generate stringer -type=Foo\n";
    assert_eq!(reflow(input, &Options::new(20)), input);
}

#[test]
fn leaves_shebang_untouched() {
    let input = "#!/usr/bin/env bash\n";
    assert_eq!(reflow(input, &Options::new(10)), input);
}

#[test]
fn leaves_eslint_directive_untouched() {
    let input = "// eslint-disable-next-line no-console\n";
    assert_eq!(reflow(input, &Options::new(20)), input);
}

#[test]
fn leaves_ts_ignore_directive_untouched() {
    let input = "// @ts-ignore — this is a deliberate cast because reasons\n";
    assert_eq!(reflow(input, &Options::new(30)), input);
}

#[test]
fn custom_skip_regex_passes_line_through() {
    let input = "// TODO(cal): this is a very long line that normally would be wrapped at thirty cols\n";
    let opts = Options::new(30).with_skip(r"TODO\(").unwrap();
    assert_eq!(reflow(input, &opts), input);
}

#[test]
fn no_default_skips_wraps_directives() {
    let input = "//go:generate stringer with a really long tail that would otherwise get wrapped\n";
    let opts = Options::new(30).with_default_skips(false);
    let out = reflow(input, &opts);
    assert_ne!(out, input);
}

#[test]
fn preserves_indented_comment_prefix() {
    let input = "    // deeply indented comment that is long enough to need wrapping please\n";
    let out = reflow(input, &Options::new(50));
    for line in out.lines() {
        assert!(line.starts_with("    // "));
    }
}

#[test]
fn preserves_exact_url_body() {
    let input = "// see https://example.com/some/very/long/path/here for more\n";
    let out = reflow(input, &Options::new(40));
    assert!(out.contains("https://example.com/some/very/long/path/here"));
}

#[test]
fn hash_prefix_shell_style() {
    let input = "# this is a shell comment that is long enough to need wrapping at a sensible width\n";
    let out = reflow(input, &Options::new(40));
    for line in out.lines() {
        assert!(line.starts_with("# "));
    }
}

#[test]
fn separator_lines_pass_through() {
    let input = "// -----------------------------\n// header below\n";
    let out = reflow(input, &Options::new(30));
    assert!(out.contains("// -----------------------------"));
}

#[test]
fn empty_input_yields_empty_output() {
    assert_eq!(reflow("", &Options::new(40)), "");
}
