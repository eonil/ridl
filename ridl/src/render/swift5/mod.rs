use extend::ext;
use indoc::indoc;

use crate::prelude::*;
use crate::model::*;
use crate::model::log::*;

pub fn render_swift5(x:&KMod) -> Result<String> {
    x.render()
}

trait Swift5Rendering {
    fn render(&self) -> Result<String>;
}
trait Swift5RenderingWithSpan {
    fn render(&self, span:KSpan) -> Result<String>;
}

impl Swift5Rendering for KMod {
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

impl Swift5Rendering for KItem {
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

impl Swift5Rendering for KNewType {
    fn render(&self) -> Result<String> {
        Ok(format!(
            indoc!(r#"
                {comment}   
                typealias {name} = {origin}
            "#),
            comment=self.comment.commentize(),
            name=self.name,
            origin=self.origin.render(self.span)?,
        ))
    }
}

impl Swift5Rendering for KEnumType {
    fn render(&self) -> Result<String> {
        Ok(format!(
            indoc!(r#"
                {comment}
                enum {name}: String, Equatable, Codable {{
                {cases}
                }}
            "#),
            comment=self.comment.commentize(),
            name=self.name,
            cases=self.cases.iter().map_join(render_case, "\n")?.indent(),
        ))
    }
}
fn render_case(x:&KEnumTypeCase) -> Result<String> {
    Ok(format!(r#"case {name} = "{value}""#, name=x.name, value=x.name))
}

impl Swift5Rendering for KSumType {
    fn render(&self) -> Result<String> {
        Ok(format!(
            indoc!(r#"
                {comment}
                enum {name}: Equatable, Codable {{
                {variants}
                }}
            "#),
            comment=self.comment.commentize(),
            name=self.name,
            variants=self.variants.iter().map_join(render_sum_type_variant, "\n")?.indent(),
        ))
    }
}
fn render_sum_type_variant(x:&KSumTypeVariant) -> Result<String> {
    Ok(format!(
        indoc!(r#"
            {comment}
            case {name}({ty})
        "#),
        comment=x.comment.commentize(),
        name=x.name,
        ty=x.content.render(x.span)?,
    )).trim()
}

impl Swift5Rendering for KProdType {
    fn render(&self) -> Result<String> {
        Ok(format!(
            indoc!(r#"
                {comment}
                struct {name}: Equatable, Codable {{
                {properties}
                }}
            "#),
            comment=self.comment.commentize(),
            name=self.name,
            properties=self.fields.iter().map_join(KProdTypeField::render, "\n")?.indent(),
        ))
    }
}
impl Swift5Rendering for KProdTypeField {
    fn render(&self) -> Result<String> {
        Ok(format!(
            indoc!(r#"
                {comment}
                var {name}: {ty}
            "#),
            comment=self.comment.commentize(),
            name=self.name,
            ty=self.content.render(self.span)?,
        )).trim()
    }
}

impl Swift5Rendering for KFuncType {
    fn render(&self) -> Result<String> {
        Ok(format!(
            indoc!(r#"
                {comment}
                typealias {name} = ({input}) -> ({output})
            "#),
            comment=self.comment.commentize(),
            name=self.name,
            input=self.input.render(self.span)?,
            output=self.output.render(self.span)?,
        ))
    }
}






impl Swift5RenderingWithSpan for KType {
    fn render(&self, span:KSpan) -> Result<String> {
        use KType::*;
        match self {
            Vector(x) => return Ok(format!("[{code}]", code=x.render(span)?)),
            Option(x) => return Ok(format!("{code}?", code=x.render(span)?)),
            Scalar(x) => x.render(span),
            Never => return err(span, "never-type is not unsupported"),
            Unknown => return err(span, "unsupported type pattern"),
        }
    }
}
impl Swift5RenderingWithSpan for KScalarType {
    fn render(&self, span:KSpan) -> Result<String> {
        use KScalarType::*;
        use KPrimType::*;
        let x = match self {
            Unit => return err(span, "unit-type (`()`) is not supported"),
            Def(x) => &x,
            Prim(Bool) => "Bool",
            Prim(I32) => "Int32",
            Prim(I64) => "Int64",
            Prim(F32) => "Int32",
            Prim(F64) => "Int64",
            Prim(String) => "String",
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







