//! Marking attributes for RIDL's REST-like API support.
//! This marks will be collected by scanner, and affect code-gen.
//! For OpenAPI3 code-gen, this will produce Parameter / Responses objects.
//!
//!     use ridl_derive::RIDL;
//!     
//!     #[derive(RIDL)]
//!     #[rest(in)]
//!     pub struct OrderItemGetInput {
//!         #[path] pub id: String,
//!         #[query] pub deep: bool,
//!     }
//!
//!     #[derive(RIDL)]
//!     #[rest(out)]
//!     pub enum OrderItemGetOutput {
//!         #[status(200)] OK(Order),
//!         #[status(401)] Unauthorized(APIError),
//!     }
//! 
//!     #[derive(RIDL)]
//!     #[rest(in)]
//!     pub struct OrderPostInput {
//!         #[body] pub form: OrderForm,
//!     }
//!
//!     #[derive(RIDL)]
//!     #[rest(out)]
//!     pub enum OrderPostOutput {
//!         #[status(200)] OK(Order),
//!         #[status(401)] Unauthorized(APIError),
//!         #[status(500)] 
//!         #[mime("application/octet-stream")] 
//!         RawServerError(Vec<u8>)
//!     }
//! 
//!     pub struct Order;
//!     pub struct OrderForm;
//!     pub struct APIError;
//! 

extern crate proc_macro;
use proc_macro::TokenStream;

/// RIDL derive macro.
/// - Though this does nothing, still required to mark helper attributes.
/// - Otherwise, you gonna see weird errors.
/// 
/// There are multiple discussions related to this issue.
/// - https://github.com/rust-lang/rust/issues/65823
#[proc_macro_derive(RIDL, attributes(rest, query, status, path, status, mime))]
pub fn derive_ridl(_item: TokenStream) -> TokenStream {
    TokenStream::new()
}



// !     #[rest(GET="/order/item")]
// !     type OrderItemGet = dyn Fn(OrderItemGetInput) -> OrderItemGetOutput;
// ! 
// !     #[rest("/order/item")]
// !     trait OrderItem {
// !         fn get(_:OrderItemGetInput) -> OrderItemGetOutput;
// !         fn post(_:OrderItemPostInput) -> OrderItemPostOutput;
// !     }
// ! 