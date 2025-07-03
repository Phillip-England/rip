use crate::*;

#[test]
fn test_html_tokenize() {

    let toks = html_tokenize("<h1 attr=''></h1>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "h1".to_string(), outer_html: "<h1 attr=''>".to_string() }, 
        TokenHtml::Close { tag_name: "h1".to_string(), outer_html: "</h1>".to_string() },
    ]);

    
    let toks = html_tokenize("<h1>Hello, World!</h1>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "h1".to_string(), outer_html: "<h1>".to_string() }, 
        TokenHtml::InnerText { text : "Hello, World!".to_string() },
        TokenHtml::Close { tag_name: "h1".to_string(), outer_html: "</h1>".to_string() },
    ]);

    let toks = html_tokenize("<h1></h1>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "h1".to_string(), outer_html: "<h1>".to_string() }, 
        TokenHtml::Close { tag_name: "h1".to_string(), outer_html: "</h1>".to_string() },
    ]);

    let toks = html_tokenize("<h1>    </h1>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "h1".to_string(), outer_html: "<h1>".to_string() },
        TokenHtml::Whitespace { text: "    ".to_string() }, 
        TokenHtml::Close { tag_name: "h1".to_string(), outer_html: "</h1>".to_string() },
    ]);

    let toks = html_tokenize("<h1><p>Hello, World!</p></h1>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "h1".to_string(), outer_html: "<h1>".to_string() }, 
        TokenHtml::Open { tag_name: "p".to_string(), outer_html: "<p>".to_string() }, 
        TokenHtml::InnerText { text : "Hello, World!".to_string() },
        TokenHtml::Close { tag_name: "p".to_string(), outer_html: "</p>".to_string() },
        TokenHtml::Close { tag_name: "h1".to_string(), outer_html: "</h1>".to_string() },
    ]);

    let toks = html_tokenize("<script>console.log('hello, world!')</script>").unwrap();
    assert!(toks == vec![
        TokenHtml::PreLikeOpen { tag_name: "script".to_string(), outer_html: "<script>".to_string() }, 
        TokenHtml::InnerText { text: "console.log('hello, world!')".to_string() },
        TokenHtml::PreLikeClose { tag_name: "script".to_string(), outer_html: "</script>".to_string() }, 
    ]);

    let toks = html_tokenize("<script>console.log('</script>')</script>").unwrap();
    assert!(toks == vec![
        TokenHtml::PreLikeOpen { tag_name: "script".to_string(), outer_html: "<script>".to_string() }, 
        TokenHtml::InnerText { text: "console.log('</script>')".to_string() },
        TokenHtml::PreLikeClose { tag_name: "script".to_string(), outer_html: "</script>".to_string() }, 
    ]);

    let toks = html_tokenize("<h1><p>Hello, World!</p>     </h1>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "h1".to_string(), outer_html: "<h1>".to_string() }, 
        TokenHtml::Open { tag_name: "p".to_string(), outer_html: "<p>".to_string() }, 
        TokenHtml::InnerText { text : "Hello, World!".to_string() },
        TokenHtml::Close { tag_name: "p".to_string(), outer_html: "</p>".to_string() },
        TokenHtml::Whitespace { text: "     ".to_string() },
        TokenHtml::Close { tag_name: "h1".to_string(), outer_html: "</h1>".to_string() },
    ]);

    let toks = html_tokenize("<h1>     <p>Hello, World!</p>     </h1>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "h1".to_string(), outer_html: "<h1>".to_string() }, 
        TokenHtml::Whitespace { text: "     ".to_string() },
        TokenHtml::Open { tag_name: "p".to_string(), outer_html: "<p>".to_string() }, 
        TokenHtml::InnerText { text : "Hello, World!".to_string() },
        TokenHtml::Close { tag_name: "p".to_string(), outer_html: "</p>".to_string() },
        TokenHtml::Whitespace { text: "     ".to_string() },
        TokenHtml::Close { tag_name: "h1".to_string(), outer_html: "</h1>".to_string() },
    ]);

    let toks = html_tokenize("<div><span>Text</span></div>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "div".to_string(), outer_html: "<div>".to_string() },
        TokenHtml::Open { tag_name: "span".to_string(), outer_html: "<span>".to_string() },
        TokenHtml::InnerText { text: "Text".to_string() },
        TokenHtml::Close { tag_name: "span".to_string(), outer_html: "</span>".to_string() },
        TokenHtml::Close { tag_name: "div".to_string(), outer_html: "</div>".to_string() },
    ]);

    let toks = html_tokenize("<div>  <span>Text</span>  </div>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "div".to_string(), outer_html: "<div>".to_string() },
        TokenHtml::Whitespace { text: "  ".to_string() },
        TokenHtml::Open { tag_name: "span".to_string(), outer_html: "<span>".to_string() },
        TokenHtml::InnerText { text: "Text".to_string() },
        TokenHtml::Close { tag_name: "span".to_string(), outer_html: "</span>".to_string() },
        TokenHtml::Whitespace { text: "  ".to_string() },
        TokenHtml::Close { tag_name: "div".to_string(), outer_html: "</div>".to_string() },
    ]);

    let toks = html_tokenize("<ul><li>Item 1</li><li>Item 2</li></ul>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "ul".to_string(), outer_html: "<ul>".to_string() },
        TokenHtml::Open { tag_name: "li".to_string(), outer_html: "<li>".to_string() },
        TokenHtml::InnerText { text: "Item 1".to_string() },
        TokenHtml::Close { tag_name: "li".to_string(), outer_html: "</li>".to_string() },
        TokenHtml::Open { tag_name: "li".to_string(), outer_html: "<li>".to_string() },
        TokenHtml::InnerText { text: "Item 2".to_string() },
        TokenHtml::Close { tag_name: "li".to_string(), outer_html: "</li>".to_string() },
        TokenHtml::Close { tag_name: "ul".to_string(), outer_html: "</ul>".to_string() },
    ]);

    let toks = html_tokenize("<a href='#'>Link</a>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "a".to_string(), outer_html: "<a href='#'>".to_string() },
        TokenHtml::InnerText { text: "Link".to_string() },
        TokenHtml::Close { tag_name: "a".to_string(), outer_html: "</a>".to_string() },
    ]);

    let toks = html_tokenize("<input type='text'/>").unwrap();
    assert!(toks == vec![
        TokenHtml::SelfClosing { tag_name: "input".to_string(), outer_html: "<input type='text'/>".to_string() },
    ]);

    let toks = html_tokenize("<br/>").unwrap();
    assert!(toks == vec![
        TokenHtml::SelfClosing { tag_name: "br".to_string(), outer_html: "<br/>".to_string() },
    ]);

    let toks = html_tokenize("<p>Hello<br/>World</p>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "p".to_string(), outer_html: "<p>".to_string() },
        TokenHtml::InnerText { text: "Hello".to_string() },
        TokenHtml::SelfClosing { tag_name: "br".to_string(), outer_html: "<br/>".to_string() },
        TokenHtml::InnerText { text: "World".to_string() },
        TokenHtml::Close { tag_name: "p".to_string(), outer_html: "</p>".to_string() },
    ]);

    let toks = html_tokenize("<section><h2>Title</h2><p>Body</p></section>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "section".to_string(), outer_html: "<section>".to_string() },
        TokenHtml::Open { tag_name: "h2".to_string(), outer_html: "<h2>".to_string() },
        TokenHtml::InnerText { text: "Title".to_string() },
        TokenHtml::Close { tag_name: "h2".to_string(), outer_html: "</h2>".to_string() },
        TokenHtml::Open { tag_name: "p".to_string(), outer_html: "<p>".to_string() },
        TokenHtml::InnerText { text: "Body".to_string() },
        TokenHtml::Close { tag_name: "p".to_string(), outer_html: "</p>".to_string() },
        TokenHtml::Close { tag_name: "section".to_string(), outer_html: "</section>".to_string() },
    ]);

    let toks = html_tokenize("<p>  Hello   <b>world</b>  </p>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "p".to_string(), outer_html: "<p>".to_string() },
        TokenHtml::InnerText { text: "  Hello   ".to_string() },
        TokenHtml::Open { tag_name: "b".to_string(), outer_html: "<b>".to_string() },
        TokenHtml::InnerText { text: "world".to_string() },
        TokenHtml::Close { tag_name: "b".to_string(), outer_html: "</b>".to_string() },
        TokenHtml::Whitespace { text: "  ".to_string() },
        TokenHtml::Close { tag_name: "p".to_string(), outer_html: "</p>".to_string() },
    ]);

    let toks = html_tokenize("<style>h1 { color: red; }</style>").unwrap();
    assert!(toks == vec![
        TokenHtml::PreLikeOpen { tag_name: "style".to_string(), outer_html: "<style>".to_string() },
        TokenHtml::InnerText { text: "h1 { color: red; }".to_string() },
        TokenHtml::PreLikeClose { tag_name: "style".to_string(), outer_html: "</style>".to_string() },
    ]);

    let toks = html_tokenize("<textarea>  <h1>not parsed</h1>  </textarea>").unwrap();
    assert!(toks == vec![
        TokenHtml::PreLikeOpen { tag_name: "textarea".to_string(), outer_html: "<textarea>".to_string() },
        TokenHtml::InnerText { text: "  <h1>not parsed</h1>  ".to_string() },
        TokenHtml::PreLikeClose { tag_name: "textarea".to_string(), outer_html: "</textarea>".to_string() },
    ]);

    let toks = html_tokenize("<div><br/></div>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "div".to_string(), outer_html: "<div>".to_string() },
        TokenHtml::SelfClosing { tag_name: "br".to_string(), outer_html: "<br/>".to_string() },
        TokenHtml::Close { tag_name: "div".to_string(), outer_html: "</div>".to_string() },
    ]);

    let toks = html_tokenize("<p>  Hi <b>there</b> world  </p>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "p".to_string(), outer_html: "<p>".to_string() },
        TokenHtml::InnerText { text: "  Hi ".to_string() },
        TokenHtml::Open { tag_name: "b".to_string(), outer_html: "<b>".to_string() },
        TokenHtml::InnerText { text: "there".to_string() },
        TokenHtml::Close { tag_name: "b".to_string(), outer_html: "</b>".to_string() },
        TokenHtml::InnerText { text: " world  ".to_string() },
        TokenHtml::Close { tag_name: "p".to_string(), outer_html: "</p>".to_string() },
    ]);

    let toks = html_tokenize("<style>.hidden { display: none; }</style>").unwrap();
    assert!(toks == vec![
        TokenHtml::PreLikeOpen { tag_name: "style".to_string(), outer_html: "<style>".to_string() },
        TokenHtml::InnerText { text: ".hidden { display: none; }".to_string() },
        TokenHtml::PreLikeClose { tag_name: "style".to_string(), outer_html: "</style>".to_string() },
    ]);

    let toks = html_tokenize("<script>let a = '<br/>';</script>").unwrap();
    assert!(toks == vec![
        TokenHtml::PreLikeOpen { tag_name: "script".to_string(), outer_html: "<script>".to_string() },
        TokenHtml::InnerText { text: "let a = '<br/>';".to_string() },
        TokenHtml::PreLikeClose { tag_name: "script".to_string(), outer_html: "</script>".to_string() },
    ]);

    let toks = html_tokenize("<pre>    </pre>").unwrap();
    assert!(toks == vec![
        TokenHtml::PreLikeOpen { tag_name: "pre".to_string(), outer_html: "<pre>".to_string() },
        TokenHtml::Whitespace { text: "    ".to_string() },
        TokenHtml::PreLikeClose { tag_name: "pre".to_string(), outer_html: "</pre>".to_string() },
    ]);

    let toks = html_tokenize("<DIV><P>Hello</P></DIV>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "div".to_string(), outer_html: "<DIV>".to_string() },
        TokenHtml::Open { tag_name: "p".to_string(), outer_html: "<P>".to_string() },
        TokenHtml::InnerText { text: "Hello".to_string() },
        TokenHtml::Close { tag_name: "p".to_string(), outer_html: "</P>".to_string() },
        TokenHtml::Close { tag_name: "div".to_string(), outer_html: "</DIV>".to_string() },
    ]);


    let toks = html_tokenize("<style>body { color: red; } </div></style>").unwrap();
    assert!(toks == vec![
        TokenHtml::PreLikeOpen { tag_name: "style".to_string(), outer_html: "<style>".to_string() },
        TokenHtml::InnerText { text: "body { color: red; } </div>".to_string() },
        TokenHtml::PreLikeClose { tag_name: "style".to_string(), outer_html: "</style>".to_string() },
    ]);

    let toks = html_tokenize("<div><section><article><p><span>Hello</span></p></article></section></div>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "div".to_string(), outer_html: "<div>".to_string() },
        TokenHtml::Open { tag_name: "section".to_string(), outer_html: "<section>".to_string() },
        TokenHtml::Open { tag_name: "article".to_string(), outer_html: "<article>".to_string() },
        TokenHtml::Open { tag_name: "p".to_string(), outer_html: "<p>".to_string() },
        TokenHtml::Open { tag_name: "span".to_string(), outer_html: "<span>".to_string() },
        TokenHtml::InnerText { text: "Hello".to_string() },
        TokenHtml::Close { tag_name: "span".to_string(), outer_html: "</span>".to_string() },
        TokenHtml::Close { tag_name: "p".to_string(), outer_html: "</p>".to_string() },
        TokenHtml::Close { tag_name: "article".to_string(), outer_html: "</article>".to_string() },
        TokenHtml::Close { tag_name: "section".to_string(), outer_html: "</section>".to_string() },
        TokenHtml::Close { tag_name: "div".to_string(), outer_html: "</div>".to_string() },
    ]);

    let toks = html_tokenize("<a><b><c><d><e><f> content </f></e></d></c></b></a>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "a".to_string(), outer_html: "<a>".to_string() },
        TokenHtml::Open { tag_name: "b".to_string(), outer_html: "<b>".to_string() },
        TokenHtml::Open { tag_name: "c".to_string(), outer_html: "<c>".to_string() },
        TokenHtml::Open { tag_name: "d".to_string(), outer_html: "<d>".to_string() },
        TokenHtml::Open { tag_name: "e".to_string(), outer_html: "<e>".to_string() },
        TokenHtml::Open { tag_name: "f".to_string(), outer_html: "<f>".to_string() },
        TokenHtml::InnerText { text: " content ".to_string() },
        TokenHtml::Close { tag_name: "f".to_string(), outer_html: "</f>".to_string() },
        TokenHtml::Close { tag_name: "e".to_string(), outer_html: "</e>".to_string() },
        TokenHtml::Close { tag_name: "d".to_string(), outer_html: "</d>".to_string() },
        TokenHtml::Close { tag_name: "c".to_string(), outer_html: "</c>".to_string() },
        TokenHtml::Close { tag_name: "b".to_string(), outer_html: "</b>".to_string() },
        TokenHtml::Close { tag_name: "a".to_string(), outer_html: "</a>".to_string() },
    ]);

    let toks = html_tokenize("<pre><div>not parsed</div></pre>").unwrap();
    assert!(toks == vec![
        TokenHtml::PreLikeOpen { tag_name: "pre".to_string(), outer_html: "<pre>".to_string() },
        TokenHtml::InnerText { text: "<div>not parsed</div>".to_string() },
        TokenHtml::PreLikeClose { tag_name: "pre".to_string(), outer_html: "</pre>".to_string() },
    ]);


    let toks = html_tokenize("<style>.a{}</style><style>.b{}</style>").unwrap();
    assert!(toks == vec![
        TokenHtml::PreLikeOpen { tag_name: "style".to_string(), outer_html: "<style>".to_string() },
        TokenHtml::InnerText { text: ".a{}".to_string() },
        TokenHtml::PreLikeClose { tag_name: "style".to_string(), outer_html: "</style>".to_string() },
        TokenHtml::PreLikeOpen { tag_name: "style".to_string(), outer_html: "<style>".to_string() },
        TokenHtml::InnerText { text: ".b{}".to_string() },
        TokenHtml::PreLikeClose { tag_name: "style".to_string(), outer_html: "</style>".to_string() },
    ]);

    let toks = html_tokenize("<div    class='x'    >content</div>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "div".to_string(), outer_html: "<div    class='x'    >".to_string() },
        TokenHtml::InnerText { text: "content".to_string() },
        TokenHtml::Close { tag_name: "div".to_string(), outer_html: "</div>".to_string() },
    ]);

    let toks = html_tokenize(r#"<input type="text" value='123'/>"#).unwrap();
    assert!(toks == vec![
        TokenHtml::SelfClosing { tag_name: "input".to_string(), outer_html: r#"<input type="text" value='123'/>"#.to_string() },
    ]);

    let toks = html_tokenize("<div> <span> <b>bold</b> </span> </div>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "div".to_string(), outer_html: "<div>".to_string() },
        TokenHtml::Whitespace { text: " ".to_string() },
        TokenHtml::Open { tag_name: "span".to_string(), outer_html: "<span>".to_string() },
        TokenHtml::Whitespace { text: " ".to_string() },
        TokenHtml::Open { tag_name: "b".to_string(), outer_html: "<b>".to_string() },
        TokenHtml::InnerText { text: "bold".to_string() },
        TokenHtml::Close { tag_name: "b".to_string(), outer_html: "</b>".to_string() },
        TokenHtml::Whitespace { text: " ".to_string() },
        TokenHtml::Close { tag_name: "span".to_string(), outer_html: "</span>".to_string() },
        TokenHtml::Whitespace { text: " ".to_string() },
        TokenHtml::Close { tag_name: "div".to_string(), outer_html: "</div>".to_string() },
    ]);

    let toks = html_tokenize("<pre><img src='foo'/>not parsed</pre>").unwrap();
    assert!(toks == vec![
        TokenHtml::PreLikeOpen { tag_name: "pre".to_string(), outer_html: "<pre>".to_string() },
        TokenHtml::InnerText { text: "<img src='foo'/>not parsed".to_string() },
        TokenHtml::PreLikeClose { tag_name: "pre".to_string(), outer_html: "</pre>".to_string() },
    ]);

    let toks = html_tokenize("<p>The &lt;em&gt; tag is used for emphasis.</p>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "p".to_string(), outer_html: "<p>".to_string() },
        TokenHtml::InnerText { text: "The &lt;em&gt; tag is used for emphasis.".to_string() },
        TokenHtml::Close { tag_name: "p".to_string(), outer_html: "</p>".to_string() },
    ]);

    let toks = html_tokenize("<h1>Hello</h1><h2>World</h2>").unwrap();
    assert!(toks == vec![
        TokenHtml::Open { tag_name: "h1".to_string(), outer_html: "<h1>".to_string() },
        TokenHtml::InnerText { text: "Hello".to_string() },
        TokenHtml::Close { tag_name: "h1".to_string(), outer_html: "</h1>".to_string() },
        TokenHtml::Open { tag_name: "h2".to_string(), outer_html: "<h2>".to_string() },
        TokenHtml::InnerText { text: "World".to_string() },
        TokenHtml::Close { tag_name: "h2".to_string(), outer_html: "</h2>".to_string() },
    ]);


}