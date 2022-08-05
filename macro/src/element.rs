//use std::sync::Arc;
use proc_macro2::{TokenStream, Ident, Span};
use quote::{quote, ToTokens};
use syn::{Block, Token, Result};
use syn::parse::{Parse, ParseStream};
use proc_macro_error::abort;
use crate::attributes::{Attributes, parse_attributes};
//use crate::state::get_attributes;

pub struct Element{
    pub tag:OpeningTag,
    pub children:Option<Nodes>
}

impl Parse for Element{
    fn parse(input: ParseStream) -> Result<Self> {
        //println!("================== start: Element parsing #######################");
        let span = input.span();
        let tag = input.parse::<OpeningTag>()?;
        
        let mut children = None;
        if !tag.self_closing{
            let nodes = input.parse::<Nodes>()?;
            if nodes.list.len() > 0{
                children = Some(nodes);
            }
            let closing_tag = input.parse::<ClosingTag>()?;
            if closing_tag.name != tag.name{
                abort!(span, format!("Closing tag is missing for '{}'", tag.name));
            }
        }
        //println!("=================== end: Element parsing ########################");
        //println!("after Element parse, input: {}", input);
        
        Ok(Element{
            tag,
            children
        })
    }
}

impl Element{
    fn is_custom_element(&self)->bool{
        let name = self.tag.name.to_string();
        let first = name.get(0..1).unwrap();
        first.to_uppercase() == first
    }
    fn children_stream(&self)->TokenStream{
        match &self.children{
            Some(nodes)=>{
                let children = nodes.get_tuples();
                quote!(children:Some(#children))
            }
            None=>{
                quote!(children:Option::<()>::None)
            }
        }
    }
}

impl ToTokens for Element{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        //let mut properties:Vec<TokenStream> = vec![];
        let children = self.children_stream();
        let el = if self.is_custom_element(){
            let name = &self.tag.name;
            /*
            let names = match get_attributes(name.to_string()){
                Some(names)=>names,
                None=>Arc::new(vec![])
            };
            */
            let mut properties = self.tag.attributes.to_properties();//names);
            //println!("properties: {:?}", properties);
            properties.push(children);
            quote!(#name {
                #(#properties),*,
                ..Default::default()
            })
        }else{
            let attributes = self.tag.attributes.to_token_stream();
            let tag = self.tag.name.to_string();
            let is_fragment = tag.eq("x");
            quote!{
                flow_html::Element {
                    is_fragment:#is_fragment,
                    tag:#tag,
                    #attributes,
                    #children
                }
            }
        };

        el.to_tokens(tokens);
    }
}

pub struct OpeningTag{
    pub name:Ident,
    pub self_closing:bool,
    pub attributes:Attributes
}
fn get_fragment_ident()->Ident{
    Ident::new("x", Span::call_site())
}
impl Parse for OpeningTag{
    fn parse(input: ParseStream) -> Result<Self> {
        let mut self_closing = false;
        let name;
        let attributes;
        input.parse::<Token![<]>()?;
        if input.peek(Token![>]){
            name = get_fragment_ident();
            attributes = Attributes::empty()
        }else{
            name = input.parse::<Ident>()?;
            attributes = parse_attributes(input)?;
            if input.peek(Token![/]){
                input.parse::<Token![/]>()?;
                self_closing = true;
            }
        }

        
        input.parse::<Token![>]>()?;
        Ok(Self{
            name,
            self_closing,
            attributes
        })
    }
}

pub struct ClosingTag{
    pub name:Ident
}

impl Parse for ClosingTag{
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![<]>()?;
        input.parse::<Token![/]>()?;
        let name;
        if input.peek(Token![>]){
            name = get_fragment_ident();
        }else{
            name = input.parse::<Ident>()?;
        }
        input.parse::<Token![>]>()?;
        Ok(Self{
            name
        })
    }
}

pub struct Nodes{
    list:Vec<Node>
}

impl Nodes{
    pub fn get_tuples(&self)->TokenStream{
        if self.list.len() == 1{
            let node = &self.list[0];
            quote!{#node}
        }else{
            let mut group = vec![];
            let list:Vec<TokenStream> = self.list.iter()
                    .map(|item| quote!{#item})
                    .collect();
            for chunk in list.chunks(10){
                group.push(quote!{ ( #(#chunk),* ) } );
                if group.len() == 10{
                    let combined = quote!{ ( #(#group),* ) };
                    group = vec![];
                    group.push(combined);
                }
            }
            
            let children = quote!{(#(#group),*)};
            quote!{#children}
        }
    }
}

impl Parse for Nodes{
    fn parse(input: ParseStream) -> Result<Self> {
        let mut list:Vec<Node> = vec![];
        //println!("================== start: Nodes parsing ==================");
        while !input.is_empty() && (!input.peek(Token![<]) || !input.peek2(Token![/])){
            let node = input.parse::<Node>()?;
            list.push(node);
        }
        //println!("==================== end: Nodes parsing ==================");
        //println!("after nodes parse, input: {:?}", input);

        Ok(Nodes{
            list
        })
    }
}
impl ToTokens for Nodes{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.get_tuples().to_tokens(tokens);
    }
}

pub enum Node{
    Element(Element),
    Block(Block)
}

impl Parse for Node{
    fn parse(input: ParseStream) -> Result<Self> {
        let node = if input.peek(Token![<]){
            Node::Element(input.parse::<Element>()?)
        }else{
            Node::Block(input.parse::<Block>()?)
        };

        Ok(node)
    }
}
impl ToTokens for Node{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self{
            Node::Element(el)=>{
                el.to_tokens(tokens);
            }
            Node::Block(block)=>{
                if block.stmts.len() == 1{
                    let stm = &block.stmts[0];
                    stm.to_tokens(tokens);
                }else{
                    block.to_tokens(tokens);
                }
            }
        }
    }
}