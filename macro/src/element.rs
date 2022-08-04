//use proc_macro;
use proc_macro2::{/*TokenTree, Spacing, Span, Punct,*/ TokenStream, Ident};
use quote::{quote, ToTokens};
use syn::{Block, Token, Result};
use syn::parse::{Parse, ParseStream};
use std::collections::BTreeMap;
use proc_macro_error::abort;

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
                //panic!("Closing tag '{}' dont match '{}'", closing_tag.name, tag.name);
                //panic!("Closing tag '{}' dont match '{}'", closing_tag.name, tag.name);
                /*
                syn::Error::new(input.span(),
                    format!("Closing tag '{}' dont match '{}'", closing_tag.name, tag.name)
                ).to_compile_error();
                */

                //quote_spanned! {
                //    input.span() =>
                //let ty: syn::Type = syn::parse(input).unwrap();
                abort!(span, format!("Closing tag '{}' is missing", tag.name));
                //};
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
        let mut properties:Vec<TokenStream> = vec![];
        let el = if self.is_custom_element(){
            let name = &self.tag.name;
            quote!(#name {})
        }else{
            let attributes:BTreeMap<String, String> = BTreeMap::new();

            for (key, value) in attributes.iter() {
                properties.push(quote!(
                    map.insert(#key, #value);
                ));
            }

            /*
            (1, 2)
            ((1, 2), 3)
            ((1, 2), (3, 4))
            (((1, 2), (3, 4)), 5)
            */

            let children = match &self.children{
                Some(nodes)=>{
                    if nodes.list.len() == 1{
                        let node = &nodes.list[0];
                        quote!{children:Some(#node)}
                    }else{
                        /*
                        let mut list = nodes.list.iter()
                            .map(|item| quote!{#item});
                        let first = list.next();

                        println!("first: {:?}", first);
                        */

                        let mut group = vec![];
                        let list:Vec<TokenStream> = nodes.list.iter()
                                    .map(|item| quote!{#item})
                                    .collect();
                        //let count = 6;
                        for chunk in list.chunks(10){
                            group.push(quote!{ ( #(#chunk),* ) } );
                            if group.len() == 10{
                                let combined = quote!{ ( #(#group),* ) };
                                group = vec![];
                                group.push(combined);
                            }
                        }

                        //println!("\n =======> group: {:?}", group);
                        
                        let children = quote!{(#(#group),*)};

                        /*
                        let mut list = nodes.list.iter()
                            .map(|item| quote!{#item});
                        let first = list.next();
                        let second = list.next();
                        let children = list.fold(
                            quote!{(#first, #second)},
                            |last, item| quote!{(#last, #item)}
                        );
                        //println!("children: {:?}", children);
                        */
                        quote!{children:Some(#children)}
                    }
                }
                None=>{
                    quote!(children:Option::<Vec<()>>::None)
                }
            };
            let tag = self.tag.name.to_string();
            quote!{
                flow_html::HtmlElement {
                    tag:#tag,
                    attributes:{
                        let mut map = std::collections::BTreeMap::new();
                        #(#properties)*
                        map
                    },
                    #children
                }
            }
        };

        el.to_tokens(tokens);
    }
}

pub struct OpeningTag{
    pub name:Ident,
    pub self_closing:bool
}

impl Parse for OpeningTag{
    fn parse(input: ParseStream) -> Result<Self> {
        input.parse::<Token![<]>()?;
        let name = input.parse::<Ident>()?;
        //TODO read attributes

        let mut self_closing = false;
        if input.peek(Token![/]){
            input.parse::<Token![/]>()?;
            self_closing = true;
        }
        input.parse::<Token![>]>()?;
        Ok(Self{
            name,
            self_closing
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