use extend::ext;
use crate::model::{KSpan,KLineColumn};

#[ext(name=SpanScan)]
pub impl proc_macro2::Span {
    fn scan(&self) -> KSpan {
        KSpan {
            start: self.start().scan(),
            end: self.end().scan(),
        }
    }
}
#[ext(name=LineColumnScan)]
pub impl proc_macro2::LineColumn {
    fn scan(&self) -> KLineColumn {
        KLineColumn {
            line: self.line,
            column: self.column,
        }
    }
}