/// Here be dragons.
typealias Tuna = String

/// Ingredients of magic.
enum Mineral: String, Equatable, Codable {
    case ironMetal = "ironMetal"
    case alumina = "alumina"
    case arcana = "arcana"
}

struct Pet: Equatable, Codable {
    var name: [String]
    /// Did they take a walk today?
    var walk: Bool
    var livingAddress: Address?
    var contents: [u8]
}

struct Address: Equatable, Codable {
    var city: String
}

/// Edibles.
enum Dish: Equatable, Codable {
    case sushi(Tuna)
    /// Good salmons are reddish.
    case panFriedSteak(Salmon)
}

struct APIError: Equatable, Codable {
    var message: String
}


