use serde::{Serialize,Deserialize};

/// Data structure to support REST-like API.
/// - RIDL does not cover whole REST convention or HTTP standard.
/// - RIDL covers very limited subset of the REST convention or HTTP standard.
/// - RIDL only handles the forms, and does not define semantics.
///   - You are supposed to define meanings of each status code in your application.
#[derive(Serialize,Deserialize)]
#[derive(Default)]
#[derive(Debug)]
pub struct RESTAPI {
    pub functions: Vec<RESTFunction>,
}

#[derive(Serialize,Deserialize)]
#[derive(Default)]
#[derive(Debug)]
pub struct RESTFunction {
    pub comment: String,
    /// User-defined custom attribute strings to control code-gen.
    pub attributes: Vec<String>,
    pub input: RESTInput,
    pub output: RESTOutput,
}

#[derive(Serialize,Deserialize)]
#[derive(Default)]
#[derive(Debug)]
pub struct RESTInput {
    /// HTTP method such as `GET`, `POST` or etc..
    pub method: RESTMethod,
    /// Request path such as `/order/submit`.
    /// Yoou can use template placeholders like `/order/item/{id}`.
    /// That can affect code-gen and may generate a dedicated type for it.
    pub path: RESTParametricPath,
    pub query: Option<RESTMessageTypeName>,
    pub body: Option<RESTMessageTypeName>,
}

#[derive(Serialize,Deserialize)]
#[derive(Default)]
#[derive(Debug)]
pub struct RESTOutput {
    pub variants: Vec<RESTOutputVariant>,
}

#[derive(Serialize,Deserialize)]
#[derive(Default)]
#[derive(Debug)]
pub struct RESTOutputVariant {
    pub status: u16,
    pub body: Option<RESTMessageTypeName>,
    pub comment: String,
}

pub type RESTMethod = String;
pub type RESTParametricPath = String;
/// Put a name to a defined type name in RIDL schema.
pub type RESTMessageTypeName = String;