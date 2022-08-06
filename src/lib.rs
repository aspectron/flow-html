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

#[derive(Debug, Default)]
pub struct Element<'a, T:Render>{
    pub is_fragment:bool,
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
    fn render<W:Write>(&self, w:&mut W)->Result{
        if self.is_fragment{
            if let Some(children) = &self.children{
                children.render(w)?;
            }
        }else{
            write!(w, "<{}", self.tag)?;
            for (key, value) in &self.attributes{
                match value{
                    AttributeValue::Bool(v)=>{
                        if *v {
                            write!(w, " {}", key)?;
                        }
                    }
                    AttributeValue::Str(v)=>{
                        write!(w, " {}=\"{}\"", key, (*v))?;
                    }
                }
            }
            write!(w, ">")?;
            if let Some(children) = &self.children{
                children.render(w)?;
            }
            write!(w, "</{}>", self.tag)?;
        }
        Ok(())
    }
}


#[cfg(test)]
mod test{
    //cargo test -- --nocapture --test-threads=1
    use crate::html;
    use crate as flow_html;
    use crate::Render;
    use crate::renderable;
    use crate::ElementDefaults;
    #[test]
    pub fn simple_html(){
        self::print_hr("simple_html");
        let tree = html!{
            <p>
                <div class={"xyz"}></div>
                <div class={"abc"}></div>
            </p>
        };
        let result = tree.html();
        println!("tag: {:#?}", tree.tag);
        println!("html: {}", result);
        assert_eq!(result, "<p><div class=\"xyz\"></div><div class=\"abc\"></div></p>");
    }
    #[test]
    pub fn custom_elements(){
        self::print_hr("simple_html");
        let tree = html!{
            <flow-select>
                <flow-menu-item class={"xyz"} />
                <flow-menu-item class={"abc"} />
            </flow-select>
        };
        let result = tree.html();
        println!("tag: {:#?}", tree.tag);
        println!("html: {}", result);
        assert_eq!(result, "<flow-select><flow-menu-item class=\"xyz\"></flow-menu-item><flow-menu-item class=\"abc\"></flow-menu-item></flow-select>");
    }
    #[test]
    pub fn without_root_element(){
        self::print_hr("without_root_element");
        let tree = html!{
            <div class={"xyz"}></div>
            <div class={"abc"}></div>
        };
        let result = tree.html();
        println!("html: {}", result);
        assert_eq!(result, "<div class=\"xyz\"></div><div class=\"abc\"></div>");
    }
    #[test]
    pub fn complex_html(){
        self::print_hr("complex_html");
        let world  = "world";
        let num  = 123;
        let string  = "123".to_string();
        let string2  = "string2 value".to_string();
        let user = "123";
        let active = true;
        let disabled = false;
        let selected = "1";
        

        #[renderable(flow-select)]
        #[allow(unused_variables)]
        struct FlowSelect<'a, R:Render>{
            #[attr(name="is-active")]
            pub active:bool,
            pub selected:&'a str,
            pub name:String,
            pub children:Option<R>,
            pub label:Option<String>
        }
        
        #[renderable(flow-menu-item)]
        struct FlowMenuItem<'a, R:Render>{
            pub text:&'a str,
            pub value:&'a str,
            pub children:Option<R>
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
        //let name = "abc".to_string();
        //let selected = "1".to_string();
        let name2 = "aaa".to_string();
        let name3 = "bbb".to_string();
        let tree = html!{
            <div class={"abc"} ?active ?disabled ?active2={false} user data-user-name={"test-node"} &string2>
                {123} {"hello"} {world} {num} {num} {num} {string} {true}
                {1.2 as f64}
                <h1>{"hello 123"} {num}</h1>
                {"10"}
                {11}
                {12} {13} {14}
                <h3>{"single child"}</h3>
                <FlowSelect active name={name2} selected={"<1&2>\"3"} />
                <div class={"abc"}></div>
                <FlowSelect active name={name3} &selected>
                    <flow text={"abc"} />
                    <FlowMenuItem text={"abc"} value={"abc"} />
                </FlowSelect>
            </div>
        };
        
        let result = tree.html();
        println!("tag: {:#?}", tree.tag);
        println!("html: {}", result);
        assert_eq!(
            result,
            "<div active class=\"abc\" data-user-name=\"test-node\" string2=\"string2 value\" user=\"123\">123helloworld123123123123true1.2<h1>hello 123123</h1>1011121314<h3>single child</h3><flow-select is-active selected=\"&lt;1&amp;2&gt;&quot;3\" name=\"aaa\"></flow-select><div class=\"abc\"></div><flow-select is-active selected=\"1\" name=\"bbb\"><flow text=\"abc\"></flow><flow-menu-item text=\"abc\" value=\"abc\"></flow-menu-item></flow-select></div>"
        );
    }

    fn print_hr<'a>(_title: &'a str){
        //println!("\n☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁☁\n");
        println!("\n☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰☰\n")
    }
}
