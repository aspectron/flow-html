pub use std::fmt::{Result, Write};

pub trait Render:Sized{
    fn html(self)->String{
        let mut buf = String::from("");
        self.render(&mut buf).unwrap();
        buf
    }
    fn render<W:Write>(self, _w:&mut W)->Result;
}

impl Render for () {
    fn render<W:Write>(self, _w:&mut W)->Result{
        Ok(())
    }
}

impl Render for &str {
    fn render<W:Write>(self, w:&mut W)->Result{
        write!(w, "{}", self)
    }
}
impl Render for usize {
    fn render<W:Write>(self, w:&mut W)->Result{
        write!(w, "{}", self)
    }
}
impl Render for f32 {
    fn render<W:Write>(self, w:&mut W)->Result{
        write!(w, "{}", self)
    }
}
impl Render for f64 {
    fn render<W:Write>(self, w:&mut W)->Result{
        write!(w, "{}", self)
    }
}

impl Render for String {
    fn render<W:Write>(self, w:&mut W)->Result{
        write!(w, "{}", self)
    }
}


impl Render for bool {
    fn render<W:Write>(self, w:&mut W)->Result{
        write!(w, "{}", self)
    }
}

impl<A:Render, B:Render> Render for (A, B) {
    fn render<W:Write>(self, w:&mut W)->Result{
        self.0.render(w)?;
        self.1.render(w)
    }
}
