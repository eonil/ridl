use extend::ext;

use crate::prelude::*;
use crate::model::log::{Log, ErrorLogs};
use super::span::SpanScan;
use super::Result;

#[ext(name=ErrorScan)]
pub impl syn::Error {
    fn scan(&self) -> Log {
        Log {
            span: self.span().scan(),
            message: PString::new(self.to_string()),
        }
    }
}

#[ext(name=ResultConversion)]
pub impl<T> std::result::Result<T,syn::Error> {
    fn into_scan_result(self) -> Result<T> {
        self.map_err(|x| ErrorLogs(PVec::from(vec![x.scan()])))
    }
}