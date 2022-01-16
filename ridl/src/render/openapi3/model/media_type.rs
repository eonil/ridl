use serde::{Serialize,Deserialize};
use serde_with::skip_serializing_none;

use crate::prelude::*;
use super::{ReferencedOrInlineSchema};

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug)]
pub struct MediaType {
    pub schema: Option<ReferencedOrInlineSchema>,
}