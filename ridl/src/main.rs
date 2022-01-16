use ::ridl::*;

use structopt::StructOpt;

type Error = Box<dyn std::error::Error>;
type Result<T> = std::result::Result<T,Error>;

#[derive(StructOpt)]
#[structopt(name="RIDL", about="Rust code as IDL.")]
struct Opt {
    /// Target language.
    language: Language,

    /// Path to a `.rs` file.
    /// Please note that only certain subset will be supported.
    #[structopt(long="in")]
    input: Option<String>,
    /// Path to write generated code code.
    /// RIDL won't produce target code if this is not designated.
    /// Then effectively performs only lint stage.
    #[structopt(long="out")]
    output: Option<String>,

    /// Skipping type names.
    /// RIDL won't make code for types with names in `skippings`.
    /// You are supposed to provide type definitions yourself using <prelude> option.
    #[structopt(long="skip")]
    skippings: Vec<String>,

    /// Rename field names to one of these cases.
    /// For now the only supported option is `camel` which means `camelCase`.
    #[structopt(long="rename")]
    rename: Option<model::rename::Rule>,
}
#[derive(strum_macros::EnumString)]
enum Language {
    #[strum(serialize="ridl1")]
    RIDL1,
    #[strum(serialize="openapi3")]
    OpenAPI3,
    #[strum(serialize="swift5")]
    Swift5,
    #[strum(serialize="typescript4")]
    TypeScript4,
}

fn main() {
    match run() {
        Ok(_) => std::process::exit(0),
        Err(x) => {
            eprintln!("{}", x);
            std::process::exit(1);
        },
    }
}
fn run() -> Result<()> {
    let opt = Opt::from_args();
    let opt = &opt;
    let src = match &opt.input {
        None => read_all_from_stdin()?,
        Some(x) => read_all_from_file(&x)?,
    };

    let ast = syn::parse_str::<syn::File>(&src)?;
    let mut model = scan::scan(&ast)?;
    model.retain_only_non_skipping_items(&opt.skippings);
    model.rename(&model::rename::Options { 
        case: opt.rename,
        variant: opt.rename,
        field: opt.rename,
    });
    
    let dst = match &opt.language {
        Language::RIDL1 => render::ridl1::render_ridl1(&model)?,
        Language::OpenAPI3 => render::openapi3::render_openapi3(&model)?,
        Language::Swift5 => render::swift5::render_swift5(&model)?,
        Language::TypeScript4 => render::typescript4::render_typescript4(&model)?,
    };

    match &opt.output {
        None => write_all_to_stdout(&dst)?,
        Some(x) => write_all_to_file(&dst, &x)?,
    }

    Ok(())
}

fn read_all_from_stdin() -> Result<String> {
    use std::io::Read;
    let mut s = String::new();
    std::io::stdin().read_to_string(&mut s)?;
    Ok(s)
}
fn read_all_from_file(path: &str) -> Result<String> {
    let s = std::fs::read_to_string(path)?;
    Ok(s)
}

fn write_all_to_stdout(s:&str) -> Result<()> {
    use std::io::Write;
    std::io::stdout().write(s.as_bytes())?;
    Ok(())
}
fn write_all_to_file(s:&str, path:&str) -> Result<()> {
    std::fs::write(path, s)?;
    Ok(())
}
