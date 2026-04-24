use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use clap::Parser;

use parfit::{reflow, reflow_source, Language, Options};

mod walker;

/// Paragraph fit — a codebase-aware comment reflow tool.
///
/// Without path arguments, parfit reads stdin and writes stdout.
/// With path arguments, parfit rewrites the files in place by
/// default; pass --stdout to preview instead.
///
/// With -r, positional arguments become name patterns that are
/// matched recursively against the working directory tree. A
/// positional that is an existing directory becomes the search
/// root.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Files, directories, or name patterns. When omitted, parfit
    /// reads standard input.
    #[arg(value_name = "PATH")]
    paths: Vec<String>,

    /// Target width in columns.
    #[arg(short = 'w', long, default_value_t = 68)]
    width: usize,

    /// Match positional arguments recursively against the current
    /// directory (or a directory given as a positional). Treats
    /// each non-directory positional as a glob-style name pattern.
    #[arg(short = 'r', long)]
    recursive: bool,

    /// Print the reformatted output to stdout instead of
    /// rewriting files. Default when reading from stdin; opt-in
    /// when path arguments are given.
    #[arg(long)]
    stdout: bool,

    /// Extra glob of file names to include, on top of any
    /// positional pattern. Repeatable.
    #[arg(long = "include", value_name = "GLOB")]
    includes: Vec<String>,

    /// Glob of file names to exclude. Repeatable.
    #[arg(long = "exclude", value_name = "GLOB")]
    excludes: Vec<String>,

    /// Language. When set, parfit reflows only comment blocks and
    /// leaves code untouched. Overrides auto-detection from the
    /// file extension. Use `text` to force plain-text paragraph
    /// reflow regardless of extension.
    #[arg(short = 'l', long = "lang", value_name = "NAME")]
    lang: Option<String>,

    /// Custom regex whose matching lines pass through verbatim.
    /// Repeatable.
    #[arg(short = 's', long = "skip", value_name = "REGEX")]
    skips: Vec<String>,

    /// Turn off the built-in directive skip list.
    #[arg(long)]
    no_default_skips: bool,

    /// Force a specific comment prefix instead of auto-detecting.
    #[arg(short = 'p', long, value_name = "STRING")]
    prefix: Option<String>,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let opts = build_options(&cli)?;

    if cli.paths.is_empty() && !cli.recursive {
        return run_stdin(&cli, &opts);
    }
    run_paths(&cli, &opts)
}

fn build_options(cli: &Cli) -> io::Result<Options> {
    let mut opts = Options::new(cli.width);
    if cli.no_default_skips {
        opts = opts.with_default_skips(false);
    }
    for pattern in &cli.skips {
        opts = opts
            .with_skip(pattern)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e.to_string()))?;
    }
    if let Some(prefix) = &cli.prefix {
        opts = opts.with_forced_prefix(prefix.clone());
    }
    Ok(opts)
}

fn run_stdin(cli: &Cli, opts: &Options) -> io::Result<()> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;
    let output = match lang_from_cli(cli)? {
        Some(lang) => reflow_source(&input, lang, opts),
        None => reflow(&input, opts),
    };
    io::stdout().write_all(output.as_bytes())
}

fn run_paths(cli: &Cli, opts: &Options) -> io::Result<()> {
    let (roots, mut patterns) = resolve_positionals(cli);
    patterns.extend(cli.includes.iter().cloned());

    let files = walker::walk(&roots, cli.recursive, &patterns, &cli.excludes)?;

    if files.is_empty() {
        return Ok(());
    }

    let forced_lang = lang_from_cli(cli)?;
    let mut stdout = io::stdout().lock();
    for path in &files {
        let content = fs::read_to_string(path)?;
        let lang = forced_lang.unwrap_or_else(|| Language::from_path(path));
        let wrapped = match lang {
            Language::Text => reflow(&content, opts),
            lang => reflow_source(&content, lang, opts),
        };
        if cli.stdout {
            stdout.write_all(wrapped.as_bytes())?;
        } else if content != wrapped {
            fs::write(path, wrapped)?;
        }
    }
    Ok(())
}

/// With -r, positional args split into (existing directories as
/// search roots, everything else as name patterns). Without -r,
/// positional args are literal file / directory paths.
fn resolve_positionals(cli: &Cli) -> (Vec<PathBuf>, Vec<String>) {
    if !cli.recursive {
        return (cli.paths.iter().map(PathBuf::from).collect(), Vec::new());
    }

    let mut roots = Vec::new();
    let mut patterns = Vec::new();
    for arg in &cli.paths {
        let p = Path::new(arg);
        if p.is_dir() {
            roots.push(p.to_path_buf());
        } else {
            patterns.push(arg.clone());
        }
    }
    if roots.is_empty() {
        roots.push(PathBuf::from("."));
    }
    (roots, patterns)
}

fn lang_from_cli(cli: &Cli) -> io::Result<Option<Language>> {
    match cli.lang.as_deref() {
        None => Ok(None),
        Some(name) => Language::from_str(name)
            .map(Some)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidInput, e)),
    }
}
