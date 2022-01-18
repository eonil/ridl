use crate::prelude::*;
use crate::model::*;
use super::{Result, err, ir};

/// Scans a simplified type expression for serialization schema.
/// - `Path` becomes to a reference to an explicit type.
///   - `Path` to `Option` type will be recognized as RIDL optional.
/// - `References`, `Paren`, `Ptr` will be stripped away.
/// - `Array`, `Slice` becomes a RIDL array.
/// - Everything else is not supported and returns an `Err`.
pub fn scan(x:&syn::Type) -> Result<KType> {
    scan_type(&ir::scan_type(x)?)
}

fn scan_type(x:&ir::Type) -> Result<KType> {
    match x.name.as_str() {
        "Vec" => {
            if x.params.len() == 1 {} else { return err(x.span, "`Vec` type must have one parameter") }
            let p = x.params.first().unwrap();
            Ok(KType::Vector(scan_scalar_type(&p)?))
        },
        "Option" => {
            if x.params.len() == 1 {} else { return err(x.span, "`Option` type must have one parameter") }
            let p = x.params.first().unwrap();
            Ok(KType::Option(scan_scalar_type(&p)?))
        },
        _ => {
            Ok(KType::Scalar(scan_scalar_type(x)?))
        }
    }
}

fn scan_scalar_type(x:&ir::Type) -> Result<KScalarType> {
    if !x.params.is_empty() { return err(x.span, "scalar type with generic parameter is not supported") }
    match x.name.as_str() {
        "()" => Ok(KScalarType::Unit),
        "bool" => Ok(KScalarType::Prim(KPrimType::Bool)),
        "i32" => Ok(KScalarType::Prim(KPrimType::I32)),
        "f64" => Ok(KScalarType::Prim(KPrimType::F64)),
        "str" => Ok(KScalarType::Prim(KPrimType::String)),
        "String" => Ok(KScalarType::Prim(KPrimType::String)),
        _ => Ok(KScalarType::Def(x.name.clone())),
    }
}
