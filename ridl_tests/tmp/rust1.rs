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
