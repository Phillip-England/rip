use rlex::{self, Rlex, DefaultState, DefaultToken};

#[derive(Debug, PartialEq, Eq)]
enum LexerState {
    InTag,
    InText,
    InPreLike,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenHtml {
    Open { tag_name: String, outer_html: String },
    Close { tag_name: String, outer_html: String },
    SelfClosing { tag_name: String, outer_html: String },
    PreLikeOpen { tag_name: String, outer_html: String },
    PreLikeClose { tag_name: String, outer_html: String },
    InnerText { text: String },
    Whitespace { text: String },
}

fn validate_token_html_backslash_count(tag_str: &str) -> Result<(), String> {
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

fn validate_token_html_quotes(tag_str: &str) -> Result<(), String> {
    let mut r: Rlex<DefaultState, DefaultToken> = Rlex::new(tag_str, DefaultState::Default);
    let mut collect: Vec<String> = vec![];
    while !r.at_end() {
        let start_char = r.char();
        if start_char == '\'' {
            let start = r.pos();
            r.next();
            r.next_until('\'');
            let str = r.str_from_rng(start, r.pos());
            collect.push(str.to_string());
        }
        if start_char == '"' {
            let start = r.pos();
            r.next();
            r.next_until('"');
            let str = r.str_from_rng(start, r.pos());
            collect.push(str.to_string());
        }
        r.next();
    }
    let mut src = r.src().to_string();
    for s in collect {
        let squeezed = s.replace(" ", "");
        if squeezed == "\"\"" || squeezed == "''" {
            return Err(format!("ERR_HTML_FORMAT: the following tag makes poor use of quotes has extract quotes: {}", tag_str));
        }
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
    println!("{}", src);
    if src.contains("'") || src.contains("\"") {
        return Err(format!("ERR_HTML_FORMAT: the following tag makes poor use of quotes and is malformed: {}", tag_str));
    }
    return Ok(());
}

pub fn new_token_html_from_tag(tag_str: &str) -> Result<TokenHtml, String> {
    validate_token_html_backslash_count(tag_str)?;
    validate_token_html_quotes(tag_str)?;
    let tag_name = html_tag_name(tag_str)?;
    let format_breaking_tag_names = vec!["script".to_owned(), "style".to_owned(), "textarea".to_owned(), "xmp".to_owned(), "pre".to_owned()];     
    let mut is_format_breaking = false;
    if format_breaking_tag_names.contains(&tag_name) {
        is_format_breaking = true;
    } 
    let tag_str_squeezed = &tag_str.replace(' ', "").clone();
    let mut chars = tag_str_squeezed.chars();
    let second_char = chars.nth(1);
    if second_char == Some('/') {
        if is_format_breaking {
            return Ok(TokenHtml::PreLikeClose { tag_name: tag_name, outer_html: tag_str.to_string() });
        }
        return Ok(TokenHtml::Close { tag_name: tag_name, outer_html: tag_str.to_string() });
    }
    let second_to_last_char = chars.rev().nth(1);
    if second_to_last_char == Some('/') {
        return Ok(TokenHtml::SelfClosing { tag_name: tag_name, outer_html: tag_str.to_string() });
    }
    if is_format_breaking {
        return Ok(TokenHtml::PreLikeOpen { tag_name: tag_name, outer_html: tag_str.to_string() });
    }
    return Ok(TokenHtml::Open { tag_name: tag_name, outer_html: tag_str.to_string() });
}


fn html_tag_name(tag: &str) -> Result<String, String> {
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



pub fn tokenize(source: &str) -> Result<Vec<TokenHtml>, String> {
    let mut r: Rlex<LexerState, TokenHtml> = Rlex::new(source, LexerState::InTag);
    let format_breaking_tag_names = vec!["script".to_owned(), "style".to_owned(), "textarea".to_owned(), "xmp".to_owned(), "pre".to_owned()];      
    while !r.at_end() {
        match r.state() {
            LexerState::InTag => {
                let toks = handle_in_tag(&mut r, &format_breaking_tag_names)?;
                match toks {
                    Some(toks) => {
                        for tok in toks {
                            r.token_push(tok);
                        }
                    },
                    None => {
                        continue;
                    }
                };
            },
            LexerState::InText => {
                let toks = handle_in_text(&mut r)?;
                match toks {
                    Some(toks) => {
                        for tok in toks {
                            r.token_push(tok);
                        }
                    },
                    None => {
                        continue;
                    }
                };
            },
            LexerState::InPreLike => {
                let toks = handle_in_pre_like(&mut r)?;
                match toks {
                    Some(toks) => {
                        for tok in toks {
                            r.token_push(tok);
                        }
                    },
                    None => {
                        continue;
                    }
                };
            },
        };
    }
    let toks = r.token_consume();
    return Ok(toks);
}

fn handle_in_tag(r: &mut Rlex<LexerState, TokenHtml>, format_breaking_tag_names: &Vec<String>) -> Result<Option<Vec<TokenHtml>>, String> {
    let start = r.pos();
    while !r.at_end() {
        r.next_until('>');
        if r.is_in_quote() {
            continue;
        }
        break;
    }
    let tag_str = r.str_from_rng(start, r.pos()).to_owned();
    let tag_name = html_tag_name(&tag_str)?;
    // important! stepping off the '>' and into the next section
    r.next();
    if format_breaking_tag_names.contains(&tag_name) {
        r.state_set(LexerState::InPreLike);
        return Ok(Some(vec![TokenHtml::PreLikeOpen { tag_name: tag_name, outer_html: tag_str }]));
    }
    // checking if we instantly stepped into another tag, keep the same state
    if r.char() == '<' {
        return Ok(Some(vec![TokenHtml::Open { tag_name: tag_name, outer_html: tag_str }]));
    }
    // if we didn't hit a '<', we are in a tags text
    r.state_set(LexerState::InText);
    return Ok(Some(vec![TokenHtml::Open { tag_name: tag_name, outer_html: tag_str }]));
}

fn handle_in_text(r: &mut Rlex<LexerState, TokenHtml>) -> Result<Option<Vec<TokenHtml>>, String> {
    // walking until we hit a '<' (since we are in html text, it doens't matter if it is wrapped in quotes or not)
    let start = r.pos();
    r.next_until('<');
    // important! stepping off the '<' and into the last char of the html tag text
    r.prev();
    // if our current char is '>' then we know tags have stacked together like <h1><p> so no need to collect
    r.state_set(LexerState::InTag);
    if r.char() == '>' {
        // important! stepping back onto the '<' so we are at the start of the next tag
        r.next();
        return Ok(None);
    }
    // important! stepping back onto the '<' so we are at the start of the next tag
    let tag_text = r.str_from_rng(start, r.pos()).to_owned();
    r.next();
    if tag_text.replace(" ", "").len() == 0 {
        return Ok(Some(vec![TokenHtml::Whitespace { text: tag_text }]))
    }
    return Ok(Some(vec![TokenHtml::InnerText { text: tag_text }]));
}

fn handle_in_pre_like(r: &mut Rlex<LexerState, TokenHtml>) -> Result<Option<Vec<TokenHtml>>, String> {
    let prev_tok = match r.token_prev().cloned() {
        Some(tok) => { tok },
        None => {
            return Err("ERR_HTML_FORMAT: ended up inside of a prelike token without a reference token to peek back on".to_string());
        }
    };
    let (tag_name, _outer_html) = match prev_tok {
        TokenHtml::PreLikeClose { tag_name, outer_html } => ( tag_name, outer_html ),
        _ => {
            return Err("ERR_HTML_FORMAT: expected the previous token to be prelike".to_string());
        }
    };
    let tag_name_ref = &tag_name;
    // we need to search for the next closing tag that matches the prev tag (which is pre-like)
    let text_start = r.pos();
    while !r.at_end() {
        // keep walking until we find a '<' (it doesn't matter if it is quoted or not)
        // it may close directly after like <script></script>
        if r.char() != '<' {
            r.next_until('<');
        }
        let close_tag_start = r.pos();
        r.next_until('>');
        let close_tag = r.str_from_rng(close_tag_start, r.pos());
        let close_tag_condensed = close_tag.replace(' ', "");
        // if we dont find the close tag, continue
        if close_tag_condensed != format!("</{}>", tag_name_ref) {
            r.goto_pos(close_tag_start);
            continue;
        }
        // we found the close tag
        let prelike_text = r.str_from_rng(text_start, close_tag_start);
        let close_tok = TokenHtml::Close { tag_name: tag_name_ref.clone(), outer_html: close_tag.to_string() };
        // if the prelike text only contains spaces, then it needs to be treated like whitespace
        if prelike_text.replace(' ', "").len() == 0 {
            return Ok(Some(vec![TokenHtml::Whitespace { text: prelike_text.to_string() }, close_tok]));
        }
        return Ok(Some(vec![close_tok]));
    }
    // we should exit in the loop because we MUST find a closing prelike tag
    return Err("ERR_HTML_FORMAT: failed to find a closing tag for ".to_string());
}



#[test]
fn test_tokenize() {
    // let toks = tokenize("<h1>Hello, World!</h1>").unwrap();
    // println!("{:?}", toks);
    // assert!(toks == vec![
    //     TokenHtml::Open { tag_name: "h1".to_string(), outer_html: "<h1>".to_string() }, 
    //     TokenHtml::InnerText { text : "Hello, World!".to_string() },
    //     TokenHtml::Close { tag_name: "h1".to_string(), outer_html: "</h1>".to_string() },
    // ]);


    // assert!(tokenize("<h1><p>Hello, World!</p></h1>").unwrap() == vec!["<h1>", "<p>",  "Hello, World!", "</p>", "</h1>"]);
    // assert!(tokenize("<h1><p>Hello, World!</p>    </h1>").unwrap() == vec!["<h1>", "<p>",  "Hello, World!", "</p>", "    ", "</h1>"]);
    // assert!(tokenize("<div><span>Text</span></div>").unwrap() == vec!["<div>", "<span>", "Text", "</span>", "</div>"]);
    // assert!(tokenize("<div>  <span>Text</span>  </div>").unwrap() == vec!["<div>", "  ", "<span>", "Text", "</span>", "  ", "</div>"]);
    // assert!(tokenize("<ul><li>Item 1</li><li>Item 2</li></ul>").unwrap() == vec!["<ul>", "<li>", "Item 1", "</li>", "<li>", "Item 2", "</li>", "</ul>"]);
    // assert!(tokenize("<a href=\"#\">Link</a>").unwrap() == vec!["<a href=\"#\">", "Link", "</a>"]);
    // assert!(tokenize("<input type=\"text\"/>").unwrap() == vec!["<input type=\"text\"/>"]);
    // assert!(tokenize("<br/>").unwrap() == vec!["<br/>"]);
    // assert!(tokenize("<p>Hello<br/>World</p>").unwrap() == vec!["<p>", "Hello", "<br/>", "World", "</p>"]);
    // assert!(tokenize("<section><h2>Title</h2><p>Body</p></section>").unwrap() == vec!["<section>", "<h2>", "Title", "</h2>", "<p>", "Body", "</p>", "</section>"]);
}


// #[test]
// fn test_new_token_html_tag() {
//     assert!(new_token_html_from_tag("<div>").unwrap() == TokenHtml::Open { tag_name: "div".to_string(), outer_html: "<div>".to_string() });
//     assert!(new_token_html_from_tag("<DIV>").unwrap() == TokenHtml::Open { tag_name: "div".to_string(), outer_html: "<DIV>".to_string() });
//     assert!(new_token_html_from_tag("<div class='x'>").unwrap() == TokenHtml::Open { tag_name: "div".to_string(), outer_html: "<div class='x'>".to_string() });
//     assert!(new_token_html_from_tag("</div>").unwrap() == TokenHtml::Close { tag_name: "div".to_string(), outer_html: "</div>".to_string() });
//     assert!(new_token_html_from_tag("<br/>").unwrap() == TokenHtml::SelfClosing { tag_name: "br".to_string(), outer_html: "<br/>".to_string() });
//     assert!(new_token_html_from_tag("<br />").unwrap() == TokenHtml::SelfClosing { tag_name: "br".to_string(), outer_html: "<br />".to_string() });
//     assert!(new_token_html_from_tag("<input type='text'/>").unwrap() == TokenHtml::SelfClosing { tag_name: "input".to_string(), outer_html: "<input type='text'/>".to_string() });
//     assert!(new_token_html_from_tag("<hr//>").is_err());
//     assert!(new_token_html_from_tag("<script>").unwrap() == TokenHtml::PreLikeOpen { tag_name: "script".to_string(), outer_html: "<script>".to_string() });
//     assert!(new_token_html_from_tag("</script>").unwrap() == TokenHtml::PreLikeClose { tag_name: "script".to_string(), outer_html: "</script>".to_string() });
//     assert!(new_token_html_from_tag("<style>").unwrap() == TokenHtml::PreLikeOpen { tag_name: "style".to_string(), outer_html: "<style>".to_string() });
//     assert!(new_token_html_from_tag("</style>").unwrap() == TokenHtml::PreLikeClose { tag_name: "style".to_string(), outer_html: "</style>".to_string() });
//     assert!(new_token_html_from_tag("<pre>").unwrap() == TokenHtml::PreLikeOpen { tag_name: "pre".to_string(), outer_html: "<pre>".to_string() });
//     assert!(new_token_html_from_tag("</pre>").unwrap() == TokenHtml::PreLikeClose { tag_name: "pre".to_string(), outer_html: "</pre>".to_string() });
//     assert!(new_token_html_from_tag("<textarea>").unwrap() == TokenHtml::PreLikeOpen { tag_name: "textarea".to_string(), outer_html: "<textarea>".to_string() });
//     assert!(new_token_html_from_tag("</textarea>").unwrap() == TokenHtml::PreLikeClose { tag_name: "textarea".to_string(), outer_html: "</textarea>".to_string() });
//     assert!(new_token_html_from_tag("<xmp>").unwrap() == TokenHtml::PreLikeOpen { tag_name: "xmp".to_string(), outer_html: "<xmp>".to_string() });
//     assert!(new_token_html_from_tag("</xmp>").unwrap() == TokenHtml::PreLikeClose { tag_name: "xmp".to_string(), outer_html: "</xmp>".to_string() });
// }

#[test]
fn test_validate_token_backslash_count() {
    assert!(validate_token_html_backslash_count("<h1>").is_ok());
    assert!(validate_token_html_backslash_count("<h1//>").is_err());
    assert!(validate_token_html_backslash_count("<////h1//>").is_err());
    assert!(validate_token_html_backslash_count("<h1 attr='//////////'/>").is_ok());

}

#[test]
fn test_validate_token_html_quotes_more() {
    assert!(validate_token_html_quotes(r#"<div data-attr="hello">"#).is_ok());
    assert!(validate_token_html_quotes(r#"<div data-attr='hello'>"#).is_ok());
    assert!(validate_token_html_quotes(r#"<div data-attr="'quoted'">"#).is_ok());
    assert!(validate_token_html_quotes(r#"<div data-attr='"quoted"'>"#).is_ok());
    assert!(validate_token_html_quotes(r#"<div data-attr='"inner"stuff"'>"#).is_ok());
    assert!(validate_token_html_quotes(r#"<div data-attr="'inner'stuff'">"#).is_ok());
    assert!(validate_token_html_quotes(r#"<input value='"He said, \"Hello\""'/>"#).is_ok());
    assert!(validate_token_html_quotes(r#"<input value="'She said, 'Hi''"/>"#).is_ok());
    assert!(validate_token_html_quotes(r#"<img alt='"a "complex" alt"'>"#).is_ok());
    assert!(validate_token_html_quotes(r#"<div data-attr='missing end>"#).is_err());
    assert!(validate_token_html_quotes(r#"<div data-attr="missing end>"#).is_err());
    assert!(validate_token_html_quotes(r#"<div class="foo" bar='baz'>"#).is_ok());
    assert!(validate_token_html_quotes(r#"<div class="foo" bar='baz>"#).is_err());
    assert!(validate_token_html_quotes(r#"<path d='M10 10 H90 V90 H10 Z'/>"#).is_ok());
    assert!(validate_token_html_quotes(r#"<path d="M10 10 H90 V90 H10 Z"/>"#).is_ok());
    assert!(validate_token_html_quotes(r#"<div class="foo"""""""""""""""""""""""""""""""""">"#).is_err());
    assert!(validate_token_html_quotes(r#"<div class='foo'''''''''''''''''''''''''''''''>"#).is_err());
    assert!(validate_token_html_quotes(r#"<div attr='he said \"hi\"'>"#).is_ok()); // no escape semantics in HTML attr
    assert!(validate_token_html_quotes(r#"<div attr="it\'s fine">"#).is_ok());
    assert!(validate_token_html_quotes(r#"<div attr="''">"#).is_ok());
    assert!(validate_token_html_quotes(r#"<div attr='""'>"#).is_ok());
}


#[test]
fn test_html_tag_name() {
    assert!(html_tag_name("<h1>").unwrap() == "h1");
    assert!(html_tag_name(r#"<h1 someAttr="double quotes">"#).unwrap() == "h1");
    assert!(html_tag_name(r#"<h1 someAttr='single quotes'>"#).unwrap() == "h1");
    assert!(html_tag_name(r#"<    h1>"#).unwrap() == "h1");
    assert!(html_tag_name(r#"<h1         >"#).unwrap() == "h1");
    assert!(html_tag_name(r#"<input/>"#).unwrap() == "input");
    assert!(html_tag_name(r#"<     input/>"#).unwrap() == "input");
    assert!(html_tag_name(r#"<     input     />"#).unwrap() == "input");
    assert!(html_tag_name(r#"<>"#).is_err());
    assert!(html_tag_name("<div>").unwrap() == "div");
    assert!(html_tag_name("<br/>").unwrap() == "br");
    assert!(html_tag_name("<DIV>").unwrap() == "div");
    assert!(html_tag_name("<img />").unwrap() == "img");
    assert!(html_tag_name(r#"<img src="foo/bar.png" alt='an "img"'>"#).unwrap() == "img");
    assert!(html_tag_name("   <   section     class='x'>").unwrap() == "section");
    assert!(html_tag_name("<     >").is_err());
    assert!(html_tag_name("h1").is_err());
    assert!(html_tag_name(r#"<meta http-equiv="X-UA-Compatible"/>"#).unwrap() == "meta");
    assert!(html_tag_name(r#"<path d="M10 10 H 90 V 90 H 10 Z"/>"#).unwrap() == "path");
    assert!(html_tag_name(r#"<hr//>"#).unwrap() == "hr");
    assert!(html_tag_name(r#"<h2 class="intro>"#).unwrap() == "h2");
    assert!(html_tag_name("").is_err());
    assert!(html_tag_name("<h1").is_err());
    assert!(html_tag_name("h1>").is_err());
    assert!(html_tag_name("</h1>").unwrap() == "h1");
    assert!(html_tag_name("<    /h1>").unwrap() == "h1");
    assert!(html_tag_name("<    /   h1   >").unwrap() == "h1");
    assert!(html_tag_name("</   h1>").unwrap() == "h1");
}
