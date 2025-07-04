use crate::*;

#[test]
fn test_new_token_html_tag() {
    assert!(new_token_html_from_tag("<div>").unwrap() == TokenHtml::Open { tag_name: "div".to_string(), outer_html: "<div>".to_string() });
    assert!(new_token_html_from_tag("<DIV>").unwrap() == TokenHtml::Open { tag_name: "div".to_string(), outer_html: "<DIV>".to_string() });
    assert!(new_token_html_from_tag("<div class='x'>").unwrap() == TokenHtml::Open { tag_name: "div".to_string(), outer_html: "<div class='x'>".to_string() });
    assert!(new_token_html_from_tag("</div>").unwrap() == TokenHtml::Close { tag_name: "div".to_string(), outer_html: "</div>".to_string() });
    assert!(new_token_html_from_tag("<br/>").unwrap() == TokenHtml::SelfClosing { tag_name: "br".to_string(), outer_html: "<br/>".to_string() });
    assert!(new_token_html_from_tag("<br />").unwrap() == TokenHtml::SelfClosing { tag_name: "br".to_string(), outer_html: "<br />".to_string() });
    assert!(new_token_html_from_tag("<input type='text'/>").unwrap() == TokenHtml::SelfClosing { tag_name: "input".to_string(), outer_html: "<input type='text'/>".to_string() });
    assert!(new_token_html_from_tag("<hr//>").is_err());
    assert!(new_token_html_from_tag("<script>").unwrap() == TokenHtml::PreLikeOpen { tag_name: "script".to_string(), outer_html: "<script>".to_string() });
    assert!(new_token_html_from_tag("</script>").unwrap() == TokenHtml::PreLikeClose { tag_name: "script".to_string(), outer_html: "</script>".to_string() });
    assert!(new_token_html_from_tag("<style>").unwrap() == TokenHtml::PreLikeOpen { tag_name: "style".to_string(), outer_html: "<style>".to_string() });
    assert!(new_token_html_from_tag("</style>").unwrap() == TokenHtml::PreLikeClose { tag_name: "style".to_string(), outer_html: "</style>".to_string() });
    assert!(new_token_html_from_tag("<pre>").unwrap() == TokenHtml::PreLikeOpen { tag_name: "pre".to_string(), outer_html: "<pre>".to_string() });
    assert!(new_token_html_from_tag("</pre>").unwrap() == TokenHtml::PreLikeClose { tag_name: "pre".to_string(), outer_html: "</pre>".to_string() });
    assert!(new_token_html_from_tag("<textarea>").unwrap() == TokenHtml::PreLikeOpen { tag_name: "textarea".to_string(), outer_html: "<textarea>".to_string() });
    assert!(new_token_html_from_tag("</textarea>").unwrap() == TokenHtml::PreLikeClose { tag_name: "textarea".to_string(), outer_html: "</textarea>".to_string() });
    assert!(new_token_html_from_tag("<xmp>").unwrap() == TokenHtml::PreLikeOpen { tag_name: "xmp".to_string(), outer_html: "<xmp>".to_string() });
    assert!(new_token_html_from_tag("</xmp>").unwrap() == TokenHtml::PreLikeClose { tag_name: "xmp".to_string(), outer_html: "</xmp>".to_string() });
}

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
    assert!(validate_token_html_quotes(r#"<div class='foo'''''''''''''''''''''''''''''''''''''>"#).is_err());
    assert!(validate_token_html_quotes(r#"<div attr='he said \"hi\"'>"#).is_ok()); // no escape semantics in HTML attr
    assert!(validate_token_html_quotes(r#"<div attr="it\'s fine">"#).is_ok());
    assert!(validate_token_html_quotes(r#"<div attr="''">"#).is_ok());
    assert!(validate_token_html_quotes(r#"<div attr='""'>"#).is_ok());
    assert!(validate_token_html_quotes(r#"<div attr=''>"#).is_ok());
    assert!(validate_token_html_quotes(r#"<div attr='>"#).is_err());
    assert!(validate_token_html_quotes(r#"<div attr='''''''>"#).is_err());
}


#[test]
fn test_html_tag_name() {
    assert!(html_tag_name("<h1>").unwrap() == "h1");
    assert!(html_tag_name("< / h1    >").unwrap() == "h1");
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