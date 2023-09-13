use std::io::{Error, ErrorKind};
use nipper::{Document, Selection};
use anyhow::Result;

pub struct Upgrader {
    domain: String,
    pub document: Document,
}

impl Upgrader {
    pub fn new(html: &str, domain: &str) -> Result<Self> {
        let doc = Document::from(html);
        Ok(Self { document: doc, domain: String::from(domain) })
    }

    fn remap_images_and_scripts(&mut self) -> Result<()> {
        let remap = |e: &mut Selection<'_>| {
            let attribute = e.attr("src").unwrap().to_string();
            if attribute.starts_with("..") {
                let new_path = attribute.replace("..", &self.domain);
                e.set_attr("src", &new_path);
            }
            if attribute.starts_with("/") {
                let new_path = format!("{}{}", self.domain, &attribute);
                e.set_attr("src", &new_path);
            }
        };
        
        remap(&mut self.document.select("img"));
        remap(&mut self.document.select("script"));
        Ok(())
    }

    fn add_style(&mut self) -> Result<()> {
        let mut style_tag = self.document.select("head > style");
        if !style_tag.exists() {
            return Err(anyhow::Error::new(
                Error::new(ErrorKind::InvalidData, "plan cannot be parsed")
            ));
        }
        style_tag.set_html(include_str!("public/style.css"));
        Ok(())
    }

    fn add_timer(&mut self) -> Result<()> {
        let mut head_tag = self.document.select("body");
        head_tag.append_html(r#"<script src="https://code.jquery.com/jquery-2.2.4.min.js"></script><script src="https://zsem.edu.pl/plany/scripts/dobrazmiana.js"></script>"#);
        Ok(())
    }

    pub fn default_transformations(&mut self) -> Result<()> {
        self.remap_images_and_scripts()?;
        self.add_style()?;
        self.add_timer()?;
        Ok(())
    }

    pub fn output(&self) -> String {
        self.document.html().to_string()
    }
}