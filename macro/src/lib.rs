use proc_macro::TokenStream;
use syn::{
    DeriveInput,
    parse_macro_input,
    parse::{ParseStream,Parse},
    ext::IdentExt,
    Meta, NestedMeta
};
use quote::quote;
mod element;
mod attributes;
use element::Element;
use attributes::{AttributeName, AttributeNameString};
use proc_macro_error::proc_macro_error;


#[proc_macro]
#[proc_macro_error]
pub fn html(input: TokenStream) -> TokenStream {
    let element =  parse_macro_input!(input as Element);
    println!("\n====>html element: {}", quote!{#element}.to_string());
    quote!{#element}.into()
}

struct RenderableAttributes {
    pub tag_name : String
}

impl Parse for RenderableAttributes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let tag_name = AttributeName::parse_separated_nonempty_with(input, syn::Ident::parse_any)?;
        Ok(RenderableAttributes{
            tag_name : tag_name.to_string()
        })
    }
}

#[proc_macro_attribute]
//#[proc_macro_derive(Renderable)]
#[proc_macro_error]
pub fn renderable(attr: TokenStream, item: TokenStream) -> TokenStream {
    let renderable_attr = parse_macro_input!(attr as RenderableAttributes);
    let tag_name = renderable_attr.tag_name;
    let format_str = format!("<{} {{}}>{{}}</{}>", tag_name, tag_name);
    //println!("renderable_attr: {:?}", tag_name);
    //let def:proc_macro2::TokenStream = item.clone().into();
    let ast = parse_macro_input!(item as DeriveInput);
    let struct_name = &ast.ident;
    let struct_params = &ast.generics;
    let generics_only = ast.generics.clone();
    let where_clause = match generics_only.where_clause.clone() {
        Some(where_clause) => quote!{ #where_clause },
        None => quote!{}
    };

    let mut field_visibility_vec = vec![];
    let mut field_ident_vec = vec![];
    let mut field_type_vec = vec![];
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(ref fields),
        ..
    }) = ast.data
    {
        for field in fields.named.iter() {
            //let field_name: syn::Ident = field.ident.as_ref().unwrap().clone();
            field_ident_vec.push(&field.ident);
            field_visibility_vec.push(&field.vis);
            field_type_vec.push(&field.ty);
            //let name: String = field_name.to_string();
            //println!("\n\n----->name: {}, \ntype: {:?}, \nattrs: {:?}", name, field.ty, field.attrs);
            let mut attrs:Vec<_> = field.attrs.iter().collect();
            if attrs.len()>0{
                let attr  = attrs.remove(0);
                let meta = attr.parse_meta().unwrap();
                
                match meta{
                    Meta::List(list)=>{
                        //println!("meta-list: {:#?}", list);
                        //println!("meta-list.path: {:#?}", list.path.get_ident().unwrap().to_string());
                        //println!("nested: {:?}", list.nested);
                        for item in list.nested.iter(){
                            if let NestedMeta::Meta(m) = item{
                                if let Meta::NameValue(name_value) = m{
                                    let key = name_value.path.get_ident().unwrap().to_string();
                                    let value:String = match &name_value.lit{
                                        syn::Lit::Int(v)=>v.to_string(),
                                        syn::Lit::Str(v)=>v.value(),
                                        syn::Lit::Bool(v)=>v.value().to_string(),
                                        _=>"".to_string()
                                    };
                                    println!("key: {}, value: {}", key, value);
                                }
                            }
                        }

                    }
                    _=>{
                        
                    }
                }
            }
        }
    }

    let ts = quote!(
        #[derive(Debug)]
        pub struct #struct_name #struct_params #where_clause {
            #( #field_visibility_vec #field_ident_vec : #field_type_vec ),*
        }

        impl #struct_params flow_html::Render for #struct_name #struct_params #where_clause {
            fn render<W:core::fmt::Write>(self, w:&mut W)->core::fmt::Result{
                let this = &self;
                let attr = this.get_attributes();
                let children = this.get_children();
                write!(w, #format_str, attr, children)
            }
        }
        impl #struct_params flow_html::ElementDefaults for #struct_name #struct_params #where_clause {
            fn _get_attributes(&self)->String{
                format!("class=\"abc\"")
            }
            fn _get_children(&self)->String{
                "".to_string()
            }
        }
    );
    println!("###### render:ts: {}", ts.to_string());
    ts.into()
} 