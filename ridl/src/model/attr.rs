use serde::{Serialize,Deserialize};

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default,Clone)]
#[derive(Debug)]
pub struct KAttrs {
    #[serde(default)]
    #[serde(skip_serializing_if="is_default")]
    pub rest: Vec<KAttrREST>,
}

#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Clone)]
#[derive(Debug)]
pub enum KAttrREST {
    FnMethod(String),
    FnPath(String),
    MessageIn,
    MessageOut,
    PathParam,
    QueryParam,
    BodyParam,
    Status(i64),
    MIME(String),
}

pub type KAttrRESTHTTPMethod = String;
pub type KAttrRESTHTTPPath = String;

fn is_default(x:&Vec<KAttrREST>) -> bool {
    *x == Vec::default()
}