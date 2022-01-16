use serde::{Serialize,Deserialize};
use serde_with::skip_serializing_none;

use crate::prelude::*;
use super::is_default;
use super::{Reference,ReferencedOrInlineSchema};

/// OpenAPI 3.0 Schema object.
/// - Follows JSON Schema 2020-12 spec where needed
///   - https://json-schema.org/specification.html
/// - Only certain strict subset will be supported.
/// - Any unsupported properties for unsupported features will be rejected.
#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[serde(deny_unknown_fields)]
#[serde(rename_all="camelCase")]
pub struct Schema {
    pub title:  Option<String>,
    pub summary: Option<String>,
    pub required: Option<List<String>>,
    pub r#enum: Option<List<serde_json::Value>>,

    pub r#type: Option<String>,
    
    pub all_of: Option<Vec<ReferencedOrInlineSchema>>,
    pub one_of: Option<Vec<ReferencedOrInlineSchema>>,
    pub any_of: Option<Vec<ReferencedOrInlineSchema>>,
    pub not: Option<Box<ReferencedOrInlineSchema>>,
    pub items: Option<Box<ReferencedOrInlineSchema>>,
    pub properties: Option<Map<String, ReferencedOrInlineSchema>>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub additional_properties: Box<AdditionalProperties>,
    pub description: Option<String>,
    pub format: Option<String>,
    pub default: Option<serde_json::Value>,

    pub discriminator: Option<Discriminator>,
    pub example: Option<serde_json::Value>,
    #[serde(default)]
    #[serde(skip_serializing_if = "is_default")]
    pub deprecated: bool,
}

/// When request bodies or response payloads may be one of a number of different schemas, a discriminator object can be used to aid in serialization, deserialization, and validation. The discriminator is a specific object in a schema which is used to inform the consumer of the specification of an alternative schema based on the value associated with it.
/// When using the discriminator, inline schemas will not be considered.
/// 
/// The discriminator object is legal only when using one of the composite keywords oneOf, anyOf, allOf.
#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Discriminator {
    pub property_name: String,
    pub mapping: Option<Map<String,String>>,
}

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(untagged)]
#[serde(rename_all="camelCase")]
pub enum AdditionalProperties {
    Bool(bool),
    Referenced(Reference),
    Inline(Schema),
}
impl Default for AdditionalProperties {
    fn default() -> AdditionalProperties { Self::Bool(false) }
}
