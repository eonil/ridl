mod scan2;
 
use base_db::Upcast;
use hir_ty::db::HirDatabase;
use scan2::model::*;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

mod ra {
    pub use ide::FileId;
    pub use ide::SourceRoot;
    pub use ide::CrateGraph;
    pub use ide::Change;
    pub use ide::Edition;
    pub use base_db::Dependency;
    pub use base_db::ProcMacro;
    pub use ide::AnalysisHost;
    pub use vfs::file_set::FileSet;
    pub use vfs::VfsPath;
    pub use base_db::CrateName;
    pub use project_model::ProjectManifest;
    pub use project_model::ProjectWorkspace;
    pub use paths::AbsPathBuf;
    pub use paths::AbsPath;
    pub use project_model::CargoConfig;
    pub use rust_analyzer::cli::load_cargo::load_workspace;
    pub use rust_analyzer::cli::load_cargo::LoadCargoConfig;
    pub use vfs::Vfs;
}

#[derive(Debug)]
struct Missing(&'static str);
impl std::error::Error for Missing {}
impl std::fmt::Display for Missing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> { 
        write!(f, "unexpectedly missing value: {}", self.0)
    }
}

fn main() {
    run().unwrap();
}
fn run() -> Result<()> {
    let prod_root_path = std::env::current_dir()?.parent().ok_or(Missing("bad path"))?.join("ridl");
    let prod_cargo_toml_path = prod_root_path.join("Cargo.toml");
    let manifest = ra::ProjectManifest::from_manifest_file(ra::AbsPathBuf::assert(prod_cargo_toml_path))?;
    fn progress(s:String) { println!("{}", s) }
    let ws = ra::ProjectWorkspace::load(manifest, &ra::CargoConfig::default(), &progress)?;
    let conf = ra::LoadCargoConfig {
        load_out_dirs_from_check: false,
        with_proc_macro: false,
        prefill_caches: false,
    };
    let (h,vfs,_) = ra::load_workspace(ws, &conf)?;
    let db = h.raw_database().upcast() as &dyn HirDatabase;
    let ks = hir::Crate::all(db);
    let prod_root_vpath = ra::VfsPath::new_real_path(prod_root_path.to_str().ok_or(Missing("bad prod root path"))?.to_owned());
    for k in ks {
        let vpath = vfs.file_path(k.root_file(db));
        if vpath.starts_with(&prod_root_vpath) {
            println!("{:#?}", vpath);
            let scrate = scan_crate(&k,db);
            println!("{}", scrate);
        }
    }
    Ok(())
}





fn scan_crate(kcrate:&hir::Crate, db:&dyn HirDatabase) -> SCrate {
    let mut scrate = SCrate::default(); 
    for module in kcrate.modules(db).iter() {
        let smod = scan_module(&module,db);
        scrate.modules.push(smod);
    }
    scrate
}

fn scan_module(module:& hir::Module, db:&dyn HirDatabase) -> SModule {
    use hir::HirDisplay;
    use hir::ModuleDef::{Adt, Function, Module, Trait, TypeAlias};
    use hir::Adt::{Enum, Struct, Union};
    use scan2::model::*;
    let mut smod = SModule::default();
    smod.path = module.path_to_root(db).iter().rev().map(|x| x.name(db).as_ref().map(ToString::to_string).unwrap_or(String::new())).collect::<Vec<String>>().join("::");
    smod.name = module.name(db).as_ref().map(ToString::to_string).unwrap_or(String::new());
    for decl in module.declarations(db) {
        match decl {
            Module(_) => (),
            Adt(Struct(x)) => {
                for x in x.ty(db).fields(db) {
                    println!("TY: {}: {}", x.0.name(db), x.1.display(db));
                }
                println!("TY ARGS: {}", x.ty(db).type_arguments().count());
                for x in x.ty(db).type_arguments() {
                    println!("TY ARG: {}", x.display(db));
                }
                let mut sstruct = SStruct::default();
                sstruct.name = x.name(db).to_string();
                for field in x.fields(db) {
                    let mut sfield = SField::default();
                    sfield.name = field.name(db).to_string();
                    sfield.tyx = field.ty(db).display(db).to_string();
                    sstruct.fields.push(sfield);
                }
                smod.decls.push(SModuleDecl::Struct(sstruct));
            },
            Adt(Union(x)) => {
                let mut sunion = SUnion::default();
                sunion.name = x.name(db).to_string();
                for field in x.fields(db) {
                    let mut sfield = SField::default();
                    sfield.name = field.name(db).to_string();
                    sfield.tyx = field.ty(db).display(db).to_string();
                    sunion.fields.push(sfield);
                }
                smod.decls.push(SModuleDecl::Union(sunion));
            },
            Adt(Enum(x)) => {
                let mut senum = SEnum::default();
                senum.name = x.name(db).to_string();
                for variant in x.variants(db) {
                    let mut svariant = SVariant::default();
                    svariant.name = variant.name(db).to_string();
                    for field in variant.fields(db) {
                        let mut sfield = SField::default();
                        sfield.name = field.name(db).to_string();
                        sfield.tyx = field.ty(db).display(db).to_string();
                        svariant.fields.push(sfield);
                    }
                    senum.variants.push(svariant);
                }
                smod.decls.push(SModuleDecl::Enum(senum));
            },
            TypeAlias(x) => {
                let mut salias = SAlias::default();
                salias.name = x.name(db).to_string();
                salias.tyx = x.ty(db).display(db).to_string();
                smod.decls.push(SModuleDecl::Alias(salias));
            },
            Trait(x) => {
                let mut strait = STrait::default();
                strait.name = x.name(db).to_string();
                for item in x.items(db) {
                    match item {
                        hir::AssocItem::Function(x) => {
                            let sfunction = scan_function(&x,db);
                            strait.decls.push(STraitDecl::Function(sfunction));
                        },
                        _ => {
                            if let Some(name) = item.name(db) {
                                strait.decls.push(STraitDecl::Unknown(name.to_string()));
                            }
                        },
                    }
                }
                smod.decls.push(SModuleDecl::Trait(strait));
            }
            Function(x) => {
                let sfunction = scan_function(&x,db);
                smod.decls.push(SModuleDecl::Function(sfunction));
            }
            _ => {
                if let Some(cpath) = decl.canonical_path(db) {
                    smod.decls.push(SModuleDecl::Unknown(cpath));
                }
            },
        }
    }
    smod
}

