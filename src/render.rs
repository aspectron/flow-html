pub use std::fmt::{Result, Write};
use crate::utils::{ElementResult, Element, document};
use std::collections::BTreeMap;
use crate::Html;


/*
pub trait RenderBase:Sized{
    fn render_html<W:Write>(self, _w:&mut W)->Result;
}
*/
pub trait Render:Sized{
    fn html(&self)->String{
        let mut buf = String::from("");
        self.render(&mut buf).unwrap();
        buf
    }
    // fn render_tree(self)->ElementResult<(Vec<Element>, BTreeMap<String, Element>)>{
    fn render_tree(self)->ElementResult<Html>{
        let mut parent = document().create_element("div").unwrap();
        //parent.set_attribute("class", "temp-root")?;
        let map = self.render_tree_into(&mut parent)?;
        let mut list = vec![];
        let children = parent.children();
        let len = children.length();
        for index in 0..len{
            if let Some(child) = children.get_with_index(index){
                list.push(child);
            }
        }
        Ok(Html::new(list, map)?)
    }
    fn render_tree_into(self, parent: &mut Element)->ElementResult<BTreeMap<String, Element>>{
        let mut map = BTreeMap::new();
        self.render_node(parent, &mut map)?;
        Ok(map)
    }
    
    fn render_node(self, _parent:&mut Element, _map:&mut BTreeMap<String, Element>)->ElementResult<()>{
        Ok(())
    }

    fn render<W:Write>(&self, w:&mut W)->Result;
}


//impl Render for () {}
//impl Render for &str {}
impl Render for () {
    fn render<W:Write>(&self, _w:&mut W)->Result{
        Ok(())
    }
}

impl Render for &str {
    fn render<W:Write>(&self, w:&mut W)->Result{
        write!(w, "{}", self)
    }
    fn render_node(self, parent:&mut Element, _map:&mut BTreeMap<String, Element>)->ElementResult<()>{
        let el = document().create_text_node(self);
        parent.append_child(&el)?;
        Ok(())
    }
}

macro_rules! impl_tuple {
    ($($ident:ident)+) => {
        //impl<$($ident: Render,)+> Render for ($($ident,)+) {}
        impl<$($ident: Render,)+> Render for ($($ident,)+) {
            #[inline]
            #[allow(non_snake_case)]
            fn render<W:Write>(&self, w:&mut W)->Result{
                let ($($ident,)+) = self;
                $($ident.render(w)?;)+
                Ok(())
            }
            #[allow(non_snake_case)]
            fn render_node(self, parent:&mut Element, map:&mut BTreeMap<String, Element>)->ElementResult<()>{
                let ($($ident,)+) = self;
                $($ident.render_node(parent, map)?;)+
                Ok(())
            }
        }
    }
}

macro_rules! impl_types {
    ($($ident:ident)+) => {
        $(
            //impl Render for $ident {}
            impl Render for $ident {
                fn render<W:Write>(&self, w:&mut W)->Result{
                    write!(w, "{}", self)
                }
                fn render_node(self, parent:&mut Element, _map:&mut BTreeMap<String, Element>)->ElementResult<()>{
                    let el = document().create_text_node(&format!("{}", self));
                    parent.append_child(&el)?;
                    Ok(())
                }
            }
        )+
    }
}

impl_types!{f32 f64 u128 u64 u32 u16 u8 i8 i16 i32 i64 i128 bool String usize}

impl_tuple!{A B}
impl_tuple!{A B C}
impl_tuple!{A B C D}
impl_tuple!{A B C D E}
impl_tuple!{A B C D F G}
impl_tuple!{A B C D F G H}
impl_tuple!{A B C D F G H I}
impl_tuple!{A B C D F G H I J}
impl_tuple!{A B C D F G H I J K}

