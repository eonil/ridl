use extend::ext;
use super::KSpan;
use crate::prelude::*;

#[must_use]
pub type Result<T> = std::result::Result<T,ErrorLogs>;

#[derive(Clone)]
#[derive(Debug)]
pub struct Log {
    pub span: KSpan,
    pub message: PString,
}
impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f,
            "LOG({}:{}-{}:{}): {}", 
            self.span.start.line, 
            self.span.start.column,
            self.span.end.line,
            self.span.end.column,
            self.message)
    }
}

#[derive(Clone)]
#[derive(Debug)]
pub struct ErrorLogs(pub PVec<Log>);
impl std::error::Error for ErrorLogs {}
impl std::fmt::Display for ErrorLogs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in self.0.iter() {
            write!(f, "{}\n", x)?;
        }
        Ok(())
    }
}

#[ext(name=VecLogUtil)] 
pub impl<I,O> Vec<I> {
    fn map_collect_result<F>(&self, fx:F) -> Result<Vec<O>> where F:Fn(&I)->Result<O> {
        self.iter().map_collect_result(fx)
    }
    fn map_collect_optional_result<F>(&self, fx:F) -> Result<Vec<O>> where F:Fn(&I)->Result<Option<O>> {
        self.iter().map_collect_optional_result(fx)
    }
}
#[ext(name=IterLogUtil)] 
pub impl<X:Iterator,O> X {
    fn map_collect_result<F>(&mut self, fx:F) -> Result<Vec<O>> where F:Fn(X::Item)->Result<O> {
        let mut oks = Vec::new();
        let mut logs = PVec::<Log>::new();
        for x in self {
            match fx(x) {
                Err(sublogs) => logs.extend(sublogs.0),
                Ok(ok) => oks.push(ok),
            }
        }
        if logs.len() > 0 { return Err(ErrorLogs(logs)) }
        Ok(oks)
    }
    fn map_collect_optional_result<F>(&mut self, fx:F) -> Result<Vec<O>> where F:Fn(X::Item)->Result<Option<O>> {
        let mut oks = Vec::new();
        let mut logs = PVec::<Log>::new();
        for x in self {
            match fx(x) {
                Err(sublogs) => logs.extend(sublogs.0),
                Ok(Some(ok)) => oks.push(ok),
                Ok(None) => continue,
            }
        }
        if logs.len() > 0 { return Err(ErrorLogs(logs)) }
        Ok(oks)
    }
}