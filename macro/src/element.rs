use proc_macro2::{TokenStream, Ident};
use quote::{quote, ToTokens};
use syn::{Block, Token, Result};
use syn::parse::{Parse, ParseStream};
use proc_macro_error::abort;
use crate::attributes::{Attributes, parse_attributes};

pub struct Element{
    pub tag:OpeningTag,
    pub children:Option<Nodes>
}

impl Parse for Element{
    fn parse(input: ParseStream) -> Result<Self> {
        let span = input.span();
        let tag = input.parse::<OpeningTag>()?;
        
        let mut children = None;
        if !tag.self_closing{
            children = Some(input.parse::<Nodes>()?);
            let closing_tag = input.parse::<ClosingTag>()?;
            if closing_tag.name != tag.name{
                abort!(span, format!("Closing tag is missing for '{}'", tag.name));
            }
        }

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
}

impl ToTokens for Element{
    fn to_tokens(&self, tokens: &mut TokenStream) {
        //let mut properties:Vec<TokenStream> = vec![];
        let el = if self.is_custom_element(){
            let name = &self.tag.name;
            let properties = self.tag.attributes.to_properties();
            //println!("properties: {:?}", properties);
            quote!(#name {
                #(#properties),*
            })
        }else{
            let attributes = self.tag.attributes.to_token_stream();

            let children = match &self.children{
                Some(nodes)=>{
                    if nodes.list.len() == 1{
                        let node = &nodes.list[0];
                        quote!{children:Some(#node)}
                    }else{
                        let mut group = vec![];
                        let list:Vec<TokenStream> = nodes.list.iter()
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
                        quote!{children:Some(#children)}
                    }
                }
                None=>{
                    quote!(children:Option::<Vec<()>>::None)
                }
            };
            let tag = self.tag.name.to_string();
            quote!{
                flow_html::Element {
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

impl Parse for OpeningTag{
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![<]>()?;
        let name = input.parse::<Ident>()?;
        let attributes = parse_attributes(input)?;

        let mut self_closing = false;
        if input.peek(Token![/]){
            input.parse::<Token![/]>()?;
            self_closing = true;
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
        let name = input.parse::<Ident>()?;
        input.parse::<Token![>]>()?;
        Ok(Self{
            name
        })
    }
}

pub struct Nodes{
    list:Vec<Node>
}

impl Parse for Nodes{
    fn parse(input: ParseStream) -> Result<Self> {
        let mut list:Vec<Node> = vec![];
        while !input.peek(Token![<]) || !input.peek2(Token![/]){
            let node = input.parse::<Node>()?;
            list.push(node);
        }

        //println!("input: {:?}", input);

        Ok(Nodes{
            list
        })
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