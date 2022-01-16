use serde::{Serialize,Deserialize};
use serde_with::skip_serializing_none;

use crate::prelude::*;
use super::is_default;
use super::{MIMEType,MediaType};

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct RequestBody {
    pub description: Option<String>,
    pub content: Map<MIMEType,MediaType>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub required: bool,
}

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub enum ParameterIn {
    Query,
    Header,
    Path,
    Cookie,
}
