use std::collections::BTreeMap;
pub use crate::utils::{Element, document, ElementResult};

pub struct Html {
    pub roots: Vec<Element>,
    pub hooks: BTreeMap<String, Element>,
}

impl Html {
    pub fn new(roots : Vec<Element>, hooks : BTreeMap<String, Element>) -> ElementResult<Html> {
        let html = Html {
            roots,
            hooks
        };
        Ok(html)
    }

    pub fn roots<'html>(&'html self) -> &'html Vec<Element> {
        &self.roots
    }

    pub fn hooks<'html>(&'html self) -> &'html BTreeMap<String,Element> {
        &self.hooks
    }

    pub fn inject_into(&self, element : &Element) -> ElementResult<()> {
        for root in self.roots.iter() {
            element.append_child(&root)?;
        }
        Ok(())
    }
}
