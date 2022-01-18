use serde_derive::{Serialize, Deserialize};

/// An invented concept to simplify type definition.
/// Proper support for optional/array types will require full support for generics.
/// To eliminate complexity of generics support, I just baked-in some essential generic patterns.
#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub enum KType {
    /// 0..N homogeneous instances.
    Vector(KScalarType),
    /// 0..1 instances.
    Option(KScalarType),
    /// Always 1 instance.
    Scalar(KScalarType),
    /// Always 0 instance.
    Never,
    /// Unsupported types.
    /// This is required to skip parsing unsupported syntax patterns.
    Unknown,
}
impl KType {
    pub fn is_vector(&self) -> bool {
        if let KType::Vector(_) = self { true } else { false }
    }
    pub fn is_option(&self) -> bool {
        if let KType::Option(_) = self { true } else { false }
    }
    pub fn is_scalar(&self) -> bool {
        if let KType::Scalar(_) = self { true } else { false }
    }
}
impl Default for KType {
    fn default() -> KType { KType::Unknown }
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub enum KScalarType {
    /// Name to a defined type.
    /// This must be a defined name in schema document.
    Def(KTypeName),
    /// Pre-defined primitive types.
    /// Some code-gen can reject certain set of primitive types.
    /// Primitive types are implicitly defined by KCG.
    Prim(KPrimType),
    /// Unit type.
    /// Some code-gen can reject unit type.
    /// Unit type is implicitly defined by KCG.
    Unit,
}
impl Default for KScalarType {
    fn default() -> KScalarType { Self::Unit }
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub enum KPrimType {
    Bool,
    I32,
    I64,
    F32,
    F64,
    String,
}

pub type KTypeName = String;