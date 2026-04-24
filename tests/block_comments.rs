use parfit::{reflow_source, Language, Options};

#[test]
fn rust_reflows_javadoc_style_block_comment() {
    let src = "\
pub struct Foo;

/**
 * This is a javadoc-style block comment that is deliberately much longer than the target width so that parfit has to reflow it across lines.
 */
impl Foo {}
";
    let out = reflow_source(src, Language::Rust, &Options::new(40));
    assert!(out.contains("pub struct Foo;\n"));
    assert!(out.contains("impl Foo {}\n"));
    assert!(!out.contains(
        "This is a javadoc-style block comment that is deliberately much longer than the target width"
    ));
    for line in out.lines() {
        if line.trim_start().starts_with('*') {
            assert!(line.chars().count() <= 40, "wrapped line too long: {line}");
        }
    }
}

#[test]
fn go_passes_through_single_line_block_comment_context() {
    let src = "\
package main

/* short */
func main() {}
";
    let out = reflow_source(src, Language::Go, &Options::new(40));
    assert!(out.contains("package main\n"));
    assert!(out.contains("func main() {}\n"));
}

#[test]
fn c_code_outside_block_comment_is_preserved() {
    let src = "\
int x = 42;
/*
 * A long enough comment that absolutely must reflow at a narrow width like thirty.
 */
int y = 7;
";
    let out = reflow_source(src, Language::C, &Options::new(30));
    assert!(out.contains("int x = 42;\n"));
    assert!(out.contains("int y = 7;\n"));
    assert!(!out.contains(
        "A long enough comment that absolutely must reflow at a narrow width like thirty."
    ));
}
