use std::io::{self, Write};

use anyhow::{Context, Result, bail};

/// Numbered menu on stderr; returns selected index.
pub fn pick_one(title: &str, options: &[String]) -> Result<usize> {
    if options.is_empty() {
        bail!("没有可选项");
    }
    if options.len() == 1 {
        eprintln!("{title}：{}（仅此一项）", options[0]);
        return Ok(0);
    }

    eprintln!();
    eprintln!("━━ {title} ━━");
    for (i, opt) in options.iter().enumerate() {
        eprintln!("  {:>2}. {}", i + 1, opt);
    }
    eprint!("请输入编号 [1-{}]：", options.len());
    io::stderr().flush().ok();

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .context("read selection")?;
    let choice: usize = line
        .trim()
        .parse()
        .with_context(|| format!("无效编号: {line:?}"))?;

    if choice == 0 || choice > options.len() {
        bail!("编号超出范围");
    }
    Ok(choice - 1)
}

/// Ask yes/no on stderr; `default_yes` used when user presses Enter on empty line.
pub fn confirm(prompt: &str, default_yes: bool) -> Result<bool> {
    let hint = if default_yes { "Y/n" } else { "y/N" };
    eprint!("{prompt} [{hint}]：");
    io::stderr().flush().ok();

    let mut line = String::new();
    io::stdin().read_line(&mut line).context("read confirm")?;
    let answer = line.trim().to_lowercase();
    Ok(match answer.as_str() {
        "" => default_yes,
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => default_yes,
    })
}
