use crate::tokenizer::{html_tokenize, TokenHtml};

pub struct DomNode {
    Children:  Vec<DomNode>
}

impl DomNode {

    pub fn new() -> DomNode {
        let node = DomNode {
            Children: vec![],
        };
        node
    }

    pub fn append(&mut self, node: DomNode) {
        self.Children.push(node);
    } 

}


pub fn dom_tree_from_html(html: &str) {
    let toks = html_tokenize("<p>Hello!</p>").unwrap();
    let head = DomNode::new();
    let mut stack: Vec<TokenHtml> = vec![];

 

}




