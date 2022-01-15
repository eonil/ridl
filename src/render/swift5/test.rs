// use indoc::indoc;
// use syn::File;
// use quote::quote;
// use crate::scan::*;
// use super::render_swift5;
 
// #[test]
// fn scan_sum_type() {
//     let a = quote! {  

//         //! Here be dragons.

//         type Tuna = String;
//         type Salmon = String;

//         /// Ingredients of magic.
//         enum Mineral {
//             Iron,
//             Alumina,
//             Arcana,
//         }

//         /// Edible objects.
//         #[ridl tag="type"]
//         enum Fish {
//             Tuna(Tuna),
//             /// Good salmons are reddish.
//             Salmon(Salmon),
//         }
        
//         struct Pet {
//             name: Vec<String>,
//             /// Did they take a walk today?
//             walk: bool,
//             address: Option<Address>,
//         }

//         struct Address {
//             city: String,
//         }

//     }.to_string();
//     let b = syn::parse_str::<syn::File>(&a).unwrap();
//     let c = match scan(&b) {
//         Ok(x) => x,
//         Err(x) => {
//             println!("{:#?}", &x);
//             panic!();
//         },
//     };
//     let d = render_swift5(&c).unwrap();
//     println!("{}", d);
// }