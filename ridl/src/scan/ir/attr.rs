use serde::{Serialize,Deserialize};
use crate::model::log::ErrorLogs;
use crate::model::log::Result;
use crate::scan::err::ResultConversion;
use crate::scan::err;

/// Reduce Rust attribute syntax to a simpler form.
/// An attribute can be one of these forms.
/// 
/// ```compile_fail
///     #[a]
///     #[a(b,c,d)]
///     #[a("B",222,false)]
///     #[a(b="B",c=222,false)]
/// ```
/// 
/// - Always have a name.
/// - Optional key-value list.
/// - Key or value can be ommited.
/// 
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default,Clone)]
#[derive(Debug)]
pub struct Attr {
    pub name: AttrName,
    pub params: Vec<AttrParam>,
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub enum AttrParam {
    Key(AttrName),
    Value(AttrValue),
    KeyValue(AttrName,AttrValue),
}

pub type AttrName = String;

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub enum AttrValue {
    Bool(bool),
    I64(i64),
    String(String),
}








// impl Attr {
//     fn scan_doc_comment(&self) -> Result<String> {
//         match self {
//             Attr::Map(kvs) => {
//                 if kvs.len() == 1 {} else { return err(self, "not a doc comment") }
//                 if let Some(content) = let kvs.get("doc") { content } else { return err(self, "not a doc comment") }

//             },
//             _ => return err(self, "not a doc comment attribute"),
//         }
//     }
// }










impl ToString for AttrValue {
    fn to_string(&self) -> String {
        use AttrValue::*;
        match self {
            Bool(x) => x.to_string(),
            I64(x) => x.to_string(),
            String(x) => x.to_string(),
        }
    }
}

impl std::convert::TryFrom<&syn::Attribute> for Attr {
    type Error = ErrorLogs;
    fn try_from(a:&syn::Attribute) -> Result<Attr> {
        let m = a.parse_meta().into_scan_result()?;
        use syn::Meta::*;
        let x = match &m {
            Path(x) => Attr { name: scan_non_generic_name(x)?, params: vec![] },
            // e.g. `derive(Copy, Clone)`.
            List(x) => {
                let n = scan_non_generic_name(&x.path)?;
                let mut ps = Vec::<AttrParam>::new();
                for sub in x.nested.iter() {
                    let p = match sub {
                        syn::NestedMeta::Meta(Path(x)) => AttrParam::Key(scan_non_generic_name(x)?),
                        syn::NestedMeta::Meta(List(x)) => return err(x, "unsupported attribute form"),
                        syn::NestedMeta::Meta(NameValue(x)) => AttrParam::KeyValue(scan_non_generic_name(&x.path)?, scan_value(&x.lit)?),
                        syn::NestedMeta::Lit(x) => AttrParam::Value(scan_value(x)?),
                    };
                    ps.push(p);
                }
                Attr { name: n, params: ps }
            },
            NameValue(x) => Attr { name: "".to_owned(), params: vec![AttrParam::KeyValue(scan_non_generic_name(&x.path)?, scan_value(&x.lit)?)] },
        };
        Ok(x)
    }
}
fn scan_non_generic_name(p:&syn::Path) -> Result<String> {
    if p.segments.is_empty() { return err(&p, "zero-length path segment is not supported") }
    let seg = p.segments.last().unwrap();
    if let syn::PathArguments::None = seg.arguments {} else { return err(&seg, "generic parameter is not supported") }
    Ok(seg.ident.to_string())
}
fn scan_value(x:&syn::Lit) -> Result<AttrValue> {
    let v = match x {
        syn::Lit::Bool(b) => AttrValue::Bool(b.value),
        syn::Lit::Int(u) => AttrValue::I64(u.base10_parse::<i64>().into_scan_result()?),
        syn::Lit::Str(s) => AttrValue::String(s.value()),
        _ => return err(&x, "unsupported literal form")
    };
    Ok(v)
}






#[cfg(test)]
mod test {
    use quote::quote;
    use quote::ToTokens;
    use crate::prelude::*;
    use super::*;


    #[test]
    fn scan_attr_1() {
        let a = quote! {
            #[abc]
        };
        let b = a.into_token_stream();
        let c = syn::parse::Parser::parse2(syn::Attribute::parse_outer, b).unwrap();
        assert_eq!(c.len(), 1);
        let d = &c[0];
        let e = Attr::try_from(d).unwrap();
        assert_eq!(e, Attr { name: "abc".to_owned(), params: vec![] });
    }

    #[test]
    fn scan_attr_2() {
        let a = quote! {
            #[abc(d, e)]
        };
        let b = a.into_token_stream();
        let c = syn::parse::Parser::parse2(syn::Attribute::parse_outer, b).unwrap();
        assert_eq!(c.len(), 1);
        let d = &c[0];
        let e = Attr::try_from(d).unwrap();
        assert_eq!(e, Attr { name: "abc".to_owned(), params: vec![
            AttrParam::Key("d".to_owned()),
            AttrParam::Key("e".to_owned()),
        ] });
    }

    #[test]
    fn scan_attr_3() {
        let a = quote! {
            #[abc(a=10, c="d", e=true)]
        };
        let b = a.into_token_stream();
        let c = syn::parse::Parser::parse2(syn::Attribute::parse_outer, b).unwrap();
        assert_eq!(c.len(), 1);
        let d = &c[0];
        let e = Attr::try_from(d).unwrap();
        let mut kvs = Map::<String,AttrValue>::new();
        kvs.insert("a".to_owned(), AttrValue::I64(10));
        kvs.insert("c".to_owned(), AttrValue::String("d".to_owned()));
        kvs.insert("e".to_owned(), AttrValue::Bool(true));
        assert_eq!(e, Attr { name: "abc".to_owned(), params: vec![
            AttrParam::KeyValue("a".to_owned(), AttrValue::I64(10)),
            AttrParam::KeyValue("c".to_owned(), AttrValue::String("d".to_owned())),
            AttrParam::KeyValue("e".to_owned(), AttrValue::Bool(true)),
        ] });
    }
}