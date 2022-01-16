//! OpenAPI Schema 3.0 Model
//! https://swagger.io/specification/
//! 
//! Strictly follows spec. Simplification will be done to KCG's own model.
//! Strict subset of OpenAPI 3.0 model. 
//! Unsupported features will cause an error.

mod info;
mod parameter;
mod request_body;
mod response;
mod media_type;
mod schema;

use serde_derive::{Serialize, Deserialize};
use serde_with::skip_serializing_none;

pub use info::*;
pub use parameter::*;
pub use request_body::*;
pub use response::*;
pub use media_type::*;
pub use schema::*;

pub type List<T> = std::vec::Vec<T>;
// pub type Map<K,V> = std::collections::HashMap<K,V>;
// pub type Map<K,V> = vector_map::VecMap<K,V>;
pub type Map<K,V> = linear_map::LinearMap<K,V>;

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Doc {
    pub openapi: String,
    pub info: Info,
    pub servers: Option<List<Server>>,
    pub paths: Paths,
    pub components: Option<Components>,
    pub security: Option<List<SecurityRequirement>>,
    pub tags: Option<List<Tag>>,
    pub external_docs: Option<ExternalDocumentation>,
}

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Server {
    pub url: String,
    pub description: Option<String>,
    pub variables: Map<String,ServerVariable>,
}

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct ServerVariable {
    pub r#enum: Option<List<String>>,
    pub default: String,
    pub description: Option<String>,
}

pub type Paths = Map<String,PathItem>;

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct PathItem {
    #[serde(rename="$ref")]
    r#ref: Option<String>,
    summary: Option<String>,
    description: Option<String>,
    // get: Option<Operation>,
    // put: Option<Operation>,
    // post: Option<Operation>,
    // delete: Option<Operation>,
    // options: Option<Operation>,
    // head: Option<Operation>,
    // patch: Option<Operation>,
    // trace: Option<Operation>,
    // servers: Option<Vec<Server>>,
    // parameters: Option<ReferenceOrParameter>,
}

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Components {
    pub schemas: Option<Map<String,ReferencedOrInlineSchema>>,
    pub responses: Option<Map<String,ResponseOrReference>>,
    pub parameters: Option<Map<String,ParameterOrReference>>,
    pub request_bodies: Option<Map<String,RequestBodyOrReference>>,
}

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct SecurityRequirement {

}

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Tag {

}

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct ExternalDocumentation {

}







#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(untagged)]
#[serde(rename_all="camelCase")]
pub enum ReferencedOrInlineSchema {
    Referenced(Reference),
    Inline(Schema),
}

/// Only local paths are supported.
#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Reference {
    #[serde(rename="$ref")]
    pub r#ref: String,
}



pub type MIMEType = String;








fn is_default<T:Default + PartialEq>(x:&T) -> bool {
    *x == T::default()
}



#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
#[serde(untagged)]
#[serde(rename_all="camelCase")]
pub enum Or<A,B> {
    A(A),
    B(B),
}
pub type SchemaOrReference = Or<Schema,Reference>;
pub type ResponseOrReference = Or<Response,Reference>;
pub type ParameterOrReference = Or<Parameter,Reference>;
pub type RequestBodyOrReference = Or<RequestBody,Reference>;
