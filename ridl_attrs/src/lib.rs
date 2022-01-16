//! Marking attributes for RIDL's REST-like API support.
//! This marks will be collected by scanner, and affect code-gen.
//! For OpenAPI3 code-gen, this will produce Parameter / Responses objects.
//!
//!     #[rest(in)]
//!     pub struct OrderItemGetInput {
//!         #[path] pub id: String,
//!         #[query] pub deep: bool,
//!     }
//!
//!     #[rest(out)]
//!     pub enum OrderItemGetOutput {
//!         #[status(200)] OK(Order),
//!         #[status(401)] Unauthorized(APIError),
//!     }
//! 
//!     #[rest(in)]
//!     pub struct OrderPostInput {
//!         #[body] pub form: OrderForm,
//!     }
//!
//!     #[rest(out)]
//!     pub enum OrderPostOutput {
//!         #[status(200)] OK(Order),
//!         #[status(401)] Unauthorized(APIError),
//!         #[status(500)] 
//!         #[mime("application/octet-stream")] 
//!         RawServerError(Vec<u8>)
//!     }
//!

extern crate proc_macro;
use proc_macro::TokenStream;
 
#[proc_macro_attribute] pub fn rest(_attr: TokenStream, item: TokenStream) -> TokenStream { item }
#[proc_macro_attribute] pub fn path(_attr: TokenStream, item: TokenStream) -> TokenStream { item }
#[proc_macro_attribute] pub fn status(_attr: TokenStream, item: TokenStream) -> TokenStream { item }
#[proc_macro_attribute] pub fn mime(_attr: TokenStream, item: TokenStream) -> TokenStream { item }






// !     #[rest(GET="/order/item")]
// !     type OrderItemGet = dyn Fn(OrderItemGetInput) -> OrderItemGetOutput;
// ! 
// !     #[rest("/order/item")]
// !     trait OrderItem {
// !         fn get(_:OrderItemGetInput) -> OrderItemGetOutput;
// !         fn post(_:OrderItemPostInput) -> OrderItemPostOutput;
// !     }
// ! 