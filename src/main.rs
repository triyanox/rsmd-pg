use std::collections::HashMap;

#[derive(Debug)]
enum Node {
    Header(usize, String),
    Paragraph(Vec<Node>),
    Text(String),
    Strikethrough(String),
    Emphasis(String),
    Bold(String),
    Link(String, String),
    Image(String, String, String),
    FootnoteDefinition(String, String),
    Blockquote(Vec<Node>),
    InlineCode(String),
    CodeBlock(String, Option<String>),
    HorizontalRule,
    UnorderedList(Vec<Node>),
    OrderedList(Vec<Node>),
}

struct Parser {
    text: String,
    pos: usize,
    nodes: Vec<Node>,
}

impl Parser {
    fn new(text: String) -> Parser {
        Parser {
            text,
            pos: 0,
            nodes: Vec::new(),
        }
    }
}

fn main() {
    let markdown = r#"
# This is a header
This is a ~paragraph~ with some *emphasis* and **strong** text.

- This is an unordered list item.
- This is another unordered liP st item.

1. This is an ordered list item.
2. This is another ordered list item.

> This is a blockquote.

Some inline code: `let x = 42;`.

A link: [Google](https://www.google.com/).

An image: ![Image alt text](https://via.placeholder.com/150 "Image title").

---

| Column 1 | Column 2 |
| --- | --- |
| Row 1, cell 1 | Row 1, cell 2 |
| Row 2, cell 1 | Row 2, cell 2 |

[^1]: This is a footnote definition."#;
}
