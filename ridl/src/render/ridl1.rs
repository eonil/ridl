use serde::{Serialize,Deserialize};
use serde_with::skip_serializing_none;
use crate::prelude::*;
use crate::model::{KMod, KSpan};
use crate::model::rest;
use crate::model::log::{Log, ErrorLogs, Result};

#[skip_serializing_none]
#[derive(Serialize,Deserialize)]
pub struct Doc {
    pub file: KMod,
    pub rest: Option<rest::RESTAPI>,
}

pub fn render_ridl1(m:&KMod) -> Result<String> {
    let doc = Doc {
        file: m.clone(),
        rest: None,
    };
    let code = match serde_yaml::to_string(&doc) {
        Err(xx) => return err(m.span, &format!("YAML encoding error: {}", xx)),
        Ok(xx) => xx,
    };
    Ok(code)
}

fn err<T>(span: KSpan, message: &str) -> Result<T> {
    let log = Log { span: span, message: PString::new(message.to_string()) };
    Err(ErrorLogs(PVec::from(vec![log])))
}
