//! Comment-prefix detection.

/// Detect the leading whitespace plus comment marker on a line and
/// return the exact byte prefix to preserve on continuation lines.
/// Handles `//`, `///`, `//!`, `#`, `;`, ` * ` (C multi-line
/// continuation), and falls back to the bare leading whitespace
/// when nothing matches.
pub(crate) fn detect_prefix(line: &str) -> String {
    let ws_end = line
        .find(|c: char| !c.is_whitespace())
        .unwrap_or(line.len());
    let (ws, rest) = line.split_at(ws_end);

    for marker in [
        "///", "//!", "//", "/**", "/*", "*/", " * ", "*", "#!", "#", ";",
    ] {
        if rest.starts_with(marker) {
            // Include one space after the marker if present. The
            // conventional code-comment look is `// foo`, not `//foo`.
            let mut end = ws.len() + marker.len();
            if line[end..].starts_with(' ') {
                end += 1;
            }
            return line[..end].to_string();
        }
    }

    ws.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_slash_slash_prefix() {
        assert_eq!(detect_prefix("// hello"), "// ");
        assert_eq!(detect_prefix("    // hello"), "    // ");
        assert_eq!(detect_prefix("//hello"), "//");
    }

    #[test]
    fn detects_hash_prefix() {
        assert_eq!(detect_prefix("# hello"), "# ");
        assert_eq!(detect_prefix("#hello"), "#");
    }

    #[test]
    fn detects_rustdoc_prefixes() {
        assert_eq!(detect_prefix("/// hello"), "/// ");
        assert_eq!(detect_prefix("//! hello"), "//! ");
    }
}
