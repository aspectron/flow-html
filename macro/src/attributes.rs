use proc_macro2::{TokenStream, Ident, Literal/*, Span*/};
use quote::{quote, ToTokens};
use syn::{
    Block,
    Token,
    Result,
    punctuated::Punctuated,
    parse::{Parse, ParseStream},
    ext::IdentExt
};
use rand::Rng;
//use std::sync::Arc;


pub type AttributeName = Punctuated<Ident, Token![-]>;

pub trait AttributeNameString {
    fn to_string(&self)->String;
}

impl AttributeNameString for AttributeName{
    fn to_string(&self)->String{
        let mut items = self.iter()
            .map(|a| a.to_string());
        let first = items.next().unwrap();
        items.fold(first, |a, b|format!("{}-{}", a, b))
    }
}

pub struct Attributes<'a>{
    list:Vec<Attribute<'a>>
}

impl<'a> Attributes<'a>{
    /*
    pub fn get_names(&self)->Vec<String>{
        let mut list = vec![];
        for attr in &self.list{
            list.push(attr.get_name())
        }
        list
    }
    */
    pub fn empty()->Self{
        Self{list:vec![]}
    }
    pub fn to_properties(&self/*, names:Arc<Vec<String>>*/)->Vec<TokenStream>{
        let mut properties = vec![];
        //let mut used = vec![];
        for attr in &self.list{
            let name = &attr.name;
            let value = match attr.attr_type{
                AttributeType::String=>{
                    if attr.value.is_some(){
                        let value = attr.get_value();
                        quote!(:&#value)
                    }else{
                        quote!(:&#name)
                    }
                }
                _=>{
                    if attr.value.is_some(){
                        let value = attr.get_value();
                        quote!(:#value)
                    }else{
                        quote!()
                    }
                }
            };
            //used.push(name.to_string());
            properties.push(quote!(
                #name #value
            ));
        }
        /*
        println!("used: {:?} , names:{:?}", used, names);
        for name in names.iter(){
            if !used.contains(name){
                let name_ident = Ident::new(name, Span::call_site());
                properties.push(quote!(
                    #name_ident: None
                ));
            }
        }
        */
        properties
    }
    pub fn to_token_stream(&self)->TokenStream{
        let mut attrs = vec![];
        let mut ref_field = quote!(reff:None);
        for attr in &self.list{
            let name = attr.get_name();
            let value = attr.get_value();
            let mut append = true;
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
                AttributeType::Ref=>{
                    ref_field = quote!{reff: Some((#name, #value))};
                    append = false;
                    quote!()
                }
            };
            if append{
                attrs.push(quote!(
                    map.insert(#name, #value);
                ));
            }
        }
        quote!{
            #ref_field,
            attributes:{
                let mut map = std::collections::BTreeMap::new();
                #(#attrs)*
                map
            }
        }.into()
    }
}


pub enum AttributeValue<'a>{
    Block(Block),
    Literal(Literal),
    _Str(&'a str)
}
pub enum AttributeType{
    Bool,
    Str,
    String,
    Ref
}
pub struct Attribute<'a>{
    pub name: AttributeName,
    pub attr_type: AttributeType,
    pub value: Option<AttributeValue<'a>>
}

impl<'a> Attribute<'a>{
    pub fn new(name:AttributeName, attr_type:AttributeType, value:Option<AttributeValue<'a>>)->Attribute<'a>{
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
                match value {
                    AttributeValue::Block(v)=>{
                        (&v.stmts[0]).into_token_stream()
                    }
                    AttributeValue::Literal(v)=>{
                        quote!(#v).into_token_stream()
                    }
                    AttributeValue::_Str(v)=>{
                        quote!(#v).into_token_stream()
                    }
                }
                
            }
            None => {
                match self.attr_type{
                    AttributeType::Ref=>{
                        let mut rng = rand::thread_rng();
                        let code = format!("ref_{}", rng.gen::<u32>());
                        quote!(#code)
                    }
                    _=>{
                        self.name.to_token_stream()
                    }
                }
                
            }
        }
    }
}

impl<'a> Parse for Attribute<'a>{
    fn parse(input: ParseStream) -> Result<Self> {
        let mut attr_type = AttributeType::Str;
        if input.peek(Token![?]){
            input.parse::<Token![?]>()?;
            attr_type = AttributeType::Bool;
        }else if input.peek(Token![&]){
            input.parse::<Token![&]>()?;
            attr_type = AttributeType::String;
        }else if input.peek(Token![@]){
            input.parse::<Token![@]>()?;
            attr_type = AttributeType::Ref;
        }
        
        let name = AttributeName::parse_separated_nonempty_with(input, syn::Ident::parse_any)?;
        if input.peek(Token![=]){
            input.parse::<Token![=]>()?;
            let value;
            if input.peek(syn::token::Brace){
                value = AttributeValue::Block(input.parse::<Block>()?);
            }else{
                //value = AttributeValue::Str("");
                //println!("input: {:#?}", input);
                value = AttributeValue::Literal(input.parse::<Literal>()?);
            }
            return Ok(Attribute::new(name, attr_type, Some(value)));
        }
        Ok(Attribute::new(name, attr_type, None))
    }
}

pub fn parse_attributes<'a>(input: ParseStream)->Result<Attributes<'a>>{
    let mut list = vec![];
    //print!("parse_attributes: {:?}", input);
    while !(input.peek(Token![/]) || input.peek(Token![>])){
        let attribute = input.parse::<Attribute>()?;
        list.push(attribute);
    }

    Ok(Attributes{
        list
    })
}