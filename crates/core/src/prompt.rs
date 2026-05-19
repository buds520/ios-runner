use std::io::{self, Write};

use anyhow::{Context, Result, bail};

/// Numbered menu on stderr; returns selected index.
pub fn pick_one(title: &str, options: &[String]) -> Result<usize> {
    if options.is_empty() {
        bail!("no options to choose from");
    }
    if options.len() == 1 {
        eprintln!("{title}: {} (only option)", options[0]);
        return Ok(0);
    }

    eprintln!("\n{title}");
    for (i, opt) in options.iter().enumerate() {
        eprintln!("  {}. {}", i + 1, opt);
    }
    eprint!("Enter number [1-{}]: ", options.len());
    io::stderr().flush().ok();

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .context("read selection")?;
    let choice: usize = line
        .trim()
        .parse()
        .with_context(|| format!("invalid number: {line:?}"))?;

    if choice == 0 || choice > options.len() {
        bail!("selection out of range");
    }
    Ok(choice - 1)
}
