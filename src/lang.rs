//! Language definitions and comment markers. Keeps the set of
//! supported languages in one table so the rest of the crate does
//! not need to know what `.py` means.

use std::path::Path;
use std::str::FromStr;

/// Languages parfit knows how to parse. `Text` is the fallback:
/// whole-input paragraph reflow, no code / comment distinction.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    Text,
    Python,
    Shell,
    Elixir,
    Go,
    Rust,
    JavaScript,
    Java,
    Scala,
    C,
    Lua,
    SQL,
    Lisp,
    Markdown,
}

/// A paired-delimiter region: an `open` string and a matching
/// `close`.  Used in [`Spec`] for two opposite purposes — Markdown
/// code fences (pass the contents through verbatim) and C-family
/// block comments (reflow the contents as prose).  The field name
/// in [`Spec`] picks the treatment; the shape is identical.
#[derive(Clone, Copy, Debug)]
pub struct Fence {
    pub open: &'static str,
    pub close: &'static str,
}

/// Per-language markers the reflow pipeline consults.
///
/// * `line_markers` — a line that starts (after indent) with one
///   of these is a line-comment line. Source-mode gathers runs of
///   those lines and reflows them as prose.
///
/// * `ignore_markers` — a line that starts (after indent) with
///   one of these passes through verbatim. Used for structural
///   markup that must not be reflowed: Markdown headings, list
///   bullets, blockquote arrows, table rows.
///
/// * `fences` — paired-delimiter regions whose contents pass
///   through verbatim.  Markdown code fences today; raw-string
///   literals in future languages slot in alongside without
///   touching the reflow pipeline.
///
/// * `block_comments` — paired-delimiter regions whose contents
///   are prose and reflow.  The inverse of `fences`: same type,
///   opposite treatment — `fences` preserve contents byte-for-
///   byte, `block_comments` feed contents to the reflow pipeline.
///   C-family `/* … */` is the canonical case.
#[derive(Clone, Copy, Debug)]
pub struct Spec {
    pub line_markers: &'static [&'static str],
    pub ignore_markers: &'static [&'static str],
    pub fences: &'static [Fence],
    pub block_comments: &'static [Fence],
}

impl Language {
    pub fn spec(self) -> Spec {
        match self {
            Language::Text => Spec {
                line_markers: &[],
                ignore_markers: &[],
                fences: &[],
                block_comments: &[],
            },
            Language::Python => Spec {
                line_markers: &["#"],
                ignore_markers: &[],
                fences: &[],
                block_comments: &[],
            },
            Language::Shell => Spec {
                line_markers: &["#"],
                ignore_markers: &[],
                fences: &[],
                block_comments: &[],
            },
            Language::Elixir => Spec {
                line_markers: &["#"],
                ignore_markers: &[],
                fences: &[],
                block_comments: &[],
            },
            Language::Go => Spec {
                line_markers: &["//"],
                ignore_markers: &[],
                fences: &[],
                block_comments: &[Fence {
                    open: "/*",
                    close: "*/",
                }],
            },
            Language::Rust => Spec {
                line_markers: &["//", "///", "//!"],
                ignore_markers: &[],
                fences: &[],
                block_comments: &[Fence {
                    open: "/*",
                    close: "*/",
                }],
            },
            Language::JavaScript => Spec {
                line_markers: &["//"],
                ignore_markers: &[],
                fences: &[],
                block_comments: &[Fence {
                    open: "/*",
                    close: "*/",
                }],
            },
            Language::Java => Spec {
                line_markers: &["//"],
                ignore_markers: &[],
                fences: &[],
                block_comments: &[Fence {
                    open: "/*",
                    close: "*/",
                }],
            },
            Language::Scala => Spec {
                line_markers: &["//"],
                ignore_markers: &[],
                fences: &[],
                block_comments: &[Fence {
                    open: "/*",
                    close: "*/",
                }],
            },
            Language::C => Spec {
                line_markers: &["//"],
                ignore_markers: &[],
                fences: &[],
                block_comments: &[Fence {
                    open: "/*",
                    close: "*/",
                }],
            },
            Language::Lua => Spec {
                line_markers: &["--"],
                ignore_markers: &[],
                fences: &[],
                block_comments: &[],
            },
            Language::SQL => Spec {
                line_markers: &["--"],
                ignore_markers: &[],
                fences: &[],
                block_comments: &[],
            },
            Language::Lisp => Spec {
                line_markers: &[";;", ";"],
                ignore_markers: &[],
                fences: &[],
                block_comments: &[],
            },
            Language::Markdown => Spec {
                line_markers: &[],
                ignore_markers: &["#", "- ", "* ", "+ ", "> ", "|"],
                fences: &[
                    Fence {
                        open: "```",
                        close: "```",
                    },
                    Fence {
                        open: "~~~",
                        close: "~~~",
                    },
                ],
                block_comments: &[],
            },
        }
    }

    /// Infer the language from a file path's extension. Returns
    /// [`Language::Text`] when the extension is unknown or absent.
    pub fn from_path(path: &Path) -> Self {
        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(e) => e.to_ascii_lowercase(),
            None => return Language::Text,
        };
        match ext.as_str() {
            "py" | "pyi" => Language::Python,
            "sh" | "bash" | "zsh" | "ksh" | "fish" => Language::Shell,
            "ex" | "exs" => Language::Elixir,
            "go" => Language::Go,
            "rs" => Language::Rust,
            "js" | "mjs" | "cjs" | "jsx" | "ts" | "tsx" => Language::JavaScript,
            "java" => Language::Java,
            "scala" | "sc" => Language::Scala,
            "c" | "h" | "cc" | "cpp" | "cxx" | "hpp" | "hh" | "m" | "mm" => Language::C,
            "lua" => Language::Lua,
            "sql" => Language::SQL,
            "el" | "lisp" | "clj" | "cljs" | "cljc" | "scm" | "ss" | "rkt" => Language::Lisp,
            "md" | "markdown" | "mkd" => Language::Markdown,
            _ => Language::Text,
        }
    }
}

impl FromStr for Language {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_lowercase().as_str() {
            "text" | "txt" | "plain" => Language::Text,
            "python" | "py" => Language::Python,
            "shell" | "sh" | "bash" => Language::Shell,
            "elixir" | "ex" => Language::Elixir,
            "go" => Language::Go,
            "rust" | "rs" => Language::Rust,
            "javascript" | "js" | "typescript" | "ts" => Language::JavaScript,
            "java" => Language::Java,
            "scala" => Language::Scala,
            "c" | "cpp" | "c++" => Language::C,
            "lua" => Language::Lua,
            "sql" => Language::SQL,
            "lisp" | "clojure" | "scheme" | "racket" => Language::Lisp,
            "markdown" | "md" => Language::Markdown,
            other => return Err(format!("unknown language: {other}")),
        })
    }
}
