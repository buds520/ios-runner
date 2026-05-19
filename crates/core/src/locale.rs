use std::cell::Cell;
use std::path::Path;

use crate::config::RunnerConfig;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Lang {
    ZhCn,
    En,
}

impl Lang {
    pub fn parse(s: &str) -> Self {
        let s = s.trim();
        if s.eq_ignore_ascii_case("en") || s.starts_with("en-") {
            Lang::En
        } else {
            Lang::ZhCn
        }
    }

    pub fn as_config_str(self) -> &'static str {
        match self {
            Lang::ZhCn => "zh-CN",
            Lang::En => "en",
        }
    }
}

thread_local! {
    static CURRENT: Cell<Lang> = const { Cell::new(Lang::ZhCn) };
}

pub fn set_lang(lang: Lang) {
    CURRENT.with(|c| c.set(lang));
}

pub fn lang() -> Lang {
    CURRENT.with(|c| c.get())
}

/// `IOS_RUNNER_LANG` overrides `.ios-runner.toml` when both are present.
pub fn init_locale(root: Option<&Path>) {
    if let Ok(v) = std::env::var("IOS_RUNNER_LANG") {
        set_lang(Lang::parse(&v));
        return;
    }
    if let Some(root) = root {
        if let Ok(config) = RunnerConfig::load(root) {
            set_lang(Lang::parse(&config.language));
        }
    }
}

pub fn t(zh: &'static str, en: &'static str) -> &'static str {
    match lang() {
        Lang::ZhCn => zh,
        Lang::En => en,
    }
}

/// Macro-friendly format when one side needs `format!`.
pub fn tf(zh: impl FnOnce() -> String, en: impl FnOnce() -> String) -> String {
    match lang() {
        Lang::ZhCn => zh(),
        Lang::En => en(),
    }
}
