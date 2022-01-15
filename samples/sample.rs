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
