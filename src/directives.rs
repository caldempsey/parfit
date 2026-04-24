//! Heuristics for lines that are machine-readable directives
//! rather than prose. Derived from patterns observed across Go,
//! Rust, TypeScript / JavaScript, Python, and shell codebases.

/// Return true when the line looks like a directive that should
/// pass through the reflow pipeline verbatim.
pub(crate) fn is_default_directive(line: &str) -> bool {
    // Go directives and build tags.
    if line.starts_with("//go:")
        || line.starts_with("// +build")
        || line.starts_with("//nolint")
        || line.starts_with("//noinspection")
        || line.starts_with("//lint:")
    {
        return true;
    }

    // Rust attributes leaked into comment blocks (rare but real).
    if line.starts_with("#[") || line.starts_with("#![") {
        return true;
    }

    // Shebangs (shell, Python, Node scripts).
    if line.starts_with("#!/") {
        return true;
    }

    // Python pragmas and noqa markers.
    if line.starts_with("# type:") || line.starts_with("# noqa") || line.starts_with("# pragma:") {
        return true;
    }

    // TypeScript / JavaScript tooling directives.
    if line.starts_with("// @ts-")
        || line.starts_with("// eslint-")
        || line.starts_with("/* eslint-")
        || line.starts_with("// @param")
        || line.starts_with("// @returns")
        || line.starts_with("// @internal")
        || line.starts_with("// @deprecated")
        || line.starts_with("// @see")
    {
        return true;
    }

    // A line that is a lone URL with nothing else to wrap.
    if line.split_whitespace().count() == 1 && line.contains("://") {
        return true;
    }

    // Separator lines inside comments ("// ----" etc.).
    if line
        .chars()
        .all(|c| "-=*_".contains(c) || c.is_whitespace())
        && !line.trim().is_empty()
    {
        return true;
    }

    false
}
