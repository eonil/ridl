/// Here be dragons.
type Tuna = string

/// Ingredients of magic.
enum Mineral {
    ironMetal = "ironMetal",
    alumina = "alumina",
    arcana = "arcana"  
}

type Pet = {
    name: string[]
    /// Did they take a walk today?
    walk: boolean
    livingAddress?: Address
    contents: u8[]
}

type Address = {
    city: string
}

/// Edibles.
type Dish = { sushi: Tuna } | { panFriedSteak: Salmon }

type APIError = {
    message: string
}


