use rlex::{self, Rlex, DefaultState, DefaultToken};

pub fn validate_token_html_backslash_count(tag_str: &str) -> Result<(), String> {
    let mut r: Rlex<DefaultState, DefaultToken> = Rlex::new(tag_str, DefaultState::Default);
    let mut count = 0;
    while !r.at_end() {
        if r.char() == '/' {
            if !r.is_in_quote() {
                count += 1;
            }
        }
        r.next();
    }
    if count > 1 {
        return Err(format!("ERR_HTML_FORMAT: the following tag has more than 1 '/' character outside of quotes: {}", tag_str));
    }
    return Ok(());
}

pub fn validate_token_html_quotes(tag_str: &str) -> Result<(), String> {
    let mut r: Rlex<DefaultState, DefaultToken> = Rlex::new(tag_str, DefaultState::Default);
    r.trace_on();
    let mut collect: Vec<String> = vec![];
    while !r.at_end() {
        let start_char = r.char();
        if start_char == '\'' {
            let start = r.pos();
            r.next();
            r.next_until('\'');
            let pos = r.pos();
            let str = r.str_from_rng(start, pos);
            collect.push(str.to_string());
        }
        if start_char == '"' {
            let start = r.pos();
            r.next();
            r.next_until('"');
            let pos = r.pos();
            let str = r.str_from_rng(start, pos);
            collect.push(str.to_string());
        }
        r.next();
    }
    if collect.contains(&"''".to_string()) || collect.contains(&"\"\"".to_string()) {
        let mut single_count = 0;
        let mut double_count = 0;
        for s in &collect {
            if s == "''" {
                single_count+=1;
            }
            if s == "\"\"" {
                double_count+=1;
            }
        }
        if double_count > 1 || single_count > 1 {
            return Err(format!("ERR_HTML_FORMAT: quotation mark error -> {}", tag_str));
        }
    }
    let mut src = r.src().to_string();
    for s in collect {
        // let squeezed = s.replace(" ", "");
        // if squeezed == "\"\"" || squeezed == "''" {
        //     return Err(format!("ERR_HTML_FORMAT: the following tag makes poor use of quotes has extract quotes: {}", tag_str));
        // }
        let mut chars = s.chars();
        let first_char = match chars.nth(0) {
            Some(c) => {c},
            None => {
                continue;
            }
        };
        let last_char = match chars.last() {
            Some(c) => {c},
            None => {
                continue;
            }
        };
        if first_char != last_char {
            return Err(format!("ERR_HTML_FORMAT: the following tag makes poor use of quotes and is missing a closing quote: {}", tag_str));
        }
        src = src.replace(&s, "");
    }
    if src.contains("'") || src.contains("\"") {
        return Err(format!("ERR_HTML_FORMAT: the following tag makes poor use of quotes and is malformed: {}", tag_str));
    }
    return Ok(());
}


pub fn html_tag_name(tag: &str) -> Result<String, String> {
    // trimming whitespace and ensuring we have an input longer than 3 chars
    let tag = tag.trim();
    if tag.len() < 3 {
        return Err(format!("{} is less than 3 chars, valid html tags must be 3 or more chars", tag))
    }
    // checking the first and last characters to ensure they are '<' and '>'
    let mut chars = tag.chars();
    let first_char = chars.next().unwrap(); // cannot fail
    let last_char = chars.last().unwrap(); // cannot fail
    if first_char != '<' || last_char != '>' {
        return Err(format!("{} does not start with a '<' and end with a '>' which is required for html tags", tag))
    }
    // removing the outer '<' and '>' and splitting by whitespace and getting our iterator
    let cleaned = tag.replace('<', "").replace('>', "");
    let parts = cleaned.split_whitespace();
    let mut parts_iter = parts.into_iter();
    // if the first part is a '/' then we are dealing with a closing tag
    let first_part = match parts_iter.next() {
        Some(part) => { part },
        None => {
            return Err(format!("{} failed to split this by whitespace and access the first 'part'", tag))
        },
    };
    // this will represent the 'part' of the whitespace split we are returning
    let target_part: &str;
    if first_part == "/" {
        let second_part = match parts_iter.next() {
            Some(part) => { part },
            None => {
                return Err(format!("{} failed to split this by whitespace and access the second 'part'", tag))
            },
        };
        target_part = second_part;
    } else {
        target_part = first_part;
    }
    // trimming off the '/' from the target_part if needed
    if target_part.starts_with('/') || target_part.ends_with('/') {
        let target_part = &target_part.replace('/', "");
        return Ok(target_part.to_string().to_lowercase());
    }
    return Ok(target_part.to_string().to_lowercase());
}