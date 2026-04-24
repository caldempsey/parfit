//! Configuration for [`reflow`](crate::reflow).

use regex::Regex;

use crate::directives::is_default_directive;

/// Options controlling how parfit reflows a block of text.
#[derive(Clone, Debug)]
pub struct Options {
    /// Target width in columns, inclusive of the detected prefix.
    pub width: usize,
    /// When set, overrides prefix auto-detection.
    pub forced_prefix: Option<String>,
    pub(crate) default_skips: bool,
    pub(crate) extra_skips: Vec<Regex>,
}

impl Options {
    /// Build a default configuration at the given width. The
    /// built-in skip list is on by default.
    pub fn new(width: usize) -> Self {
        Self {
            width,
            forced_prefix: None,
            default_skips: true,
            extra_skips: Vec::new(),
        }
    }

    /// Toggle the built-in directive skip list.
    pub fn with_default_skips(mut self, on: bool) -> Self {
        self.default_skips = on;
        self
    }

    /// Force an exact comment prefix for every paragraph.
    pub fn with_forced_prefix(mut self, prefix: String) -> Self {
        self.forced_prefix = Some(prefix);
        self
    }

    /// Add a user regex to the skip list. Matching lines pass
    /// through unchanged.
    pub fn with_skip(mut self, pattern: &str) -> Result<Self, regex::Error> {
        self.extra_skips.push(Regex::new(pattern)?);
        Ok(self)
    }

    pub(crate) fn matches_skip(&self, line: &str) -> bool {
        let trimmed = line.trim_start();
        if self.default_skips && is_default_directive(trimmed) {
            return true;
        }
        self.extra_skips.iter().any(|r| r.is_match(line))
    }
}
