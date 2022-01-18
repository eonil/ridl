use crate::prelude::*;
use crate::model::{KAttrs,KAttrREST};
use crate::model::log::*;
use super::{ir, err, err_with};

#[ext(name=VecAttrScan)]
pub(super) impl Vec<syn::Attribute> {
    fn scan(&self) -> Result<KAttrs> {
        const BAD_FORM_ERR: &'static str = "badly formed attribute";
        let mut x = KAttrs::default();
        for a in self.iter() {
            let ir = ir::Attr::try_from(a)?;
            let n = ir.name.as_str();
            let k = ir.params.iter().next().map(ir::AttrParam::key).unwrap_or("");
            let m = match (n,k) {
                ("rest","in") => KAttrREST::MessageIn,
                ("rest","out") => KAttrREST::MessageOut,
                // ("rest", _) => {
                //     if ir.params.len() == 2 {} else { return err(a, BAD_FORM_ERR) }
                //     let k = if let ir::AttrParam::Key(k) = &ir.params[0] { k } else { return err(a, BAD_FORM_ERR) };
                //     let v = if let ir::AttrParam::Value(ir::AttrValue::String(v)) = &ir.params[1] { v } else { return err(a, BAD_FORM_ERR) };
                //     x.rest.push(KAttrREST::FnMethod(k.to_string()));
                //     x.rest.push(KAttrREST::FnPath(v.to_owned()));
                //     continue;
                // },
                ("path","") => KAttrREST::PathParam,
                ("query","") => KAttrREST::QueryParam,
                ("body","") => KAttrREST::BodyParam,
                _ => {
                    let v = ir.params.iter().next().map(ir::AttrParam::value);
                    match (n,v) {
                        ("status",Some(Some(ir::AttrValue::I64(x)))) => KAttrREST::Status(*x),
                        ("status",_) => return err_with(a, BAD_FORM_ERR),
                        ("mime",Some(Some(ir::AttrValue::String(x)))) => KAttrREST::MIME(x.to_owned()),
                        ("mime",_) => return err_with(a, BAD_FORM_ERR),
                        _ => continue,
                    }
                },
            };
            x.rest.push(m);
        }
        Ok(x)
    }
}
impl ir::AttrParam {
    fn key(&self) -> &str {
        if let ir::AttrParam::Key(s) = self { &s } else { "" }
    }
    fn value(&self) -> Option<&ir::AttrValue> {
        if let ir::AttrParam::Value(v) = self { Some(&v) } else { None }
    }
}




#[cfg(test)]
mod test {
    use quote::quote;
    use quote::ToTokens;
    use super::*;

    #[test]
    fn scan() {
        let a = quote! {
            #[rest(in)]
        };
        let b = a.into_token_stream();
        let c = syn::parse::Parser::parse2(syn::Attribute::parse_outer, b).unwrap();
        assert_eq!(c.len(), 1);
        let d = c.scan().unwrap();
        assert_eq!(d, KAttrs {
            rest: vec![
                KAttrREST::MessageIn,
            ],
        });
    }
}