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
