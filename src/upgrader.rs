use html5ever::{parse_document, ParseOpts, Parser, tree_builder::TreeBuilderOpts};
use anyhow::Result;

struct Upgrader {
    pub document: Parser<String>,
}

impl Upgrader {
    pub fn new(html: String) -> Result<Self> {
        let opts = ParseOpts {
            tree_builder: TreeBuilderOpts {
                drop_doctype: true,
                ..Default::default()
            }
            ..Default::default()
        };
        let doc = parse_document(html, opts); 
        Ok(Self { document: doc })
    }

    fn add_style(&mut self) -> Result<()> {
        let style = 
        Ok(())
    }

    fn add_timer(&mut self) -> Result<()> {
        unimplemented!()
    }
}