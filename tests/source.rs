use parfit::{reflow_source, Language, Options};

#[test]
fn python_reflows_hash_comments_leaves_code_alone() {
    let src = "\
def f():
    # this is a really long comment that ought to be reflowed at a narrow width like thirty-two because it is long
    x = 42
    return x
";
    let out = reflow_source(src, Language::Python, &Options::new(32));
    // Code lines are byte-for-byte preserved.
    assert!(out.contains("def f():\n"));
    assert!(out.contains("    x = 42\n"));
    assert!(out.contains("    return x\n"));
    // Comment was wrapped.
    assert!(
        !out.contains("this is a really long comment that ought to be reflowed at a narrow width")
    );
    for line in out.lines() {
        if line.trim_start().starts_with('#') {
            assert!(line.chars().count() <= 32, "wrapped line too long: {line}");
        }
    }
}

#[test]
fn go_reflows_slash_slash_comments() {
    let src = "\
package foo

// this is a doc comment that needs wrapping because it is longer than thirty-two columns at least
func Bar() {
    // inline comment is also long enough to need wrapping when we run parfit at a narrow width
    return
}
";
    let out = reflow_source(src, Language::Go, &Options::new(32));
    assert!(out.contains("package foo\n"));
    assert!(out.contains("func Bar() {\n"));
    assert!(out.contains("    return\n"));
    assert!(out.contains("}\n"));
}

#[test]
fn shell_reflows_hash_comments() {
    let src = "\
#!/usr/bin/env bash
# this is a shell script comment that parfit should wrap at thirty-two columns but the shebang above should pass through
set -euo pipefail
";
    let out = reflow_source(src, Language::Shell, &Options::new(32));
    assert!(out.contains("#!/usr/bin/env bash\n"));
    assert!(out.contains("set -euo pipefail\n"));
}

#[test]
fn rust_reflows_line_and_doc_comments() {
    let src = "\
/// this is a rustdoc line that is plenty long enough to require wrapping at a narrow target width of thirty-two or so
pub fn f() {
    // inline comment long enough to wrap as well when we aim for thirty-two columns
    let _ = 42;
}
";
    let out = reflow_source(src, Language::Rust, &Options::new(32));
    assert!(out.contains("pub fn f() {\n"));
    assert!(out.contains("    let _ = 42;\n"));
}

#[test]
fn text_mode_reflows_everything() {
    let src = "a very long paragraph that absolutely has to wrap at twenty cols for this test to prove anything at all\n";
    let out = reflow_source(src, Language::Text, &Options::new(20));
    for line in out.lines() {
        assert!(line.chars().count() <= 20);
    }
}

#[test]
fn language_inferred_from_path_extension() {
    use std::path::Path;
    assert_eq!(Language::from_path(Path::new("foo.py")), Language::Python);
    assert_eq!(Language::from_path(Path::new("src/lib.rs")), Language::Rust);
    assert_eq!(Language::from_path(Path::new("main.go")), Language::Go);
    assert_eq!(Language::from_path(Path::new("a.ts")), Language::JavaScript);
    assert_eq!(Language::from_path(Path::new("build.sh")), Language::Shell);
    assert_eq!(Language::from_path(Path::new("M.ex")), Language::Elixir);
    assert_eq!(Language::from_path(Path::new("x.scala")), Language::Scala);
    assert_eq!(Language::from_path(Path::new("y.java")), Language::Java);
    assert_eq!(Language::from_path(Path::new("z.cpp")), Language::C);
    assert_eq!(Language::from_path(Path::new("q.lua")), Language::Lua);
    assert_eq!(Language::from_path(Path::new("README")), Language::Text);
    assert_eq!(
        Language::from_path(Path::new("notes.md")),
        Language::Markdown
    );
    assert_eq!(
        Language::from_path(Path::new("notes.markdown")),
        Language::Markdown
    );
}

#[test]
fn language_name_parse() {
    use std::str::FromStr;
    assert_eq!(Language::from_str("python").unwrap(), Language::Python);
    assert_eq!(Language::from_str("PY").unwrap(), Language::Python);
    assert_eq!(Language::from_str("text").unwrap(), Language::Text);
    assert!(Language::from_str("haskell").is_err());
}
