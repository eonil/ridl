use super::{Attr, Type, Result, err, err_with, scan_attrs, scan_type};

/// - This does not scan receiver, argument names and argument attributes.
pub struct Fn {
    pub attrs: Vec<Attr>,
    pub name: String,
    pub input: Vec<Type>,
    pub output: Option<Type>,
}

pub fn scan_fn(x:&syn::ItemFn) -> Result<Fn> {
    if !x.sig.generics.params.is_empty() { return err_with(x, "generic function item is not supported") }
    if x.sig.variadic.is_some() { return err_with(x, "variadic function item is not supported") }
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
