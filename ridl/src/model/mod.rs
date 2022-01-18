//! Defines model of KCG.
//! 
//! KCG can build its own model by scanning other datastructure schema.
//! For OpenAPI 3.0 Schema, KCG finds certain patterns to build certain types.
//! 
//! All types are `K` prefixed which means "Schema".

pub mod log;
pub mod span;
mod attr;
mod r#type;

pub mod skip;
pub mod rename;
pub mod rest;

use serde_derive::{Serialize, Deserialize};

pub use span::{KSpan, KLineColumn};
pub use attr::{KAttrs, KAttrREST};
pub use r#type::{KType, KScalarType, KPrimType, KTypeName};

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub struct KMod {
    pub span: KSpan,
    pub name: String,
    pub comment: String,
    pub items: Vec<KItem>,
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub enum KItem {
    Mod(KMod),
    New(KNewType),
    Enum(KEnumType),
    Sum(KSumType),
    Prod(KProdType),
    Func(KFuncType),
}
impl KItem {
    pub fn span(&self) -> &KSpan {
        use KItem::*;
        match self {
            Mod(x) => &x.span,
            New(x) => &x.span,
            Enum(x) => &x.span,
            Sum(x) => &x.span,
            Prod(x) => &x.span,
            Func(x) => &x.span,
        }
    }
    pub fn name(&self) -> &str {
        use KItem::*;
        match self {
            Mod(x) => x.name.as_str(),
            New(x) => x.name.as_str(),
            Enum(x) => x.name.as_str(),
            Sum(x) => x.name.as_str(),
            Prod(x) => x.name.as_str(),
            Func(x) => x.name.as_str(),
        }
    }
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Clone)]
#[derive(Debug)]
pub struct KNewType {
    pub span: KSpan,
    pub name: String,
    #[serde(default)]
    #[serde(skip_serializing_if="is_default")]
    pub attrs: KAttrs,
    pub comment: String,
    pub origin: KType,
}

/// Finite constant value set.
/// Rust code-gen provides automatic enum/string conversion.
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Clone)]
#[derive(Debug)]
pub struct KEnumType {
    pub span: KSpan,
    pub name: String,
    pub comment: String,
    pub cases: Vec<KEnumTypeCase>,
}
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Clone)]
#[derive(Debug)]
pub struct KEnumTypeCase {
    pub span: KSpan,
    pub name: String,
    pub comment: String,
    // /// Underlying value for this case.
    // pub value: Option<i32>,
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Clone)]
#[derive(Debug)]
pub struct KSumType {
    pub span: KSpan,
    pub name: String,
    pub comment: String,
    pub serialization: KSumTypeSerializationForm,
    #[serde(default)]
    #[serde(skip_serializing_if="is_default")]
    pub attrs: KAttrs,
    pub variants: Vec<KSumTypeVariant>,
}
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub enum KSumTypeSerializationForm {
    /// Default serialization form of Rust(serde), Swift(Codable) and Smithy.
    /// Discriminant is a metadata of its content.
    NameBased,
    /// **NOT SUPPORTED FOR NOW**
    /// =========================
    /// Default serialization form of TypeScript and GraphQL.
    /// Discriminant is embedded in content.
    /// Therefore, content must be a prod-type.
    /// In this case, name of discriminant property is required to eliminate ambiguity.
    /// - `discriminant`: name of discriminant property embedded in content.
    ///     This can be set by putting `#[ridl::form(tag="")]` on a sum-type definition.
    TypeBased { discriminant: String },
}
impl Default for KSumTypeSerializationForm {
    fn default() -> KSumTypeSerializationForm { KSumTypeSerializationForm::NameBased }
}
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Clone)]
#[derive(Debug)]
pub struct KSumTypeVariant {
    pub span: KSpan,
    pub name: String,
    pub comment: String,
    #[serde(default)]
    #[serde(skip_serializing_if="is_default")]
    pub attrs: KAttrs,
    /// Type of stored data in this sum-type variant.
    /// Name-based sum-types can define array/optional content.
    /// Type-based sum-types only can define explicit reference to other type.
    pub content: KType,
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Clone)]
#[derive(Debug)]
pub struct KProdType {
    pub span: KSpan,
    pub name: String,
    pub comment: String,
    #[serde(default)]
    #[serde(skip_serializing_if="is_default")]
    pub attrs: KAttrs,
    pub fields: Vec<KProdTypeField>,
}
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Clone)]
#[derive(Debug)]
pub struct KProdTypeField {
    pub span: KSpan,
    pub name: String,
    pub comment: String,
    #[serde(default)]
    #[serde(skip_serializing_if="is_default")]
    pub attrs: KAttrs,
    pub content: KType,
}



#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Clone)]
#[derive(Debug)]
pub struct KFuncType {
    pub span: KSpan,
    pub name: String,
    pub comment: String,
    #[serde(default)]
    #[serde(skip_serializing_if="is_default")]
    pub attrs: KAttrs,
    pub input: KType,
    pub output: KType,
}
    



#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Clone)]
#[derive(Debug)]
pub struct KItemPath {
    pub span: KSpan,
    // pub mods: Vec<String>,
    pub name: String,
}





fn is_default(x:&KAttrs) -> bool {
    *x == KAttrs::default()
}