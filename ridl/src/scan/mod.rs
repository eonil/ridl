mod span;
mod ty;
mod ir;
mod attr;
mod err;

use std::string::ToString;
use syn::spanned::Spanned;
use extend::ext;

use crate::prelude::*;
use crate::model::*;
use crate::model::log::*;
use span::SpanScan;
use attr::VecAttrScan;
use ty::TypeScan;

pub fn scan(x: &syn::File) -> Result<KMod> {
    x.scan()
}

#[ext(name=FileScan)]
impl syn::File {
    fn scan(&self) -> Result<KMod> {
        Ok(KMod {
            span: KSpan::default(),
            name: String::new(),
            items: self.items.map_collect_result(syn::Item::scan)?,
            comment: self.attrs.scan_doc_comment()?,
        })
    }
}

#[ext(name=ItemScan)]
impl syn::Item {
    fn scan(&self) -> Result<KItem> {
        use syn::Item::*;
        match self {
            Mod(x) => x.scan(),
            Type(x) => x.scan(),
            Enum(x) => x.scan(),
            Struct(x) => x.scan(),
            // Trait(x)
            // Union(x) 
            _ => err(&self, "unsupported item")
        }
    }
}

#[ext(name=ItemModScan)]
impl syn::ItemMod {
    fn scan(&self) -> Result<KItem> {
        let xs = match &self.content {
            None => Vec::<KItem>::new(),
            Some(x) => x.1.map_collect_result(syn::Item::scan)?,
        };
        Ok(KItem::Mod(KMod {
            span: self.ident.span().scan(),
            name: self.ident.to_string(),
            comment: self.attrs.scan_doc_comment()?,
            items: xs,
        }))
    }
}

#[ext(name=ItemTypeScan)]
impl syn::ItemType {
    /// Scans a type-alias.
    /// Maps to a new-type in RIDL schema.
    fn scan(&self) -> Result<KItem> {
        if !self.generics.params.is_empty() { return err(&self.generics, "generic parameter is not supported") }
        println!("{:#?}", &*self);
        match &*self.ty {
            // syn::Type::TraitObject(_) => {
            //     let form = self.ty.scan_func_type_form()?;
            //     Ok(KItem::Func(KFuncType {
            //         span: self.span().scan(),
            //         name: self.ident.to_string(),
            //         attrs: self.attrs.scan()?,
            //         comment: self.attrs.scan_doc_comment()?,
            //         input: KTypeRef::Def(KItemPath { span: self.ty.span().scan(), name: form.input_type_name.clone() }),
            //         output: KTypeRef::Def(KItemPath { span: self.ty.span().scan(), name: form.output_type_name.clone() }),
            //     }))
            // },
            syn::Type::BareFn(x) => {
                if x.inputs.len() == 1 {} else { return err(&x.inputs, "only 1 input argument is supported") }
                let input_ty = &x.inputs[0].ty;
                let output_ty = if let syn::ReturnType::Type(_,ty) = &x.output { ty } else { return err(&x.output, "output type must be specified explicitly") };
                Ok(KItem::Func(KFuncType {
                    span: self.span().scan(),
                    name: self.ident.to_string(),
                    attrs: self.attrs.scan()?,
                    comment: self.attrs.scan_doc_comment()?,
                    input: input_ty.scan_as_single_ref()?,
                    output: output_ty.scan_as_single_ref()?,
                }))
            },
            _ => Ok(KItem::New(KNewType {
                span: self.span().scan(),
                name: self.ident.to_string(),
                attrs: self.attrs.scan()?,
                comment: self.attrs.scan_doc_comment()?,
                origin: (*self.ty).scan_as_single_ref()?,
            })),
        }
    }
}

