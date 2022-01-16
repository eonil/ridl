use serde::{Serialize,Deserialize};
use serde_with::skip_serializing_none;

use super::is_default;
use super::ReferencedOrInlineSchema;

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Parameter {
    pub name: String,
    pub r#in: ParameterIn,
    pub description: Option<String>,
    pub required: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub deprecated: bool,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub allow_empty_value: bool,
    pub schema: Option<ReferencedOrInlineSchema>,
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
