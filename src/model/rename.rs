use super::*;

#[derive(Default)]
pub struct Options {
    pub field: Option<Rule>,
}
#[derive(strum_macros::EnumString)]
pub enum Rule {
    #[strum(serialize="camel")]
    CamelCase,
}

impl KMod {
    pub fn rename(&mut self, options:&Options) {
        for item in self.items.iter_mut() {
            item.rename(options);
        }
    }
}
impl KItem {
    fn rename(&mut self, options:&Options) {
        use KItem::*;
        match self {
            Mod(x) => x.rename(options),
            Prod(x) => x.rename(options),
            _ => (),
        }
    }
}
impl KProdType {
    fn rename(&mut self, options:&Options) {
        for field in self.fields.iter_mut() {
            field.rename(options);
        }
    }
}
impl KProdTypeField {
    fn rename(&mut self, options:&Options) {
        if let Some(rule) = &options.field {
            match rule {
            Rule::CamelCase => self.name = snake_to_camel_case(&self.name),
            }
        }
    }
}

fn snake_to_camel_case(n:&str) -> String {
    // Follows [`serde` renaming rules](https://serde.rs/container-attrs.html#rename_all).
    // Please remember that all source member names are 
    let mut s = String::new();
    let mut comps = n.split("_");
    let first = if let Some(first) = comps.next() { first } else { return String::new() };
    s.push_str(first);
    for comp in comps { s.push_str(&first_char_capitalized(comp)) }
    s
}
fn first_char_capitalized(s:&str) -> String {
    if s.is_empty() { return String::new() }
    let mut z = String::new();
    let mut chs = s.chars();
    let a = chs.next().unwrap();
    for ch in a.to_uppercase() { z.push(ch) }
    for ch in chs { z.push(ch) }
    z
}