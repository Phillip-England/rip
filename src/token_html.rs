
use crate::html_utils::*;
use rlex::{self, Rlex};


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

pub fn html_tokenize(source: &str) -> Result<Vec<TokenHtml>, String> {
    let mut r: Rlex<LexerState, TokenHtml> = Rlex::new(source, LexerState::InTag);
    r.trace_on();
    while !r.at_end() {
        match r.state() {
            LexerState::InTag => {
                let toks = handle_in_tag(&mut r)?;
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

fn handle_in_tag(r: &mut Rlex<LexerState, TokenHtml>) -> Result<Option<Vec<TokenHtml>>, String> {
    let start = r.pos();
    while !r.at_end() {
        if r.char() == '>' {
            if !r.is_in_quote() {
                break;
            }
        }
        r.next();
    }
    let pos = r.pos();
    let tag_str = r.str_from_rng(start, pos).to_owned();
    // important! stepping off the '>' and into the next section
    r.next();
    let tok = new_token_html_from_tag(&tag_str)?;
    match tok {
        TokenHtml::Open { tag_name: _, outer_html: _ } => {
            if r.char() != '<' {
                r.state_set(LexerState::InText);
            }
        },
        TokenHtml::Close { tag_name: _, outer_html: _ } => {
            r.state_set(LexerState::InText);
        },
        TokenHtml::PreLikeOpen { tag_name: _, outer_html: _ } => {
            r.state_set(LexerState::InPreLike);
        },
        TokenHtml::PreLikeClose { tag_name: _, outer_html: _ } => {
            r.state_set(LexerState::InTag);
        },
        TokenHtml::SelfClosing { tag_name: _, outer_html: _ } => {
            r.state_set(LexerState::InText);
        },
        _ => {
            return Err(format!("ERR_HTML_FORMAT: derived a TokenHtml::WhiteSpace or TokenHtml::InnerText from new_token_html_from_tag, which is not possible"));
        }
    }
    return Ok(Some(vec![tok]));
}

fn handle_in_text(r: &mut Rlex<LexerState, TokenHtml>) -> Result<Option<Vec<TokenHtml>>, String> {
    let start = r.pos();
    r.next_until('<');
    r.prev();
    r.state_set(LexerState::InTag);
    if r.char() == '>' {
        r.next();
        return Ok(None);
    }
    let pos = r.pos();
    let tag_text = r.str_from_rng(start, pos).to_owned();
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
            return Err("ERR_HTML_FORMAT: ended up inside of a prelike token without a reference token to peek back on {}".to_string());
        }
    };
    let (tag_name, _outer_html) = match prev_tok {
        TokenHtml::PreLikeOpen { tag_name, outer_html } => ( tag_name, outer_html ),
        _ => {
            return Err(format!("ERR_HTML_FORMAT: expected the previous token to be prelike {:?}", prev_tok));
        }
    };
    let tag_name_ref = &tag_name;
    // we need to search for the next closing tag that matches the prev tag (which is pre-like)
    let text_start = r.pos();
    if r.char() != '<' {
        while !r.at_end() {
            if r.char() == '<' {
                if !r.is_in_quote() {
                    break;
                }
            }
            r.next();
        }
    }
    let original_start = r.pos();
    let mut reset_count = 0;
    while !r.at_end() {
        let mut inner_count = 0;
        while inner_count < reset_count {
            r.next();
            r.next_until('<');
            inner_count += 1;
        }
        let close_tag_start = r.pos();
        r.next_until('>');
        r.state_set(LexerState::InTag);
        let pos = r.pos();
        r.next();
        let close_tag = r.str_from_rng(close_tag_start, pos);
        let close_tag_condensed = close_tag.replace(' ', "");
        if close_tag_condensed != format!("</{}>", tag_name_ref) {
            reset_count+=1;
            r.goto_pos(original_start);
            continue;
        }
        let prelike_text = r.str_from_rng(text_start, close_tag_start-1);
        let close_tok = TokenHtml::PreLikeClose { tag_name: tag_name_ref.clone(), outer_html: close_tag.to_string() };
        if prelike_text.replace(' ', "").len() == 0 {
            return Ok(Some(vec![TokenHtml::Whitespace { text: prelike_text.to_string() }, close_tok]));
        }
        return Ok(Some(vec![TokenHtml::InnerText { text: prelike_text.to_string() } , close_tok]));
    }
    // we should exit in the loop because we MUST find a closing prelike tag
    return Err(format!("ERR_HTML_FORMAT: failed to find a closing tag for "));
}
