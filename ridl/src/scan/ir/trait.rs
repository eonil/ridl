
use super::{Unknown,Type,Attr};

/// Simplified representation of a trait.
/// - A trait can contain several items.
/// - A trait can define functions.
pub struct Trait {
    pub items: Vec<TraitItem>,
}
pub enum TraitItem {
    Fn(TraitFn),
    Unknown(Unknown),
}
pub type TraitItemName = String;

pub struct TraitFn {
    pub attrs: Vec<Attr>,
    pub name: String,
    pub input: Vec<Type>,
    pub output: Type,
}






// pub fn scan_trait(x:&syn::ItemTrait) -> Trait {
    
// }