#[ext(name=ItemEnumScan)]
impl syn::ItemEnum {
    fn scan(&self) -> Result<KItem> {
        if self.generics.params.len() > 0 { return err(&self.generics, "generic parameter is not supported") }
        if self.has_no_variant_with_payload() {
            // Constant set.
            Ok(KItem::Enum(KEnumType {
                span: self.span().scan(),
                name: self.ident.to_string(),
                comment: self.attrs.scan_doc_comment()?,
                cases: self.variants.iter().map_collect_result(syn::Variant::scan_enum_type_case)?,
            }))
        }
        else {
            // Sum-type.
            Ok(KItem::Sum(KSumType {
                span: self.span().scan(),
                name: self.ident.to_string(),
                serialization: KSumTypeSerializationForm::NameBased,
                attrs: self.attrs.scan()?,
                comment: self.attrs.scan_doc_comment()?,
                variants: self.variants.iter().map_collect_result(syn::Variant::scan_sum_type_variant)?,
            }))
        }
    }
    fn has_no_variant_with_payload(&self) -> bool {
        for x in self.variants.iter() {
            if x.fields.len() > 0 { return false }
        }
        true
    }
}
#[ext(name=VariantScan)]
impl syn::Variant {
    fn scan_enum_type_case(&self) -> Result<KEnumTypeCase> {
        Ok(KEnumTypeCase {
            span: self.span().scan(),
            name: self.ident.to_string(),
            comment: self.attrs.scan_doc_comment()?,
        })
    }
    fn scan_sum_type_variant(&self) -> Result<KSumTypeVariant> {
        use syn::Fields::*;
        if self.fields.len() != 1 { return err(&self.fields, "only single variant field is supported (make an explicitly named struct to store multiple fields)") }
        let unnamed_fields = match &self.fields {
            Named(_) => return err(&self.fields, "only unnamed field is supported (no support for named field)"),
            Unnamed(x) => &x.unnamed,
            Unit => return err(&self.fields, "all sum-type variant must have a data")
        };
        if unnamed_fields.len() != 1 { return err(&unnamed_fields, "only single field is supported in sum-type variant") }
        let first_unnamed_field = &unnamed_fields[0];
        Ok(KSumTypeVariant {
            span: self.span().scan(),
            name: self.ident.to_string(),
            attrs: self.attrs.scan()?,
            comment: self.attrs.scan_doc_comment()?,
            content: first_unnamed_field.ty.scan()?,
        })
    }
}

#[ext(name=ItemStructScan)]
impl syn::ItemStruct {
    fn scan(&self) -> Result<KItem> {
        use syn::Fields::*;
        if self.generics.params.len() > 0 { return err(&self.generics, "generic parameter is not supported") }
        match &self.fields {
            Unnamed(_) => return err(&self.fields, "only named fields are supported in struct (no support for unnamed fields)"),
            Named(_) => (),
            Unit => (),
        }
        Ok(KItem::Prod(KProdType {
            span: self.span().scan(),
            name: self.ident.to_string(),
            attrs: self.attrs.scan()?,
            comment: self.attrs.scan_doc_comment()?,
            fields: self.fields.iter().map_collect_result(syn::Field::scan_prod_type_field)?,
        }))
    }
}
#[ext(name=FieldScan)]
impl syn::Field {
    fn scan_prod_type_field(&self) -> Result<KProdTypeField> {
        if self.ident == None { return err(&self, "only named fields are supported in prod-type") }
        let ident = match &self.ident {
            None => return err(&self, "only named fields are supported in prod-type"),
            Some(x) => x,
        };
        Ok(KProdTypeField {
            span: self.span().scan(),
            name: ident.to_string(),
            attrs: self.attrs.scan()?,
            comment: self.attrs.scan_doc_comment()?,
            content: self.ty.scan()?,
        })
    }
}

// #[ext(name=ItemTraitScan)]
// impl syn::ItemTrait {
//     fn scan(&self) -> KType {
        
//     }
// }

// #[ext(name=ItemUnionScan)]
// impl syn::ItemUnion {
//     fn scan(&self) -> KType {
        
//     }
// }



















#[ext(name=VecScanDocComment)]
impl Vec<syn::Attribute> {
    fn scan_doc_comment(&self) -> Result<String> {
        let mut z = String::new();
        let mut f = false;
        for x in self.iter() {
            z.push_str(&x.scan_doc_comment()?);
            z.push_str("\n");
            f = true;
        }
        if f { z.pop(); }
        Ok(z)
    }
}
#[ext(name=AttrScanDocComment)]
impl syn::Attribute {
    fn scan_doc_comment(&self) -> Result<String> {
        if self.path.ident_string_or_default() != "doc" { return Ok(String::new()) }
        const BAD_FORM: &str = "unexpected comment form";
        let m = if let Ok(m) = self.parse_meta() { m } else { return err(&self, BAD_FORM) };
        let kv = if let syn::Meta::NameValue(kv) = m { kv } else { return err(&m, BAD_FORM) };
        if kv.path.ident_string_or_default() != "doc" { return err(&kv, BAD_FORM) }
        let s = if let syn::Lit::Str(s) = kv.lit { s } else { return err(&kv, BAD_FORM) };
        Ok(s.value().trim().to_string())
    }
}














#[ext(name=PathUtil)]
impl syn::Path {
    fn ident_string_or_default(&self) -> String {
        self.get_ident().map(|x| x.to_string()).unwrap_or_default()
    }
}

fn err<T: syn::spanned::Spanned,X>(spanned:&T, message:&str) -> Result<X> {
    let log = Log { span: spanned.span().scan(), message: PString::new(message.to_string()) };
    Err(ErrorLogs(PVec::from(vec![log])))
}
