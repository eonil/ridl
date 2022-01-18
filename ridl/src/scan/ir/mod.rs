//! Scanner IR.
//! 
//! It's difficult to scan RIDL model directly from AST.
//! Especially for types.

mod attr;
mod r#fn;
mod r#trait;
mod r#type;

pub use attr::{Attr, AttrName, AttrParam, AttrValue, scan_attrs};
pub use r#fn::{Fn,scan_fn};
pub use r#trait::{Trait, TraitItem, scan_trait};
pub use r#type::{Type, scan_type};

use super::{Result, SpanScan, err, err_with};

/// Represents an unsupported construct.
pub struct Unknown {
    pub span: Span,
    pub message: String,
}

type Span = crate::model::KSpan;

fn unknown<T:syn::spanned::Spanned>(x:&T, message:&str) -> Unknown {
    Unknown { span: x.span().scan(), message: message.to_string() }
}