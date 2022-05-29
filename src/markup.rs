use super::*;
use desumark::{Error, Layout};
use std::fmt::Display;

pub struct Parser(pub Vec<(Regex, String)>);

impl Default for Parser {
    fn default() -> Self {
        Self {
            0: vec![
                // (Regex::new("\\[textwall=(.*?)\\](.*?)\\[/textwall\\]").unwrap(), "<div class=\"textwall\"><div class=\"header\">$1</div><div class=\"content\">$2</div></div>".into()),
                // (Regex::new("\\[b\\](.*?)\\[/b\\]").unwrap(), "<strong>$1</strong>".into()),
                // (Regex::new("\\[a\\](.*?)\\[/a\\]").unwrap(), "<a href=\"$1\">$1</a>".into()),
                (
                    Regex::new(r"(http|https)://([\w_-]+(?:(?:\.[\w_-]+)+))([\w.,@?^=%&amp;:/~+#-]*[\w@?^=%&amp;/~+#-])?").unwrap(),
                    "<a href=\"$0\" class=\"link\">$0</a>".into()
                ),
                (
                    Regex::new("&gt;&gt;/([0-9a-zA-Z]+)/([0-9]+)").unwrap(),
                    "<span class=\"reply\" data-short=\"$1\" data-num=\"$2\">&gt;&gt;/$1/$2</span>".into(),
                ),
                (
                    Regex::new("&gt;&gt;/([0-9a-zA-Z]+)/([0-9a-zA-Z]+)/([0-9]+)").unwrap(),
                    "<span class=\"reply\" data-lang=\"$1\" data-short=\"$2\" data-num=\"$3\">&gt;&gt;/$1/$2/$3</span>".into(),
                ),
                (
                    Regex::new("&gt;&gt;([0-9a-zA-Z]+)/([0-9]+)").unwrap(),
                    "<span class=\"reply\" data-short=\"$1\" data-num=\"$2\">&gt;&gt;/$1/$2</span>".into(),
                ),
                (
                    Regex::new("&gt;&gt;([0-9a-zA-Z]+)/([0-9a-zA-Z]+)/([0-9]+)").unwrap(),
                    "<span class=\"reply\" data-lang=\"$1\" data-short=\"$2\" data-num=\"$3\">&gt;&gt;/$1/$2/$3</span>".into(),
                ),
                (
                    Regex::new("&gt;&gt;([0-9]+)").unwrap(),
                    "<span class=\"reply\" data-num=\"$1\">$0</span>".into(),
                ),
                // (Regex::new("\\$\\{((.|\n)*)\\}\\{((.|\n)*)\\}").unwrap(), "<div class=\"quote\"><div class=\"green\">$1</div><div class=\"red\">$3</div></div>".into()),
                (
                    Regex::new(r"(?m)^&gt;(.*)$").unwrap(),
                    "<div class=\"quote\"><div class=\"green\">$1</div></div>".into(),
                )
            ],
        }
    }
}

impl Parser {
    pub fn parse<E: Display>(&self, source: &str, layout: &Layout<E>) -> Result<String, Error<E>> {
        layout.proccess(
            &self
                .0
                .iter()
                .fold(source.to_string(), |before, (rule, to)| {
                    rule.replace_all(&before, to.as_str()).to_string()
                }),
        )
    }
}
