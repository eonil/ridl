#![cfg(test)]

use indoc::indoc;
use quote::quote;
use crate::model;
use crate::scan::*;
use crate::render;
use crate::render::openapi3::render_openapi3;
use crate::render::swift5::render_swift5;
use crate::render::typescript4::render_typescript4;

#[test]
fn test_scan_model() {
    scan_model();
}
#[test]
fn test_model_serde_roundtripping() {
    let a = scan_model();
    let b = render::ridl1::render_ridl1(&a).unwrap();
    let c = serde_json::from_str::<model::KMod>(&b).unwrap();
    println!("{:#?}", a);
    assert_eq!(a, c);
}

#[test]
fn test_render_openapi3() {
    let m = scan_model();
    let x = render_openapi3(&m).unwrap();
    println!("{}", x);
}
#[test]
fn test_render_swift5() {
    let m = scan_model();
    let x = render_swift5(&m).unwrap();
    println!("{}", x);
}
#[test]
fn test_render_typescript4() {
    let m = scan_model();
    let x = render_typescript4(&m).unwrap();
    println!("{}", x);
}

fn scan_model() -> model::KMod {
    let a = make_sample_rust_code();
    let b = syn::parse_str::<syn::File>(&a).unwrap();
    scan(&b).unwrap()
}

fn make_sample_rust_code() -> String {
    quote! {
        //! Here be dragons.

        // #[ridl::http(method="POST")]
        // #[ridl::http(path="/shop/sushi")]
        // #[ridl::http(access="userprivate")]
        // type Func1 = dyn Fn(Tuna) -> Salmon;

        // #[ridl::http(method="GET")]
        // #[ridl::http(path="/members/pet")]
        // #[ridl::http(query="Pet::walk")]
        // #[ridl::http(query="Pet::address")]
        // #[ridl::http(error="404", type="APIError")]
        // type Func2 = dyn Fn(Pet) -> i32;
        // 
        // #[ridl::stream]
        // mod API2 {
        //     enum Input {
        //         TopicList,
        //         Subscribe,
        //         Unsubscribe,
        //     }
        //     enum Output {
        //         Insert(i32),
        //         Update(String),
        //         Delete(i32),
        //     }    
        // }
        
        //! Features below this line are currently supported.

        type Tuna = String;
        type Salmon = String;

        /// Ingredients of magic.
        enum Mineral {
            Iron,
            Alumina,
            Arcana,
        }

        /// Edible objects.
        #[ridl(tag="type")]
        enum Fish {
            Tuna(Tuna),
            /// Good salmons are reddish.
            Salmon(Salmon),
        }
        
        struct Pet {
            name: Vec<String>,
            /// Did they take a walk today?
            walk: bool,
            address: Option<Address>,
        }

        struct Address {
            city: String,
        }

        struct APIError {
            message: String,
        }

    }.to_string()
}