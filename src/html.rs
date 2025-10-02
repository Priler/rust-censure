use crate::lang::common::{PAT_HTML_TAG, PAT_HTML_SPACE};

#[derive(Debug, Clone)]
pub enum TokType { Word, Space, TagOpen, TagClose, TagSelf, Spacer }

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub tag: String,     // lowercased name for tags
    pub kind: TokType,
}

impl Token {
    pub fn new_tag(raw: &str) -> Self {
        let mut head = raw[1..raw.len()-1].to_lowercase(); // strip '<', '>'
        let kind = if head.starts_with('/') {
            head.remove(0);
            TokType::TagClose
        } else if raw.ends_with("/>") {
            TokType::TagSelf
        } else {
            TokType::TagOpen
        };

        let tag_name = head.split_whitespace().next().unwrap_or("").to_string();

        // spacer check (mirrors Python PAT_HTML_SPACE test)
        let kind = if matches!(kind, TokType::TagOpen|TokType::TagClose|TokType::TagSelf)
            && PAT_HTML_SPACE.is_match(raw)
        {
            TokType::Spacer
        } else { kind };

        Token { value: raw.to_string(), tag: tag_name, kind }
    }
}

/// Split an HTML line into alternating text and tags, then words/spaces inside text
pub fn tokenize_html(line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut start = 0usize;
    for m in PAT_HTML_TAG.find_iter(line) {
        let (s, e) = (m.start(), m.end());
        if s > start {
            let text = &line[start..s];
            split_text(text, &mut tokens);
        }
        tokens.push(Token::new_tag(&line[s..e]));
        start = e;
    }
    if start < line.len() {
        split_text(&line[start..], &mut tokens);
    }
    tokens
}

fn split_text(text: &str, out: &mut Vec<Token>) {
    // split by whitespace but keep spaces as separate tokens, like Python
    let mut last = 0usize;
    for m in regex::Regex::new(r"\s+").unwrap().find_iter(text) {
        let (s, e) = (m.start(), m.end());
        if s > last {
            out.push(Token { value: text[last..s].to_string(), tag: "".into(), kind: TokType::Word });
        }
        out.push(Token { value: text[s..e].to_string(), tag: "".into(), kind: TokType::Space });
        last = e;
    }
    if last < text.len() {
        out.push(Token { value: text[last..].to_string(), tag: "".into(), kind: TokType::Word });
    }
}
