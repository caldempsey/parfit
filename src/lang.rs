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
}

/// Line-comment markers a language accepts. One language can have
/// more than one (e.g. SQL is `--` but some dialects also accept
/// `#`).
#[derive(Clone, Copy, Debug)]
pub struct Spec {
    pub line_markers: &'static [&'static str],
}

impl Language {
    pub fn spec(self) -> Spec {
        match self {
            Language::Text => Spec { line_markers: &[] },
            Language::Python => Spec {
                line_markers: &["#"],
            },
            Language::Shell => Spec {
                line_markers: &["#"],
            },
            Language::Elixir => Spec {
                line_markers: &["#"],
            },
            Language::Go => Spec {
                line_markers: &["//"],
            },
            Language::Rust => Spec {
                line_markers: &["//", "///", "//!"],
            },
            Language::JavaScript => Spec {
                line_markers: &["//"],
            },
            Language::Java => Spec {
                line_markers: &["//"],
            },
            Language::Scala => Spec {
                line_markers: &["//"],
            },
            Language::C => Spec {
                line_markers: &["//"],
            },
            Language::Lua => Spec {
                line_markers: &["--"],
            },
            Language::SQL => Spec {
                line_markers: &["--"],
            },
            Language::Lisp => Spec {
                line_markers: &[";;", ";"],
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
            other => return Err(format!("unknown language: {other}")),
        })
    }
}
