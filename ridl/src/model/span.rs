use serde_derive::{Serialize, Deserialize};

// #[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Clone,Copy)]
#[derive(Debug)]
pub struct KSpan {
    pub start: KLineColumn,
    pub end: KLineColumn,
}
impl serde::Serialize for KSpan {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> where S: serde::Serializer {
        let s = format!("{}:{}-{}:{}", self.start.line, self.start.column, self.end.line, self.end.column);
        serializer.serialize_str(&s)
    }
}
impl<'de> serde::Deserialize<'de> for KSpan {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error> where D: serde::Deserializer<'de> {
        let err = serde::de::Error::custom;
        let s = deserializer.deserialize_string(KSpanStringVisit)?;
        let mut comps = s.split("-");
        let comp1 = comps.next().ok_or(err("missing span start position"))?;
        let comp2 = comps.next().ok_or(err("missing span end position"))?;
        if comps.next() != None { return Err(err("badly formed span")) }
        let mut start = comp1.split(":");
        let mut end = comp2.split(":");
        let start_line = start.next().ok_or(err("missing span start line number"))?;
        let start_column = start.next().ok_or(err("missing span start column number"))?;
        let end_line = end.next().ok_or(err("missing span end line number"))?;
        let end_column = end.next().ok_or(err("missing span end column number"))?;
        Ok(KSpan {
            start: KLineColumn { 
                line: parse_usize::<D>(start_line, "badly formed span start line")?,
                column: parse_usize::<D>(start_column, "badly formed span start column")?,
            },
            end: KLineColumn {
                line: parse_usize::<D>(end_line, "badly formed span end line")?,
                column: parse_usize::<D>(end_column, "badly formed span end column")?,
            }
        })
    }
}
fn parse_usize<'de,D>(s:&str, message_prefix:&str) -> std::result::Result<usize, D::Error> where D: serde::Deserializer<'de> {
    s.parse::<usize>().map_err(|x| serde::de::Error::custom(&format!("{} ({})", message_prefix, x)))
}
struct KSpanStringVisit;
impl<'de> serde::de::Visitor<'de> for KSpanStringVisit {
    type Value = String;
    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E> where E: serde::de::Error {
        Ok(s.to_owned())
    }
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "a string")
    }
}


#[derive(Serialize,Deserialize)]
#[derive(Eq,PartialEq)]
#[derive(Default)]
#[derive(Clone,Copy)]
#[derive(Debug)]
pub struct KLineColumn {
    pub line: usize,
    pub column: usize,
}