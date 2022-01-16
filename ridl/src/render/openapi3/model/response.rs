use serde::{Serialize,Deserialize};
use serde_with::skip_serializing_none;

use crate::prelude::*;
use super::{MIMEType,MediaType};

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Response {
    pub description: String,
    pub content: Option<Map<MIMEType,MediaType>>,
}
