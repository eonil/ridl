ridl_experiment
---------------



"Based on semantic analysis" means precise name resolution.
For semantic analysis, we need a compiler implementation.



How to use Rustc
----------------
- https://rustc-dev-guide.rust-lang.org/hir.html
- https://willcrichton.net/notes/type-directed-metaprogramming-in-rust/
- We need to provide all dependency crates to Rustc to make it work.
  - I don't know how to do it.
- `rustc_interface` is available **only in nightly compiler**.
  That is a certain level of barrier.



How to use Rust Analyzer
------------------------
- https://github.com/rust-analyzer/rust-analyzer/blob/master/docs/dev/guide.md
- Clone the RA Git repo.
- Build the doc. (`cargo doc --workspace --no-deps`)
- Open `ide` crate doc. (`target/doc/ide/index.html`)
- See `AnalysisHost`. You can create this instance.
  - `AnalysisHost` is work only with in-memory data.
  - You can provide file contents to the `AnalysisHost` methods.
- See `AnalysisHost::raw_database(&self)`. That returns `&RootDatabase`.
- See `RootDatabase::upcast(&self)`. That returns `&(dyn HirDatabase + 'static)`.
- Now you got a `HirDatabase` object.
- See `hir::Crate::all(db: &dyn HirDatabase)`. That returns `Vec<Crate>`.
- Now you have both of `Crate` and `HirDatabase` object.
- Notable functions.
  - `Crate::root_module`.
  - `Crate::modules`.
  - `Module::children`.
  - `Module::declarations`.
  - `hir::ModuleDef`.
- That must be good starting point.

RA is designed to allow errors, but precision is not proven.
RA also works with stable compiler. That's great.

This approach works well, but too slow.
Mainly because of mandatory reprocessing of whole core/std libs.
