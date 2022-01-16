use crate::prelude::*;
use crate::model::{KMod, KSpan};
use crate::model::log::{Log, ErrorLogs, Result};

pub fn render_ridl1(m:&KMod) -> Result<String> {
    let code = match serde_yaml::to_string(&m) {
        Err(xx) => return err(m.span, &format!("YAML encoding error: {}", xx)),
        Ok(xx) => xx,
    };
    Ok(code)
}

fn err<T>(span: KSpan, message: &str) -> Result<T> {
    let log = Log { span: span, message: PString::new(message.to_string()) };
    Err(ErrorLogs(PVec::from(vec![log])))
}
