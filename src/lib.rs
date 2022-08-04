pub mod render;
pub use flow_html_macro::html;
pub use render::{Render, Result, Write};
use std::collections::BTreeMap;

#[derive(Debug)]
pub enum AttributeValue<'a>{
    Bool(bool),
    Str(&'a str)
}

#[derive(Debug)]
pub struct Element<'a, T:Render>{
    pub tag:&'a str,
    pub attributes:BTreeMap<&'a str, AttributeValue<'a>>,
    pub children:Option<T>
}

impl<T:Render> Render for Element<'_, T>{
    fn render<W:Write>(self, w:&mut W)->Result{
        write!(w, "<{}", self.tag)?;
        for (key, value) in self.attributes{
            match value{
                AttributeValue::Bool(v)=>{
                    if v {
                        write!(w, " {}", key)?;
                    }
                }
                AttributeValue::Str(v)=>{
                    write!(w, " {}=\"{}\"", key, v)?;
                }
            }
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
        let string2  = "string2 value".to_string();
        let user = "123";
        let active = true;
        let disabled = false;

        let tree = html!{
            <div class={"abc"} ?active ?disabled user data-user-name={"test-node"} &string2>
                {123} {"hello"} {world} {num} {num} {num} {string} {true}
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
