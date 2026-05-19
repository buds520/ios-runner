use std::io::{self, Write};

use anyhow::{Context, Result, bail};

use crate::locale::t;

/// Numbered menu on stderr; returns selected index.
pub fn pick_one(title: &str, options: &[String]) -> Result<usize> {
    if options.is_empty() {
        bail!("{}", t("没有可选项", "No options available"));
    }
    if options.len() == 1 {
        eprintln!(
            "{}",
            crate::locale::tf(
                || format!("{title}：{}（仅此一项）", options[0]),
                || format!("{title}: {} (only option)", options[0]),
            )
        );
        return Ok(0);
    }

    eprintln!();
    eprintln!("━━ {title} ━━");
    for (i, opt) in options.iter().enumerate() {
        eprintln!("  {:>2}. {}", i + 1, opt);
    }
    eprint!(
        "{}",
        crate::locale::tf(
            || format!("请输入编号 [1-{}]：", options.len()),
            || format!("Enter number [1-{}]: ", options.len()),
        )
    );
    io::stderr().flush().ok();

    let mut line = String::new();
    io::stdin()
        .read_line(&mut line)
        .context("read selection")?;
    let choice: usize = line
        .trim()
        .parse()
        .with_context(|| {
            crate::locale::tf(
                || format!("无效编号: {line:?}"),
                || format!("Invalid number: {line:?}"),
            )
        })?;

    if choice == 0 || choice > options.len() {
        bail!("{}", t("编号超出范围", "Number out of range"));
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
