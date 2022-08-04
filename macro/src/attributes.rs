use proc_macro2::{TokenStream, TokenTree, Ident};
use quote::{quote, ToTokens};
use syn::{Block, Token, Result, punctuated::Punctuated};
use syn::parse::{Parse, ParseStream};
//use proc_macro_error::abort;
use syn::ext::IdentExt;

pub type AttributeName = Punctuated<Ident, syn::Token![-]>;

pub struct Attributes{
    list:Vec<Attribute>
}

impl Attributes{
    pub fn to_token_stream(&self)->TokenStream{
        let mut attrs = vec![];
        for attr in &self.list{
            let name = attr.get_name();
            let value = attr.get_value();
            let value = match attr.attr_type{
                AttributeType::Bool=>{
                    quote!{flow_html::AttributeValue::Bool(#value)}
                }
                AttributeType::Str=>{
                    quote!{flow_html::AttributeValue::Str(#value)}
                }
                AttributeType::String=>{
                    quote!{flow_html::AttributeValue::Str(&#value)}
                }
            };
            /*
            let v = value.clone().into_iter().next().unwrap();
            match &v{
                TokenTree::Ident(a)=>{
                    println!("\n###### Ident: {:?}", a);
                }
                TokenTree::Group(a)=>{
                    println!("\n###### Group: {:?}", a);
                }
                TokenTree::Punct(a)=>{
                    println!("\n###### Punct: {:?}", a);
                }
                TokenTree::Literal(a)=>{
                    println!("\n###### Literal: {:?}", a);
                }
            }
            */

            attrs.push(quote!(
                map.insert(#name, #value);
            ));
        }
        quote!{
            attributes:{
                let mut map = std::collections::BTreeMap::new();
                #(#attrs)*
                map
            }
        }.into()
    }
}


pub enum AttributeType{
    Bool,
    Str,
    String
}
pub struct Attribute{
    pub name: AttributeName,
    pub attr_type: AttributeType,
    pub value: Option<Block>
}

impl Attribute{
    pub fn new(name:AttributeName, attr_type:AttributeType, value:Option<Block>)->Self{
        Self { name, attr_type, value }
    }
    pub fn get_name(&self)->String{
        let mut items = self.name.iter()
            .map(|a| a.to_string());
        let first = items.next().unwrap();
        items.fold(first, |a, b|format!("{}-{}", a, b))
    }

    pub fn get_value(&self)->TokenStream{
        match &self.value {
            Some(value)=>{
                (&value.stmts[0]).into_token_stream()
            }
            None => {
                self.name.to_token_stream()
            }
        }
    }
}

impl Parse for Attribute{
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attr_type = AttributeType::Str;
        if input.peek(Token![?]){
            input.parse::<Token![?]>()?;
            attr_type = AttributeType::Bool;
        }else if input.peek(Token![&]){
            input.parse::<Token![&]>()?;
            attr_type = AttributeType::String;
        }
        let name = AttributeName::parse_separated_nonempty_with(input, syn::Ident::parse_any)?;
        if input.peek(Token![=]){
            input.parse::<Token![=]>()?;
            let value = input.parse::<Block>()?;
            return Ok(Attribute::new(name, attr_type, Some(value)));
        }
        Ok(Attribute::new(name, attr_type, None))
    }
}

pub fn parse_attributes(input: ParseStream)->Result<Attributes>{
    let mut list = vec![];
    print!("parse_attributes: {:?}", input);
    while !(input.peek(Token![/]) || input.peek(Token![>])){
        let attribute = input.parse::<Attribute>()?;
        list.push(attribute);
    }

    Ok(Attributes{
        list
    })
}