use std::sync::Arc;
use base_db::Upcast;
use hir_ty::db::HirDatabase;

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
    // let mut h = ra::AnalysisHost::new(None);
    // let mut fseed = Seed(0);
    let prod_cargo_toml_path = std::env::current_dir()?.parent().ok_or(Missing("bad path"))?.join("ridl").join("Cargo.toml");
    let manifest = ra::ProjectManifest::from_manifest_file(ra::AbsPathBuf::assert(prod_cargo_toml_path))?;
    fn progress(s:String) { println!("{}", s) }
    let ws = ra::ProjectWorkspace::load(manifest, &ra::CargoConfig::default(), &progress)?;
    let conf = ra::LoadCargoConfig {
        load_out_dirs_from_check: false,
        with_proc_macro: false,
        prefill_caches: false,
    };
    let (h,vfs,_) = ra::load_workspace(ws, &conf)?;
    // let mut x = ra::Change::default();
    // x.set_crate_graph(cg);
    // h.apply_change(x);
    let db = h.raw_database().upcast() as &dyn HirDatabase;
    step(db)?;
    Ok(())
    
    // let core_root_path = find_syslib_root_path("core")?;
    // let core_file_paths = collect_all_file_paths(&core_root_path)?;
    // let std_root_path = find_syslib_root_path("std")?;
    // let std_file_paths = collect_all_file_paths(&std_root_path)?;
    // let prod_root_path = std::env::current_dir()?.parent().ok_or(Missing("bad path"))?.join("ridl").join("src");
    // let prod_file_paths = collect_all_file_paths(&prod_root_path)?;
    // let mut all_change = ra::Change::default();
    // let mut core_fset = ra::FileSet::default();
    // let mut std_fset = ra::FileSet::default();
    // let mut prod_fset = ra::FileSet::default();
    // for core_file_path in core_file_paths.iter() {
    //     if core_file_path.extension().unwrap_or_default() != "rs" { continue }
    //     let fid = fseed.issue();
    //     let vpath = vfs_path(&core_file_path)?;
    //     let fcode = std::fs::read_to_string(&core_file_path)?;
    //     core_fset.insert(fid, vpath);
    //     all_change.change_file(fid, Some(Arc::new(fcode)));
    // }
    // for std_file_path in std_file_paths.iter() {
    //     if std_file_path.extension().unwrap_or_default() != "rs" { continue }
    //     let fid = fseed.issue();
    //     let vpath = vfs_path(&std_file_path)?;
    //     let fcode = std::fs::read_to_string(&std_file_path)?;
    //     std_fset.insert(fid, vpath);
    //     all_change.change_file(fid, Some(Arc::new(fcode)));
    // }
    // for prod_file_path in prod_file_paths.iter() {
    //     if prod_file_path.extension().unwrap_or_default() != "rs" { continue }
    //     let fid = fseed.issue();
    //     let vpath = vfs_path(&prod_file_path)?;
    //     let fcode = std::fs::read_to_string(&prod_file_path)?;
    //     prod_fset.insert(fid, vpath);
    //     all_change.change_file(fid, Some(Arc::new(fcode)));
    // }
    // all_change.set_roots(vec![
    //     ra::SourceRoot::new_local(core_fset),
    //     ra::SourceRoot::new_local(std_fset),
    //     ra::SourceRoot::new_local(prod_fset),
    // ]);
    // h.apply_change(all_change);

    // let db = h.raw_database().upcast() as &dyn HirDatabase;
    // step(db)?;

    // Ok(())








    // let mut h = ra::AnalysisHost::new(None);
    // let core_root_path = find_syslib_root_path("core")?;
    // let core_file_paths = collect_all_file_paths(&core_root_path)?;
    // let std_root_path = find_syslib_root_path("std")?;
    // let std_file_paths = collect_all_file_paths(&std_root_path)?;
    // let prod_root_path = std::env::current_dir()?.parent().ok_or(Missing("bad path"))?.join("ridl").join("src");
    // let prod_file_paths = collect_all_file_paths(&prod_root_path)?;
    // let mut all_change = ra::Change::default();
    // let mut core_fset = ra::FileSet::default();
    // let mut std_fset = ra::FileSet::default();
    // let mut prod_fset = ra::FileSet::default();
    // let mut cg = ra::CrateGraph::default();
    // for core_file_path in core_file_paths.iter() {
    //     if core_file_path.extension().unwrap_or_default() != "rs" { continue }
    //     let fid = fseed.issue();
    //     let vpath = vfs_path(&core_file_path)?;
    //     let fcode = std::fs::read_to_string(&core_file_path)?;
    //     core_fset.insert(fid, vpath);
    //     all_change.change_file(fid, Some(Arc::new(fcode)));
    // }
    // for std_file_path in std_file_paths.iter() {
    //     if std_file_path.extension().unwrap_or_default() != "rs" { continue }
    //     let fid = fseed.issue();
    //     let vpath = vfs_path(&std_file_path)?;
    //     let fcode = std::fs::read_to_string(&std_file_path)?;
    //     std_fset.insert(fid, vpath);
    //     all_change.change_file(fid, Some(Arc::new(fcode)));
    // }
    // for prod_file_path in prod_file_paths.iter() {
    //     if prod_file_path.extension().unwrap_or_default() != "rs" { continue }
    //     let fid = fseed.issue();
    //     let vpath = vfs_path(&prod_file_path)?;
    //     let fcode = std::fs::read_to_string(&prod_file_path)?;
    //     prod_fset.insert(fid, vpath);
    //     all_change.change_file(fid, Some(Arc::new(fcode)));
    // }
    // let core_lib_rs_fid = find_fid(&core_fset, &core_root_path.join("lib.rs"))?;
    // let std_lib_rs_fid = find_fid(&std_fset, &std_root_path.join("lib.rs"))?;
    // let prod_lib_rs_fid = find_fid(&prod_fset, &prod_root_path.join("lib.rs"))?;
    // let core_cid = cg.add_crate_root(
    //     core_lib_rs_fid,
    //     ra::Edition::Edition2021,
    //     None, 
    //     None,
    //     cfg::CfgOptions::default(), 
    //     cfg::CfgOptions::default(),
    //     base_db::Env::default(),
    //     Vec::new(),
    //     base_db::CrateOrigin::Lang);
    // let std_cid = cg.add_crate_root(
    //     std_lib_rs_fid,
    //     ra::Edition::Edition2021,
    //     None, 
    //     None,
    //     cfg::CfgOptions::default(), 
    //     cfg::CfgOptions::default(),
    //     base_db::Env::default(),
    //     Vec::new(),
    //     base_db::CrateOrigin::Lang);
    // let prod_cid = cg.add_crate_root(
    //     prod_lib_rs_fid,
    //     ra::Edition::Edition2021,
    //     None, 
    //     None,
    //     cfg::CfgOptions::default(), 
    //     cfg::CfgOptions::default(),
    //     base_db::Env::default(),
    //     Vec::new(),
    //     base_db::CrateOrigin::Unknown);
    // let core_dep = ra::Dependency::with_prelude(ra::CrateName::new("core")?, core_cid, true);
    // let std_dep = ra::Dependency::with_prelude(ra::CrateName::new("std")?, std_cid, true);
    // cg.add_dep(prod_cid, core_dep).map_err(|_|Missing("cycle-dependency"))?;
    // cg.add_dep(prod_cid, std_dep).map_err(|_|Missing("cycle-dependency"))?;
    // all_change.set_roots(vec![
    //     ra::SourceRoot::new_local(core_fset),
    //     ra::SourceRoot::new_local(std_fset),
    //     ra::SourceRoot::new_local(prod_fset),
    // ]);
    // all_change.set_crate_graph(cg);
    // h.apply_change(all_change);
    // let db = h.raw_database().upcast() as &dyn HirDatabase;
    // step(db)?;
    // Ok(())
}

