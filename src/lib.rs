mod star;
mod constellation;
pub use star::{Star,Equatorial};
pub use constellation::{Constellation};

#[derive(PartialEq,Eq,Clone,Default,Debug)]
pub struct Name{
    pub english: Option<String>,
    pub japanese: Option<String>,
}

impl Name {
    pub fn new(english:Option<String>,japanese:Option<String>) -> Self {
        Self{
            english:english,japanese:japanese
        }
    }
}
