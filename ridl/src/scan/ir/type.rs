use syn::spanned::Spanned;
use crate::model::KSpan;
use super::{Result, err, err_with, SpanScan};

#[derive(Eq,PartialEq)]
pub struct Type {
    pub span: KSpan,
    /// Path expression except actual type name. (last part)
    pub path: String,
    pub name: String,
    /// Parameter or bound Generic arguments.
    pub params: Vec<Type>,
}

/// Please note that `syn::Type` does not provide `span`.
pub fn scan_type(x:&syn::Type) -> Result<Type> {
    use syn::Type::*;
    match x {
        Ptr(x) => scan_type(&x.elem),
        Reference(x) => scan_type(&x.elem),
        Paren(x) => scan_type(&x.elem),
        Array(x) => scan_type(&x.elem),
        Slice(x) => scan_type(&x.elem),
        Path(x) => {
            use syn::PathArguments::*;
            if x.qself.is_some() { return err_with(x, BAD_FORM) };
            let p = &x.path;
            if p.segments.is_empty() { return err_with(x, BAD_FORM) }
            let mut segs = p.segments.clone();
            let last = segs.pop().unwrap().into_value();
            let mut expr = String::new();
            if p.leading_colon.is_some() { 
                expr.push_str("::");
            }
            for seg in segs {
                match seg.arguments {
                    None => (),
                    _ => return err_with(x, "type argument in an intermediate path segment is not supported"),
                }
                expr.push_str(&seg.ident.to_string());
                expr.push_str("::");
            }
            let mut last_params = Vec::new();
            match last.arguments {
                None => (),
                AngleBracketed(generic) => {
                    for arg in generic.args {
                        use syn::GenericArgument::*;
                        match &arg {
                            Lifetime(x) => continue,
                            Type(x) => last_params.push(scan_type(x)?),
                            Binding(x) => return err_with(x, "binding generic parameter is not supported"),
                            Constraint(x) => return err_with(x, "constraint generic parameter is not supported"),
                            Const(x) => return err_with(x, "const generic parameter is not supported"),
                        }
                    }
                },
                Parenthesized(_) => return err_with(x, "unsupported generic parameter form"),
            }
            Ok(Type {
                span: x.span().scan(),
                path: expr,
                name: last.ident.to_string(),
                params: last_params,
            })
        },
        BareFn(x) => err_with(x, BAD_FORM),
        Group(x) => err_with(x, BAD_FORM),
        ImplTrait(x) => err_with(x, BAD_FORM),
        Infer(x) => err_with(x, BAD_FORM),
        Macro(x) => err_with(x, BAD_FORM),
        Never(x) => err_with(x, BAD_FORM),
        TraitObject(x) => err_with(x, BAD_FORM),
        Tuple(x) => err_with(x, BAD_FORM),
        Verbatim(x) => err_with(x, BAD_FORM),
        _ => err_with(x, "unknown type case")
    }    
}

const BAD_FORM: &'static str = "bad/unsupported type form";

#[cfg(test)]
mod test {
    use quote::quote;
    use super::scan_type;
    
    #[test]
    fn case1() {
        let a = quote! {
            std::collections::HashMap<u32, std::path::PathBuf>
        };
        let b: syn::Type = syn::parse2(a).unwrap();
        let c = scan_type(&b).unwrap();
        assert_eq!(c.name, "HashMap");
        assert_eq!(c.params.len(), 2);
        assert_eq!(c.params[0].name, "u32");
        assert_eq!(c.params[1].name, "PathBuf");
    }

    #[test]
    #[should_panic]
    fn case2() {
        let a = quote! {
            std::collections<a>::HashMap<u32, std::path::PathBuf>
        };
        let b: syn::Type = syn::parse2(a).unwrap();
        let c = scan_type(&b).unwrap();
        assert_eq!(c.name, "HashMap");
        assert_eq!(c.params.len(), 2);
        assert_eq!(c.params[0].name, "u32");
        assert_eq!(c.params[1].name, "PathBuf");
    }
}









