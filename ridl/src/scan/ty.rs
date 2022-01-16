use crate::prelude::*;
use crate::model::*;
use super::{Result, err, span::SpanScan};

#[ext(name=TypeScan)]
pub impl syn::Type {
    /// Scans a simplified type expression for serialization schema.
    /// - `Path` becomes to a reference to an explicit type.
    ///   - `Path` to `Option` type will be recognized as RIDL optional.
    /// - `References`, `Paren`, `Ptr` will be stripped away.
    /// - `Array`, `Slice` becomes a RIDL array.
    /// - Everything else is not supported and returns an `Err`.
    fn scan(&self) -> Result<KContentStorage> {
        match self {
            syn::Type::Paren(x) => return (*x.elem).scan(),
            syn::Type::Ptr(x) => return (*x.elem).scan(),
            syn::Type::Reference(x) => return (*x.elem).scan(),
            _ => (),
        }
        if let Ok(x) = self.scan_as_array() { return Ok(x) }
        if let Ok(x) = self.scan_as_optional() { return Ok(x) }
        if let Ok(x) = self.scan_as_single_ref() { return Ok(KContentStorage {
            optional: false,
            array: false,
            r#type: x,
        }) }
        err(&self, "unsupported type pattern (RIDL supports only certain shape of limited type pattern)")
    }
    fn scan_as_array(&self) -> Result<KContentStorage> {
        match self {
            syn::Type::Array(x) => {
                let elem = (*x.elem).scan_as_single_ref()?;
                Ok(KContentStorage {
                    optional: false,
                    array: true,
                    r#type: elem,
                })
            },
            syn::Type::Slice(x) => {
                let elem = (*x.elem).scan_as_single_ref()?;
                Ok(KContentStorage {
                    optional: false,
                    array: true,
                    r#type: elem,
                })
            },
            syn::Type::Path(x) => {
                let segs = &x.path.segments;
                if segs.len() == 1 {} else { return err(&x, "") }
                let seg = segs.last().unwrap();
                if seg.ident.to_string() == "Vec" {} else { return err(&seg, "this is not a `Vec`") }
                let args = match &seg.arguments {
                    syn::PathArguments::None => return err(&seg, "missing generic argument in `Vec`"),
                    syn::PathArguments::Parenthesized(_) => return err(&seg, "bad generic argument in `Vec`"),
                    syn::PathArguments::AngleBracketed(xx) => &xx.args,
                };
                for arg in args {
                    let elem = match arg {
                        syn::GenericArgument::Type(xxx) => xxx.scan_as_single_ref()?,
                        _ => continue,
                    };
                    return Ok(KContentStorage {
                        optional: false,
                        array: true,
                        r#type: elem,
                    })
                }
                err(&x, "missing generic argument in `Vec`.")
            },
            _ => err(&self, "this is not an array type"),
        }
    }
    fn scan_as_optional(&self) -> Result<KContentStorage> {
        match self {
            syn::Type::Path(x) => {
                let segs = &x.path.segments;
                if segs.len() == 1 {} else { return err(&x, "") }
                let seg = segs.last().unwrap();
                if seg.ident.to_string() == "Option" {} else { return err(&seg, "this is not an `Option`") }
                let args = match &seg.arguments {
                    syn::PathArguments::None => return err(&seg, "missing generic argument in `Option`"),
                    syn::PathArguments::Parenthesized(_) => return err(&seg, "bad generic argument in `Option`"),
                    syn::PathArguments::AngleBracketed(xx) => &xx.args,
                };
                for arg in args {
                    let elem = match arg {
                        syn::GenericArgument::Type(xxx) => xxx.scan_as_single_ref()?,
                        _ => continue,
                    };
                    return Ok(KContentStorage {
                        optional: true,
                        array: false,
                        r#type: elem,
                    })
                }
                err(&x, "missing generic argument in `Option`.")
            },
            _ => err(&self, "this is not an optional type"),
        }
    }
    fn scan_as_single_ref(&self) -> Result<KTypeRef> {
        match self {
            syn::Type::Paren(x) => (*x.elem).scan_as_single_ref(),
            syn::Type::Ptr(x) => (*x.elem).scan_as_single_ref(),
            syn::Type::Reference(x) => (*x.elem).scan_as_single_ref(),
            syn::Type::Path(x) => {
                use syn::spanned::Spanned;
                let name = x.path.scan_name()?;
                let tyref = match name.as_str() {
                    "bool" => KTypeRef::Prim(KPrimType::Bool),
                    "i32" => KTypeRef::Prim(KPrimType::I32),
                    "i64" => KTypeRef::Prim(KPrimType::I64),
                    "f32" => KTypeRef::Prim(KPrimType::F32),
                    "f64" => KTypeRef::Prim(KPrimType::F64),
                    "str" => KTypeRef::Prim(KPrimType::String),
                    "String" => KTypeRef::Prim(KPrimType::String),
                    _ => KTypeRef::Def(KItemPath {
                        span: self.span().scan(),
                        name: name,
                    }),
                };
                Ok(tyref)
            },
            _ => err(&self, "not a reference to an explicit name"),
        }
    }
}

#[ext(name=PathScan)]
impl syn::Path {
    fn scan_name(&self) -> Result<String> {
        if let Some(ident) = self.get_ident() { return Ok(ident.to_string()) }
        err(self, "not an ident")
    }
    fn scan_ident(&self) -> Result<String> {
        if let Some(s) = self.get_ident().map(ToString::to_string) { return Ok(s) }
        err(self, "")
    }
}

