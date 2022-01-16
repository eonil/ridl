use super::*;

#[derive(Default)]
#[derive(Clone, Copy)]
pub struct Options {
    pub case: Option<Rule>,
    pub variant: Option<Rule>,
    pub field: Option<Rule>,
}
/// Follows [`serde` renaming rules](https://serde.rs/container-attrs.html#rename_all).
#[derive(strum_macros::EnumString)]
#[derive(Clone, Copy)]
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
            Enum(x) => x.rename(options),
            Sum(x) => x.rename(options),
            Prod(x) => x.rename(options),
            _ => (),
        }
    }
}

impl KEnumType {
    fn rename(&mut self, options:&Options) {
        for case in self.cases.iter_mut() {
            case.rename(options);
        }
    }
}

impl KEnumTypeCase {
    fn rename(&mut self, options:&Options) {
        if let Some(rule) = &options.case {
            match rule {
            Rule::CamelCase => self.name = pascal_case_to_camel_case(&self.name),
            }
        }
    }
}

impl KSumType {
    fn rename(&mut self, options:&Options) {
        for variant in self.variants.iter_mut() {
            variant.rename(options);
        }
    }
}

impl KSumTypeVariant {
    fn rename(&mut self, options:&Options) {
        if let Some(rule) = &options.variant {
            match rule {
            Rule::CamelCase => self.name = pascal_case_to_camel_case(&self.name),
            }
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

fn pascal_case_to_camel_case(n:&str) -> String {
    if n.is_empty() { return String::new() }
    let mut s = String::new();
    let mut chs = n.chars();
    s.extend(chs.next().unwrap().to_lowercase());
    s.extend(chs);
    s
}
fn snake_to_camel_case(n:&str) -> String {
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