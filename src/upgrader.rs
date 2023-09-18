use std::io::{Error, ErrorKind};
use nipper::Document;
use anyhow::{Result, Ok};
use url::Url;

pub struct Upgrader {
    link: Url,
    pub document: Document,
}

impl Upgrader {
    pub fn new(html: &str, link: Url) -> Result<Self> {
        let doc = Document::from(html);
        Ok(Self { document: doc, link: link })
    }

    fn remap_images_and_scripts(&mut self) -> Result<()> {
        for mut e in self.document.select("img, script").iter() {
            let attribute = e.attr("src").unwrap().to_string();
            if attribute.starts_with("..") {
                let new_path = attribute.replace("..", self.link.join("..")?.as_str());
                e.set_attr("src", &new_path);
            }
            if attribute.starts_with("/") {
                let new_path = format!("{}://{}{}", self.link.scheme(), self.link.domain().unwrap(), &attribute);
                e.set_attr("src", &new_path);
            }
        }
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
        head_tag.append_html(format!("<script>{}</script>", include_str!("public/timer.min.js")));
        Ok(())
    }

    fn remove_conflicting_scripts(&mut self) -> Result<()> {
        let conflicting = self.document.select(r#"script[src$="dobrazmiana.js"]"#);
        for mut elem in conflicting.iter() {
            elem.remove();
        }
        Ok(())
    }

    pub fn default_transformations(&mut self) -> Result<()> {
        self.remove_conflicting_scripts()?;
        self.remap_images_and_scripts()?;
        self.add_style()?;
        self.add_timer()?;
        Ok(())
    }

    pub fn output(&self) -> String {
        self.document.html().to_string()
    }
}