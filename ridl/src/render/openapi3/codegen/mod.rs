use extend::ext;

use crate::prelude::*;
use crate::model::*;
use crate::model::log::*;
use super::model as oa;

mod response;

pub fn render_openapi3(x:&KMod) -> Result<String> {
    let doc = x.render()?;
    let code = match serde_yaml::to_string(&doc) {
        Err(xx) => return err(x.span, &format!("YAML encoding error: {}", xx)),
        Ok(xx) => xx,
    };
    Ok(code)
}

#[ext(name=KModOpenAPI3Rendering)]
impl KMod {
    fn render(&self) -> Result<oa::Doc> {
        let mut doc = oa::Doc::default();
        doc.openapi = "3.0.1".to_string();
        doc.info.description.set(self.comment.trim());
        doc.info.title = self.name.clone();
        let comps = doc.components.ridl_get_or_insert_default();
        let schemas = comps.schemas.ridl_get_or_insert_default();
        for (name,schema) in self.render_items()? {
            schemas.insert(name, schema);
        }
        Ok(doc)
    }
    fn render_items(&self) -> Result<Vec<(String,oa::ReferencedOrInlineSchema)>> {
        let mut xs = Vec::new();
        for item in self.items.iter() {
            use KItem::*;
            let (name,rendered_item) = match item {
                Mod(x) => {
                    let subitems = x.render_items()?;
                    xs.extend(subitems);
                    continue;
                },
                New(x) => (&x.name, x.render()?),
                Enum(x) => (&x.name, x.render()?),
                Sum(x) => (&x.name, x.render()?),
                Prod(x) => (&x.name, x.render()?),
                Func(_) => continue,
            };
            xs.push((name.clone(), rendered_item));
        }
        Ok(xs)
    }
}

#[ext(name=KNewTypeOpenAPI3Rendering)]
impl KNewType {
    fn render(&self) -> Result<oa::ReferencedOrInlineSchema> {
        self.origin.render(self.span)
    }
}

#[ext(name=KEnumTypeOpenAPI3Rendering)]
impl KEnumType {
    fn render(&self) -> Result<oa::ReferencedOrInlineSchema> {
        let mut k = oa::Schema::default();
        k.title.set(&self.name);
        k.description.set(self.collect_all_comments().trim());
        k.r#type.set("string");
        fn json(x:&KEnumTypeCase) -> serde_json::Value { serde_json::Value::String(x.name.clone()) }
        k.r#enum = Some(self.cases.iter().map(json).collect());
        Ok(oa::ReferencedOrInlineSchema::Inline(k))
    }
    fn collect_all_comments(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.comment);
        s.push_str("\n");
        for x in self.cases.iter() {
            s.push_str(&x.comment);
        }
        s
    }
}

#[ext(name=KSumTypeOpenAPI3Rendering)]
impl KSumType {
    fn render(&self) -> Result<oa::ReferencedOrInlineSchema> {
        use KSumTypeSerializationForm::*;
        match &self.serialization {
            NameBased => self.render_name_based_form(),
            TypeBased { discriminant: x } => self.render_type_based_form(x),
        }
    }
    fn render_name_based_form(&self) -> Result<oa::ReferencedOrInlineSchema> {
        let mut k = oa::Schema::default();
        k.title.set(&self.name);
        k.description.set(self.comment.trim());
        k.r#type.set("object");
        k.one_of = Some(self.variants.iter().map_collect_result(KSumTypeVariant::render_name_based_form)?);
        Ok(oa::ReferencedOrInlineSchema::Inline(k))
    }
    fn render_type_based_form(&self, discriminant_prop_name: &str) -> Result<oa::ReferencedOrInlineSchema> {
        let mut k = oa::Schema::default();
        k.title.set(&self.name);
        k.description.set(self.comment.trim());
        k.r#type.set("object");
        k.one_of = Some(self.variants.iter().map_collect_result(KSumTypeVariant::render_type_based_form)?);
        let d = k.discriminator.ridl_get_or_insert_default();
        d.property_name = discriminant_prop_name.to_string();
        Ok(oa::ReferencedOrInlineSchema::Inline(k))
    }
}

