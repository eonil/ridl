use super::{Type, Attr, Unknown, Result, Fn, unknown, err, err_with, scan_type, scan_attrs};

/// Simplified representation of a trait.
/// - A trait can contain several items.
/// - A trait can define functions.
pub struct Trait {
    pub items: Vec<TraitItem>,
}
pub enum TraitItem {
    Fn(Fn),
    Unknown(Unknown),
}

pub fn scan_trait(x:&syn::ItemTrait) -> Result<Trait> {
    use crate::model::log::*;
    Ok(Trait {
        items: x.items.map_collect_result(scan_trait_item)?,
    })
}

fn scan_trait_item(x:&syn::TraitItem) -> Result<TraitItem> {
    match x {
        syn::TraitItem::Const(x) => Ok(TraitItem::Unknown(unknown(x, "const item in trait is not supported"))),
        syn::TraitItem::Method(x) => Ok(TraitItem::Fn(scan_trait_fn(x)?)),
        syn::TraitItem::Type(x) => Ok(TraitItem::Unknown(unknown(x, "type item in trait is not supported"))),
        syn::TraitItem::Macro(x) => Ok(TraitItem::Unknown(unknown(x, "macro item in trait is not supported"))),
        syn::TraitItem::Verbatim(x) => Ok(TraitItem::Unknown(unknown(x, "verbatim item in trait is not supported"))),
        _ => Ok(TraitItem::Unknown(unknown(x, "unknown item in trait is not supported")))
    }
}

fn scan_trait_fn(x:&syn::TraitItemMethod) -> Result<Fn> {
    if !x.sig.generics.params.is_empty() { return err_with(x, "generic function item in trait is not supported") }
    if x.sig.variadic.is_some() { return err_with(x, "variadic function item in trait is not supported") }
    let mut input = Vec::new();
    for arg in &x.sig.inputs {
        match arg {
            syn::FnArg::Receiver(_) => continue,
            syn::FnArg::Typed(x) => input.push(scan_type(&x.ty)?),
        }
    }
    Ok(Fn {
        attrs: scan_attrs(&x.attrs)?,
        name: x.sig.ident.to_string(),
        input: input,
        output: if let syn::ReturnType::Type(_,x) = &x.sig.output { Some(scan_type(&x)?) } else { None },
    })
}
