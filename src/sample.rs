

// #[ridl::http::get("/ddd/ddd")]
// type index = dyn Fn(IndexInput) -> IndexOutput;

// struct IndexInput;
// struct IndexOutput;



// #[ridl::http(method="POST")]
// #[ridl::http(path="/shop/sushi")]
// #[ridl::http(access="userprivate")]
// mod order {

// }



// struct API {

// }
// pub struct OrderSubmit {
//     post: OrderSubmitPOST,
// }
// struct OrderSubmitPOST {
//     pub input: OrderSubmitPOSTInput,
//     pub output: OrderSubmitPOSTOutput,
// }
// struct OrderSubmitPOSTInput {
// }
// enum OrderSubmitPOSTOutput {
//     Status200([Order]),
//     Status400(APIError),
//     Status401(APIError),
// }                

// mod order {
//     mod submit {
//         pub mod GET {
//             use crate::sample::*;

//             struct Input {
//             }
//             enum Output {
//                 Status200([Order]),
//                 Status400(APIError),
//                 Status401(APIError),
//             }                
//         }
//     }
// }


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


// trait HTTPFunction {
//     const METHOD: &'static str;
//     const PATH: &'static str;
//     const ACCESS: HTTPAccess;
//     type Input: HTTPInput;
//     type Output: HTTPOutput;
// }
// enum HTTPAccess {
//     Public,
//     Restricted,
// }
// trait HTTPInput {   
//     type Query;
//     type Body;
// }
// trait HTTPOutput {
//     type Status200: HTTPOutputCase;
//     type Status400: HTTPOutputCase;
//     type Status401: HTTPOutputCase;
// }
// trait HTTPOutputCase {
//     type Body;
// }



// struct HTTPOutputCaseNever;
// impl HTTPOutputCase for HTTPOutputCaseNever {
//     type Body = String;
// }

// struct OrderSubmitGet;
// impl HTTPFunction for OrderSubmit {
//     const METHOD: &'static str = "GET";
//     const PATH: &'static str = "/order/submit";
//     const ACCESS: HTTPAccess = HTTPAccess::Restricted;
//     type Input = OrderSubmitGetInput;
//     type Output = OrderSubmitGetOutput;
// }
// struct OrderSubmitGetInput;
// impl HTTPInput for OrderSubmitGetInput {
//     type Query = String;
//     type Body = String;
// }
// struct OrderSubmitGetOutput;
// impl HTTPOutput for OrderSubmitGetOutput {
//     type Status200 = ();
//     type Status400 = ();
//     type Status401 = ();
// }




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








