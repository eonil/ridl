mod attr;
mod r#trait;
mod r#type;

pub use attr::*;
pub use r#trait::*;
pub use r#type::{Type};

use super::Result;
use super::err;

/// Represents an unsupported construct.
pub struct Unknown {
    span: Span,
    code: String,
}

pub type Span = crate::model::KSpan;
