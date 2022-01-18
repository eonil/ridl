use extend::ext;
use indoc::indoc;

use crate::prelude::*;
use crate::model::*;
use crate::model::log::*;

pub fn render_typescript4(x:&KMod) -> Result<String> {
    x.render()
}

trait TypeScript4Rendering {
    fn render(&self) -> Result<String>;
}
trait TypeScript4RenderingWithSpan {
    fn render(&self, span:KSpan) -> Result<String>;
}

impl TypeScript4Rendering for KMod {
    fn render(&self) -> Result<String> {
        Ok(format!(
            indoc!(r#"
                {comment}
                {items}
            "#),
            comment=self.comment.commentize(),
            items=self.items.iter().map_join(KItem::render, "\n\n")?,
        ))
    }
}

impl TypeScript4Rendering for KItem {
    fn render(&self) -> Result<String> {
        use KItem::*;
        match self {
            Mod(x) => x.render(),
            New(x) => x.render(),
            Enum(x) => x.render(),
            Sum(x) => x.render(),
            Prod(x) => x.render(),
            Func(x) => x.render(),
        }.trim()
    }
}

impl TypeScript4Rendering for KNewType {
    fn render(&self) -> Result<String> {
        Ok(format!(
            indoc!(r#"
                {comment}   
                type {name} = {origin}
            "#),
            comment=self.comment.commentize(),
            name=self.name,
            origin=self.origin.render(self.span)?,
        ))
    }
}

impl TypeScript4Rendering for KEnumType {
    fn render(&self) -> Result<String> {
        Ok(format!(
            indoc!(r#"
                {comment}
                enum {name} {{
                {cases}  
                }}
            "#),
            comment=self.comment.commentize(),
            name=self.name,
            cases=self.cases.iter().map_join(render_case, ",\n")?.indent(),
        ))
    }
}
fn render_case(x:&KEnumTypeCase) -> Result<String> {
    Ok(format!(r#"{name} = "{value}""#, name=x.name, value=x.name))
}

impl TypeScript4Rendering for KSumType {
    fn render(&self) -> Result<String> {
        Ok(format!(
            indoc!(r#"
                {comment}
                type {name} = {variants}
            "#),
            comment=self.comment.commentize(),
            name=self.name,
            variants=self.variants.iter().map_join(render_sum_type_variant, " | ")?,
        ))
    }
}
fn render_sum_type_variant(x:&KSumTypeVariant) -> Result<String> {
    Ok(format!(
        indoc!(r#"
            {{ {name}: {ty} }}
        "#),
        // comment=x.comment.commentize(),
        name=x.name,
        ty=x.content.render(x.span)?,
    )).trim()
}

impl TypeScript4Rendering for KProdType {
    fn render(&self) -> Result<String> {
        Ok(format!(
            indoc!(r#"
                {comment}
                type {name} = {{
                {properties}
                }}
            "#),
            comment=self.comment.commentize(),
            name=self.name,
            properties=self.fields.iter().map_join(KProdTypeField::render, "\n")?.indent(),
        ))
    }
}
impl TypeScript4Rendering for KProdTypeField {
    fn render(&self) -> Result<String> {
        let name_with_optionality = if self.content.is_option() { format!("{}?", self.name) } else { self.name.clone() };
        Ok(format!(
            indoc!(r#"
                {comment}
                {name}: {ty}
            "#),
            comment=self.comment.commentize(),
            name=name_with_optionality,
            ty=self.content.render(self.span)?,
        )).trim()
    }
}

impl TypeScript4Rendering for KFuncType {
    fn render(&self) -> Result<String> {
        Ok(format!(
            indoc!(r#"
                {comment}
                declare function {name}({input}): {output}
                }}
            "#),
            comment=self.comment.commentize(),
            name=self.name,
            input=self.input.render(self.span)?,
            output=self.output.render(self.span)?,
        ))
    }
}





impl TypeScript4RenderingWithSpan for KType {
    fn render(&self, span:KSpan) -> Result<String> {
        use KType::*;
        match self {
            Vector(x) => return Ok(format!("{code}[]", code=x.render(span)?)),
            Option(x) => x.render(span), // Optionality need to be encoded in field, not type.
            Scalar(x) => x.render(span),
            Never => return err(span, "never-type is not unsupported"),
            Unknown => return err(span, "unsupported type pattern"),
        }
    }
}
impl TypeScript4RenderingWithSpan for KScalarType {
    fn render(&self, span:KSpan) -> Result<String> {
        use KScalarType::*;
        use KPrimType::*;
        let x = match self {
            Unit => return err(span, "unit-type (`()`) is not supported"),
            Def(x) => &x,
            Prim(Bool) => "boolean",
            Prim(I32) => "number",
            Prim(I64) => return err(span, "`i64` is not supported in TypeScript"),
            Prim(F32) => return err(span, "`f32` is not supported in TypeScript"),
            Prim(F64) => "number",
            Prim(String) => "string",
        };
        Ok(x.to_string())
    }
}










#[ext(name=IterUtil)]
impl<I:Iterator> I {
    fn map_join<F:Fn(I::Item)->Result<String>>(&mut self, fx:F, sep:&str) -> Result<String> {
        let mut oks = String::new();
        let mut errs = PVec::new();
        while let Some(x) = self.next() {
            match fx(x) {
                Ok(y) => oks.push_str(&y),
                Err(y) => errs.extend(y.0),
            }
            oks.push_str(sep);
        }
        if !oks.is_empty() {
            for _ in 0..sep.len() {
                oks.pop();
            }
        }
        if errs.is_empty() { Ok(oks) } else { Err(ErrorLogs(errs)) }
    }
}

#[ext(name=StringUtil)]
impl String {
    fn commentize(&self) -> String {
        let mut x = String::new();
        let mut f = false;
        for line in self.trim().lines() {
            x.push_str("/// ");
            x.push_str(line);
            x.push_str("\n");
            f = true;
        }
        if f { x.pop(); }
        x
    }
    fn indent(&self) -> String {
        let mut x = String::new();
        let mut f = false;
        for line in self.lines() {
            x.push_str("    ");
            x.push_str(line);
            x.push_str("\n");
            f = true;
        }
        if f { x.pop(); }
        x
    }
}

#[ext(name=ResultUtil)]
impl Result<String> {
    fn trim(&self) -> Result<String> {
        match self {
            Err(x) => Err(x.clone()),
            Ok(x) => Ok(x.trim().to_string()),
        }
    }
}

fn err<T>(span: KSpan, message: &str) -> Result<T> {
    let log = Log { span: span, message: PString::new(message.to_string()) };
    Err(ErrorLogs(PVec::from(vec![log])))
}







