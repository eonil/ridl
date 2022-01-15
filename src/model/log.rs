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
pub impl<I,O,F> Vec<I> where F:Fn(&I)->Result<O> {
    fn map_collect_result(&self, fx:F) -> Result<Vec<O>> {
        self.iter().map_collect_result(fx)
    }
}
#[ext(name=IterLogUtil)] 
pub impl<X:Iterator,O,F> X where F:Fn(X::Item)->Result<O> {
    fn map_collect_result(&mut self, fx:F) -> Result<Vec<O>> {
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
}