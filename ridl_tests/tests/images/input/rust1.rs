//! Here be dragons.

type Tuna = String;
type Salmon = String;

/// Ingredients of magic.
#[serde(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
enum Mineral {
    IronMetal,
    Alumina,
    Arcana,
}

#[rest(GET,"/pet/dish")]
type pet_dish = dyn Fn(Pet) -> Dish;

#[serde(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
#[rest(in)]
struct Pet {
    name: Vec<String>,
    /// Did they take a walk today?
    #[query]
    walk: bool,
    #[path]
    living_address: Option<Address>,
    #[body]
    #[mime("application/octet-stream")]
    contents: Vec<u8>,
}

#[serde(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
struct Address {
    city: String,
}

/// Edibles.
#[serde(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
#[rest(out)]
enum Dish {
    #[status(200)]
    #[mime("application/json")]
    Sushi(Tuna),
    /// Good salmons are reddish.
    #[status(401)]
    #[mime("application/json")]
    PanFriedSteak(Salmon),
}

#[serde(Serialize, Deserialize)]
#[serde(rename_all="camelCase")]
struct APIError {
    message: String,
}
