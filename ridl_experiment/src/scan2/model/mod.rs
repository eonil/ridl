use serde::{Serialize,Deserialize};

/// Scanner model.
/// Scanning a Cargo project is very slow.
/// We need to extract a model from it.
#[derive(Serialize,Deserialize)]
#[derive(Default)]
pub struct SCrate {
    pub name: String,
    pub modules: Vec<SModule>,
}

#[derive(Serialize,Deserialize)]
#[derive(Default)]
pub struct SModule {
    pub path: SModulePath,
    pub name: String,
    pub comment: String,
    pub decls: Vec<SModuleDecl>,
}

#[derive(Serialize,Deserialize)]
pub enum SModuleDecl {
    Struct(SStruct),
    Union(SUnion),
    Enum(SEnum),
    Alias(SAlias),
    Trait(STrait),
    Function(SFunction),
    Unknown(String),
}

#[derive(Serialize,Deserialize)]
#[derive(Default)]
pub struct SStruct {
    pub name: String,
    pub comment: String,
    pub fields: Vec<SField>,
}

#[derive(Serialize,Deserialize)]
#[derive(Default)]
pub struct SUnion {
    pub name: String,
    pub comment: String,
    pub fields: Vec<SField>,
}

#[derive(Serialize,Deserialize)]
#[derive(Default)]
pub struct SEnum {
    pub name: String,
    pub comment: String,
    pub variants: Vec<SVariant>,
}

#[derive(Serialize,Deserialize)]
#[derive(Default)]
pub struct SVariant {
    pub name: String,
    pub comment: String,
    pub fields: Vec<SField>,
}

#[derive(Serialize,Deserialize)]
#[derive(Default)]
pub struct SField {
    pub name: String,
    pub comment: String,
    pub tyx: STypeExpr,    
}

#[derive(Serialize,Deserialize)]
#[derive(Default)]
pub struct SAlias {
    pub name: String,
    pub comment: String,
    pub tyx: STypeExpr,
}

#[derive(Serialize,Deserialize)]
#[derive(Default)]
pub struct STrait {
    pub name: String,
    pub comment: String,
    pub decls: Vec<STraitDecl>,
}

#[derive(Serialize,Deserialize)]
pub enum STraitDecl {
    Function(SFunction),
    Unknown(String),
}

#[derive(Serialize,Deserialize)]
#[derive(Default)]
pub struct SFunction {
    pub name: String,
    pub comment: String,
    pub recv: Option<STypeExpr>,
    pub args: Vec<SFunctionArg>,
    pub ret: STypeExpr,
}

#[derive(Serialize,Deserialize)]
#[derive(Default)]
pub struct SFunctionArg {
    pub name: Option<String>,
    pub tyx: STypeExpr,
}

pub type SModulePath = String;
pub type SName = String;
pub type STypeExpr = String;







impl std::fmt::Display for SCrate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        writeln!(f, "crate {}", self.name)?;
        for x in &self.modules {
            writeln!(f, "{}", x)?;
        }
        Ok(())
    }
}
impl std::fmt::Display for SModule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        writeln!(f, "    mod {}", self.path)?;
        for x in &self.decls {
            writeln!(f, "{}", x)?;
        }
        Ok(())
    }
}
impl std::fmt::Display for SModuleDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        use SModuleDecl::*;
        match self {
            Struct(x) => write!(f, "{}", x)?,
            Union(x) => write!(f, "{}", x)?,
            Enum(x) => write!(f, "{}", x)?,
            Alias(x) => write!(f, "        {}", x)?,
            Trait(x) => write!(f, "{}", x)?,
            Function(x) => write!(f, "        {}", x)?,
            Unknown(x) => write!(f, "        {}", x)?,
        }
        Ok(())
    }
}
impl std::fmt::Display for SStruct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        writeln!(f, "        struct {}", self.name)?;
        for field in &self.fields {
            writeln!(f, "            {}", field)?;
        }
        Ok(())
    }
}
impl std::fmt::Display for SUnion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        writeln!(f, "        uinon {}", self.name)?;
        for field in &self.fields {
            writeln!(f, "            {}", field)?;
        }
        Ok(())
    }
}
impl std::fmt::Display for SEnum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        writeln!(f, "        enum {}", self.name)?;
        for variant in &self.variants {
            writeln!(f, "            {}", variant)?;
        }
        Ok(())
    }
}
impl std::fmt::Display for STrait {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        writeln!(f, "        trait {}", self.name)?;
        for decl in &self.decls {
            writeln!(f, "            {}", decl)?;
        }
        Ok(())
    }
}
impl std::fmt::Display for STraitDecl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        use STraitDecl::*;
        match self {
            Function(x) => writeln!(f, "{}", x)?,
            Unknown(x) => writeln!(f, "{}", x)?,
        }
        Ok(())
    }
}






impl std::fmt::Display for SAlias {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "type {} = {}", self.name, self.tyx)?;
        Ok(())
    }
}
impl std::fmt::Display for SVariant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{}", self.name)?;
        if !self.fields.is_empty() {
            let c = self.fields.len() - 1;
            write!(f, "(")?;
            for i in 0..c {
                let field = &self.fields[i];
                write!(f, "{}", field)?;
                write!(f, ",")?;
            }    
            write!(f, "{}", self.fields.last().unwrap())?;
            write!(f, ")")?;
        }
        write!(f, "")?;
        Ok(())
    }
}
impl std::fmt::Display for SField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "{}: {}", self.name, self.tyx)?;
        Ok(())
    }
}
impl std::fmt::Display for SFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "fn {}", self.name)?;
        write!(f, "(")?;
        if let Some(recv) = &self.recv {
            write!(f, "{}", recv)?;
            if !self.args.is_empty() {
                write!(f, ",")?;
            }
        }
        if !self.args.is_empty() {
            let c = self.args.len() - 1;
            for i in 0..c {
                let arg = &self.args[i];
                write!(f, "{}", arg)?;
                write!(f, ",")?;
            }    
            write!(f, "{}", self.args.last().unwrap())?;
        }
        write!(f, ")")?;
        write!(f, " -> ")?;
        write!(f, "{}", self.ret)?;
        Ok(())
    }
}
impl std::fmt::Display for SFunctionArg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        let name = self.name.as_ref().map(String::as_str).unwrap_or("_");
        write!(f, "{}: {}", name, self.tyx)?;
        Ok(())
    }
}

