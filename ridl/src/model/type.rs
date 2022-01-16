
/// WIP...
pub enum Type {
    /// 0..N homogeneous instances.
    Vector(ScalarType),
    /// 0..1 instances.
    Option(ScalarType),
    /// Always 1 instance.
    Scalar(ScalarType),
    /// Always 0 instance.
    Never,
    /// Unsupported types.
    /// This is required to skip parsing unsupported syntax patterns.
    Unknown(Unknown),
}
pub enum ScalarType {
    Def(TypeName),
    Prim(PrimType),
    Unit,
}
pub enum PrimType {
    Bool,
    I32,
    F64,
    String,
}