struct Seed(u32);
impl Seed {
    fn issue(&mut self) -> ide::FileId {
        self.0 += 1;
        ide::FileId(self.0)
    }
}


fn find_fid(fset:&ra::FileSet, path:&Path) -> Result<ra::FileId> {
    let x = fset.file_for_path(&vfs_path(path)?).ok_or(Missing("no file id for the path"))?;
    Ok(*x)
}

fn vfs_path(p:&Path) -> Result<vfs::VfsPath> { 
    Ok(vfs::VfsPath::new_virtual_path(p.to_str().ok_or(Missing("bad path"))?.to_owned())) 
}

fn collect_all_file_paths(abspath:&Path) -> Result<PVec<PathBuf>> {
    let mut bucket = PVec::new();
    for fentry in std::fs::read_dir(abspath.clone())? {
        let fentry = fentry?;
        let fname = fentry.file_name();
        let ftype = fentry.file_type()?;
        let subpath = abspath.join(&fname);
        if ftype.is_file() {
            bucket.push_back(subpath)
        }
        else if ftype.is_dir() {
            let sub = collect_all_file_paths(&subpath)?;
            bucket.extend(sub);
        }
    }
    Ok(bucket)
}
fn step(db: &dyn HirDatabase) -> Result<()> {
    let ks = hir::Crate::all(db);
    for k in ks {
        println!("{:#?}", k.root_file(db));
        for module in k.modules(db).iter() {
            for decl in module.declarations(db) {
                aaa(&decl, db)?;
            }
        }
    }
    Ok(())
}
fn aaa(decl:& hir::ModuleDef, db:&dyn HirDatabase) -> Result<()> {
    // println!("- {}", decl.canonical_path(db).unwrap_or_default());
    let cpath = decl.canonical_path(db).ok_or(Missing("canonical modul path"))?;
    println!("{}", cpath);
    use hir::ModuleDef::*;
    use hir::Adt::*;
    match decl {
        Adt(Struct(x)) => {
            for field in x.fields(db) {
                use hir::HirDisplay;
                println!("- {}: {},", field.name(db), field.ty(db).display(db));
            }
        },
        // TypeAlias(x) => println!("  - {}", x.name(db)),
        _ => (),
    }
    Ok(())
}

type PVec<T> = im_rc::vector::Vector<T>;
use std::path::{Path,PathBuf};














fn find_syslib_root_path(name:&str) -> Result<PathBuf> {
    let out = std::process::Command::new("sh").args(["-c", "rustc --print sysroot"]).output()?;
    if !out.status.success() { return Err(Box::new(Missing("rust sysroot"))) }
    let sysroot = std::str::from_utf8(&out.stdout[..])?.trim();
    let stdlib_root_path = PathBuf::from(sysroot).join(format!("lib/rustlib/src/rust/library/{}/src", name));
    // println!("{}", stdlib_root_path.to_str().unwrap_or_default());
    Ok(stdlib_root_path)
}