pub use std::fmt::{Result, Write};

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

