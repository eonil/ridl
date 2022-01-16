#![cfg(test)]

mod rest;

use ::ridl::*;
// use crate::model;
use crate::scan::*;
use crate::render;
use crate::render::openapi3::render_openapi3;
use crate::render::swift5::render_swift5;
use crate::render::typescript4::render_typescript4;

#[test]
fn test_scan_model() {
    let a = include_str!("images/input/rust1.rs");
    let b = syn::parse_str::<syn::File>(&a).unwrap();
    scan(&b).unwrap();
}
#[test]
fn test_model_serde_roundtripping() {
    let a = include_str!("images/input/rust1.rs");
    let b = syn::parse_str::<syn::File>(&a).unwrap();
    let m = scan(&b).unwrap();
    let x = render::ridl1::render_ridl1(&m).unwrap();
    let c = serde_yaml::from_str::<model::KMod>(&x).unwrap();
    assert_eq!(m, c);
}

#[test]
fn test_render_openapi3() {
    let a = include_str!("images/input/rust1.rs");
    let b = syn::parse_str::<syn::File>(&a).unwrap();
    let m = scan(&b).unwrap();
    let x = render_openapi3(&m).unwrap().trim().to_owned();
    let z = include_str!("images/output/openapi3").trim();
    assert_eq!(x, z);
}
#[test]
fn test_render_swift5() {
    let a = include_str!("images/input/rust1.rs");
    let b = syn::parse_str::<syn::File>(&a).unwrap();
    let m = scan(&b).unwrap();
    let x = render_swift5(&m).unwrap().trim().to_owned();
    let z = include_str!("images/output/swift5").trim();
    assert_eq!(x, z);
}
#[test]
fn test_render_typescript4() {
    let a = include_str!("images/input/rust1.rs");
    let b = syn::parse_str::<syn::File>(&a).unwrap();
    let m = scan(&b).unwrap();
    let x = render_typescript4(&m).unwrap().trim().to_owned();
    let z = include_str!("images/output/typescript4").trim();
    assert_eq!(x, z);
}

#[test]
#[should_panic]
fn test_bad_rust_code() {
    let a = include_str!("images/error/bad_sum_type.rs");
    let b = syn::parse_str::<syn::File>(&a).unwrap();
    scan(&b).unwrap();
}