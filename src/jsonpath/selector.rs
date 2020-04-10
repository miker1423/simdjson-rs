use std::fmt::Debug;

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedJsonPath {
    pub op: String,
    pub key: String,
    pub args: Vec<String>
}

#[derive(Debug, Clone, PartialEq)]
pub struct ParsedFilter {
    pub left: String,
    pub op: String,
    pub right: String
}

pub fn s(s: &str) -> String {
    s.to_string()
}

pub fn tokenize(query: String) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut token = String::new();

    for (i, c) in query.chars().enumerate() {
        token.push(c);
        match token.as_str() {
            "$" | "@" => {
                if i == 0 {
                    tokens.push(token);
                    token = String::new();
                } else {
                    return Err("JsonPath must start with $ (or @ or filters".to_owned());
                }
            }
            "." | "*" => continue,
            ".." => token = "*".to_owned(),
            _ => {
                if let Some(bracket_idx) = token.find("[") {
                    if c == ']' {
                        if token.starts_with("."){
                            tokens.push(token[1..bracket_idx].to_string());
                            tokens.push(token[bracket_idx..].to_string());
                        }
                    } else {
                        tokens.push(token[..bracket_idx].to_string());
                        tokens.push(token[bracket_idx..].to_string());
                    }
                    token = String::new();
                }
                else {
                    if c == '.' {
                        if token.starts_with("."){
                            tokens.push(token[1..token.len() - 1].to_string())
                        } else {
                            tokens.push(token[..token.len() - 1].to_string())
                        }
                        token = String::new();
                        continue;
                    }
                }
            }
        }
    }
    if token.len() > 0 {
        if token.starts_with(".") {
            token = token[1..].to_string();
        }
        if token != "*"  || tokens[tokens.len() - 1] != "*" {
            tokens.push(token);
        }
    }

    Ok(tokens)
}