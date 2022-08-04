pub mod render;
pub use flow_html_macro::{html};
pub use render::{Render, Result, Write};
use std::collections::BTreeMap;


#[derive(Debug)]
pub struct HtmlElement<'a, T:Render>{
    pub tag:&'a str,
    pub attributes:BTreeMap<&'a str, &'a str>,
    pub children:Option<T>
}

impl<T:Render> Render for HtmlElement<'_, T>{
    fn render<W:Write>(self, w:&mut W)->Result{
        write!(w, "<{}", self.tag)?;
        for (key, value) in self.attributes.iter(){
            write!(w, "{}={}", key, value)?;
        }
        write!(w, ">")?;
        if let Some(children) = self.children{
            children.render(w)?;
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
    pub fn tree_html(){
        
        let world  = "world";
        let num  = 123;
        let string  = "123".to_string();
        let tree = html!{
            <div>
                {"hello"} {world} {num} {num} {num} {string} {true}
                {1.2 as f64}
                <h1>{"hello 123"} {num}</h1>
                {"10"}
                {11}
                {12} {13} {14}
                <h3>{"single child"}</h3>
            </div>
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
        println!("tree.render: {}", tree.html());
    }
}
