//! Defines model of KCG.
//! 
//! KCG can build its own model by scanning other datastructure schema.
//! For OpenAPI 3.0 Schema, KCG finds certain patterns to build certain types.
//! 
//! All types are `K` prefixed which means "Schema".

pub mod log;
pub mod span;

use serde_derive::{Serialize, Deserialize};

pub use span::{KSpan, KLineColumn};

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Debug)]
pub struct KMod {
    pub span: KSpan,
    pub name: String,
    pub items: Vec<KItem>,
    pub comment: String,
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Debug)]
pub enum KItem {
    Mod(KMod),
    New(KNewType),
    Enum(KEnumType),
    Sum(KSumType),
    Prod(KProdType),
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
        }
    }
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct KNewType {
    pub span: KSpan,
    pub name: String,
    pub origin: KTypeRef,
    pub comment: String,
}

/// Finite constant value set.
/// Rust code-gen provides automatic enum/string conversion.
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct KEnumType {
    pub span: KSpan,
    pub name: String,
    pub cases: Vec<KEnumTypeCase>,
    pub comment: String,
}
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct KEnumTypeCase {
    pub span: KSpan,
    pub name: String,
    // /// Underlying value for this case.
    // pub value: Option<i32>,
    pub comment: String,
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct KSumType {
    pub span: KSpan,
    pub name: String,
    /// Discriminant field name. 
    /// This controls how variant discriminants to be serialized.
    /// This is controlled by `#[ridl(tag="type")]` attribute.
    pub discriminant: Option<String>,
    pub variants: Vec<KSumTypeVariant>,
    pub comment: String,
}
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct KSumTypeVariant {
    pub span: KSpan,
    pub name: String,
    /// Type of stored data in this sum-type variant.
    /// Name-based sum-types can define array/optional content.
    /// Type-based sum-types only can define explicit reference to other type.
    pub content: KContentStorage,
    pub comment: String,
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct KProdType {
    pub span: KSpan,
    pub name: String,
    pub fields: Vec<KProdTypeField>,
    pub comment: String,
}
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct KProdTypeField {
    pub span: KSpan,
    pub name: String,
    pub content: KContentStorage,
    pub comment: String,
}

/// An inveted concept to simplify type definition.
/// Proper support for optional/array types will require full support for generics.
/// To eliminate complexity of generics support, I just baked-in some essential generic patterns.
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct KContentStorage {
    pub optional: bool,
    pub array: bool,
    pub r#type: KTypeRef,
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Debug)]
pub enum KTypeRef {
    /// Unit type.
    /// Some code-gen can reject unit type.
    /// Unit type is implicitly defined by KCG.
    Unit,
    /// Pre-defined primitive types.
    /// Some code-gen can reject certain set of primitive types.
    /// Primitive types are implicitly defined by KCG.
    Prim(KPrimType),
    /// Name to a defined type.
    /// This must be a defined name in schema document.
    Def(KItemPath),
}
impl Default for KTypeRef {
    fn default() -> KTypeRef { Self::Unit }
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Debug)]
pub struct KItemPath {
    pub span: KSpan,
    // pub mods: Vec<String>,
    pub name: String,
}

/// A simple value with no substructure.
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Debug)]
pub enum KPrimType {
    Bool,
    I32,
    I64,
    F32,
    F64,
    String,
}



