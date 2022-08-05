pub mod render;
pub mod escape;
pub use flow_html_macro::{html, renderable};
pub use render::{Render, Result, Write};
pub use escape::{escape_attr, escape_html};
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

pub trait ElementDefaults {
    fn _get_attributes(&self)->String;
    fn _get_children(&self)->String;

    fn get_attributes(&self)->String{
        self._get_attributes()
    }
    fn get_children(&self)->String{
        self._get_children()
    }
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
                    write!(w, " {}=\"{}\"", key, escape_attr(v))?;
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
    use crate::renderable;
    use crate::ElementDefaults;
    #[test]
    pub fn tree_html(){
        
        let world  = "world";
        let num  = 123;
        let string  = "123".to_string();
        let string2  = "string2 value".to_string();
        let user = "123";
        let active = true;
        let disabled = false;

        #[derive(Debug)]
        struct Abc{}

        #[renderable(flow-select)]
        #[allow(unused_variables)]
        struct FlowSelect<'a>{
            #[attr(name="is-active", xyz=1, xxx=true)]
            pub active:bool,
            pub selected:&'a str,
            pub name:String,
            //pub label:Option<&'a str>,
            //pub items:Vec<&'a str>,
            //pub abc:Abc
        }

        //overries
        /*
        impl<'a> FlowSelect<'a>{
            
            fn get_attributes(&self)->String{
                format!("class=\"xxxxxxx\" active")
            }
            fn get_children(&self)->String{
                format!("<flow-menu-item value=\"sss\">xyz</flow-menu-item>")
            }
        }
        */
        let name = "abc".to_string();
        let tree = html!{
            <div class={"abc"} ?active ?disabled ?active2={false} user data-user-name={"test-node"} &string2>
                {123} {"hello"} {world} {num} {num} {num} {string} {true}
                {1.2 as f64}
                <h1>{"hello 123"} {num}</h1>
                {"10"}
                {11}
                {12} {13} {14}
                <h3>{"single child"}</h3>
                <FlowSelect active name selected={"<1&2>\"3"} />
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
        

        println!("tree: {:?}", tree);
        println!("tree.html: {}", tree.html());
    }
}
