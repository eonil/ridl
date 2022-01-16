use serde::{Serialize,Deserialize};
use serde_with::skip_serializing_none;

use crate::prelude::*;
use super::is_default;
use super::ReferencedOrInlineSchema;

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Info {
    pub title: String,
    pub description: Option<String>,
    pub terms_of_service: Option<String>,
    pub contact: Option<Contact>,
    pub license: Option<License>,
    pub version: String,
}

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct Contact {
    pub name: Option<String>,
    pub url: Option<String>,
    pub email: Option<String>,
}

#[skip_serializing_none]
#[derive(Eq, PartialEq)]
#[derive(Serialize, Deserialize)]
#[derive(Default)]
#[derive(Debug)]
#[serde(rename_all="camelCase")]
pub struct License {
    pub name: String,
    pub url: Option<String>,
}