#[ext(name=KSumTypeVariantOpenAPI3Rendering)]
impl KSumTypeVariant {
    fn render_name_based_form(&self) -> Result<oa::ReferencedOrInlineSchema> {
        let mut x = oa::Schema::default();
        x.description.set(&self.comment);
        let ps = x.properties.ridl_get_or_insert_default();
        let p = self.content.render(self.span)?;
        ps.insert(self.name.clone(), p);
        Ok(oa::ReferencedOrInlineSchema::Inline(x))
    }
    fn render_type_based_form(&self) -> Result<oa::ReferencedOrInlineSchema> {
        if self.content.is_vector() == true { return err(self.span, "vector-type is not supported in type-based sum-type") }
        if self.content.is_option() == true { return err(self.span, "option-type is not supported in type-based sum-type") }
        self.content.render(self.span)
    }
}

#[ext(name=KProdTypeOpenAPI3Rendering)]
impl KProdType {
    fn render(&self) -> Result<oa::ReferencedOrInlineSchema> {
        let mut k = oa::Schema::default();
        k.title.set(&self.name);
        k.description.set(self.comment.trim());
        k.r#type.set("object");
        let reqs = k.required.ridl_get_or_insert_default();
        let mut propks = oa::Map::new();
        for field in self.fields.iter() {
            let propk = field.content.render(self.span)?;
            if !field.content.is_option() { reqs.push(field.name.clone()) }
            propks.insert(field.name.clone(), propk);
        }
        k.properties.set(propks);
        Ok(oa::ReferencedOrInlineSchema::Inline(k))
    }
}

#[ext(name=KTypeOpenAPI3Rendering)]
impl KType {
    fn render(&self, span:KSpan) -> Result<oa::ReferencedOrInlineSchema> {
        match self {
            KType::Vector(x) => {
                let mut k = oa::Schema::default();
                k.items = Some(Box::new(x.render(span)?));
                k.r#type.set("array");
                Ok(oa::ReferencedOrInlineSchema::Inline(k))
            },
            KType::Option(x) => x.render(span), // Optinality need to be defined in parent node.
            KType::Scalar(x) => x.render(span),
            KType::Never => return err(span, "never-type is not unsupported"),
            KType::Unknown => return err(span, "unsupported type pattern"),
        }
    }
}

#[ext(name=KScalarTypeOpenAPI3Rendering)]
impl KScalarType {
    fn render(&self, span:KSpan) -> Result<oa::ReferencedOrInlineSchema> {
        fn inline(r#type:&str, format:&str) -> Result<oa::ReferencedOrInlineSchema> {
            let mut k = oa::Schema::default();
            k.r#type.set(r#type);
            k.format = if format.is_empty() { None } else { Some(format.to_string()) };
            Ok(oa::ReferencedOrInlineSchema::Inline(k))
        }
        use KScalarType::*;
        use KPrimType::*;
        match self {
            Unit => return err(span, "unit-type (`()`) is not supported"),
            Prim(Bool) => inline("boolean", ""),
            Prim(I32) => inline("integer", "int32"),
            Prim(I64) => inline("integer", "int64"),
            Prim(F32) => inline("number", "float"),
            Prim(F64) => inline("number", "double"),
            Prim(String) => inline("string", ""),
            Def(x) => Ok(oa::ReferencedOrInlineSchema::Referenced(oa::Reference { r#ref: make_opanapi3_ref(&x) })),
        }
    }
}
fn make_opanapi3_ref(name:&str) -> String {
    format!("#/components/schemas/{}", name).to_string()
}










#[ext(name=DefaultOptionUtil)]
impl<T:Default + PartialEq> Option<T> {
    fn set<V:Into<T>>(&mut self, content: V) {
        let value = content.into();
        *self = if value == T::default() { None } else { Some(value) }
    }
    fn ridl_get_or_insert_default(&mut self) -> &mut T {
        match self {
            None => {
                *self = Some(T::default());
                return self.ridl_get_or_insert_default();
            },
            Some(x) => return x,
        }
    }
}

fn err<T>(span: KSpan, message: &str) -> Result<T> {
    let log = Log { span: span, message: PString::new(message.to_string()) };
    Err(ErrorLogs(PVec::from(vec![log])))
}