fn scan_function(x:&hir::Function, db:&dyn HirDatabase) -> SFunction {
    use hir::HirDisplay;
    let mut sfunction = SFunction::default();
    sfunction.name = x.name(db).to_string();
    if let Some(param) = x.self_param(db) {
        sfunction.recv = Some(param.ty(db).display(db).to_string());
    }
    if let Some(params) = x.method_params(db) {
        for param in params {
            let mut sarg = SFunctionArg::default();
            if let Some(name) = param.name(db) {
                sarg.name = Some(name.to_string());
            }
            sarg.tyx = param.ty().display(db).to_string();
            sfunction.args.push(sarg);
        }
    }
    // for param in x.assoc_fn_params(db) {
    //     let mut sarg = SFunctionArg::default();
    //     if let Some(name) = param.name(db) {
    //         sarg.name = Some(name.to_string());
    //     }
    //     sarg.tyx = param.ty().display(db).to_string();
    //     sfunction.args.push(sarg);
    // }
    sfunction.ret = x.ret_type(db).display(db).to_string();
    sfunction
}




// fn print_decl(decl:& hir::ModuleDef, db:&dyn HirDatabase) -> Result<()> {
//     use hir::HirDisplay;
//     use hir::ModuleDef::*;
//     use hir::Adt::*;
//     match decl {
//         Module(_) => (),
//         Adt(Struct(x)) => {
//             println!("struct {}", x.name(db));
//             for field in x.fields(db) {
//                 println!("    {}: {}", field.name(db), field.ty(db).display(db));
//             }
//         },
//         Adt(Union(x)) => {
//             println!("union {}", x.name(db));
//             for field in x.fields(db) {
//                 println!("    struct {}: {}", field.name(db), field.ty(db).display(db));
//             }
//         },
//         Adt(Enum(x)) => {
//             println!("enum {}", x.name(db));
//             for variant in x.variants(db) {
//                 print!("    {}", variant.name(db));
//                 let params = variant.fields(db).iter()
//                     .map(|field| format!("{}:{}", field.name(db), field.ty(db).display(db)))
//                     .collect::<Vec<_>>()
//                     .join(", ");
//                 println!("({})", params);
//             }
//         },
//         TypeAlias(x) => {
//             println!("type {}", x.name(db));
//         },
//         Trait(x) => {
//             println!("trait {}", x.name(db));
//             for item in x.items(db) {
//                 match item {
//                     hir::AssocItem::Function(x) => {
//                         println!("    fn {}", x.name(db));
//                     },
//                     hir::AssocItem::Const(x) => {
//                         println!("    const {}", "????");
//                     },
//                     hir::AssocItem::TypeAlias(x) => {
//                         println!("    type {}", x.name(db));
//                     },
//                 }
//             }
//         }
//         Function(x) => {
//             println!("fn {}({}) -> {}", x.name(db), "????", x.ret_type(db).display(db));
//             let params = Vec::new();
//             if x.self_param(db).is_some() { params.push("self") };
//             for arg in x.method_params(db).iter() {
//                 arg
//                 arg.name(db) as str;
//             }
//             let args = x.method_params(db);
//             let ret = x.ret_type(db);
//             for variant in x.variants(db) {
//                 print!("    {}", variant.name(db));
//                 let params = variant.fields(db).iter()
//                     .map(|field| format!("{}:{}", field.name(db), field.ty(db).display(db)))
//                     .collect::<Vec<_>>()
//                     .join(", ");
//                 println!("({})", params);
//             }
//         }
//         _ => {
//             let cpath = decl.canonical_path(db).ok_or(Missing("canonical modul path"))?;
//             println!("unknown {} ", cpath);
//         },
//     }
//     Ok(())
// }











