pub use flow_html_macro::{html};
use std::collections::BTreeMap;
use std::fmt::{Result, Write};

pub trait Render:Sized{
    fn to_string(self)->String{
        let mut buf = String::from("");
        self.render(&mut buf).unwrap();
        buf
    }
    fn render<W:Write>(self, _w:&mut W)->Result;
}

impl Render for () {
    fn render<W:Write>(self, _w:&mut W)->Result{
        Ok(())
    }
}

impl Render for &str {
    fn render<W:Write>(self, w:&mut W)->Result{
        write!(w, "{}", self)
    }
}


#[derive(Debug)]
pub struct HtmlElement<'a, T:Render>{
    pub tag:&'a str,
    pub attributes:BTreeMap<&'a str, &'a str>,
    pub children:Option<Vec<T>>
}

impl<T:Render> Render for HtmlElement<'_, T>{
    fn render<W:Write>(self, w:&mut W)->Result{
        write!(w, "<{}", self.tag)?;
        for (key, value) in self.attributes.iter(){
            write!(w, "{}={}", key, value)?;
        }
        write!(w, ">")?;
        if let Some(children) = self.children{
            for child in children{
                child.render(w)?;
            }
        }
        write!(w, "</{}>", self.tag)
    }
}


#[cfg(test)]
mod test{
    use crate::html;
    use crate as flow_html;
    use crate::Render;
    #[test]
    pub fn test(){
        
        let num  = "world";
        let tree = html!{
            <div>{"hello"} {num}</div>
        };
        
        /*
        let result = flow_html::HtmlNode{
            attributes:
                { let mut map = std :: collections :: BTreeMap :: new() ; map },
            children:
                { Option::<Vec<()>>::None }
        };
        */
        

        println!("result: {:?}", tree);
        println!("tree.render: {}", tree.to_string());
    }
}
