RIDL
====
Eonil, 2022.

Rust code as IDL.



Quickstart
----------
Prepare a Rust code file (`.rs`), and call command.
This command generates Swift5 code.

    ridl swift5 --in src.rs --out dst.swift

You also can pipe.

    cat src.rs | ridl swift5 > dst.swift

See files in `tests/images` for actual samples.




Code-Gen Illustrated
--------------------
This rust code.
```rust
enum Fish {
    Tuna(Tuna),
    Salmon(Salmon),
}
type Tuna = String;
type Salmon = String;
```

Becomes this OpenAPI3 schema.
```yaml
---
openapi: 3.0.1
info:
  title: ""
  description: "Here be dragons.\nFeatures below this line are currently supported."
  version: ""
paths: {}
components:
  schemas:
    Tuna:
      type: string
    Salmon:
      type: string
    Fish:
      title: Fish
      type: object
      oneOf:
        - properties:
            Tuna:
              $ref: "#/components/schemas/Tuna"
        - properties:
            Salmon:
              $ref: "#/components/schemas/Salmon"
          description: Good salmons are reddish.
      description: "Edible objects.\n\nGood salmons are reddish."
```

And this Swift code.
```swift
typealias Tuna = String
typealias Salmon = String
/// Edible objects.
enum Fish: Equatable, Codable {
    case Tuna(Tuna)
    /// Good salmons are reddish.
    case Salmon(Salmon)
}
```

And this TypeScript code.
```typescript
type Tuna = string
type Salmon = string
/// Edible objects.
type Fish = { Tuna: Tuna } | { Salmon: Salmon }
```





Caveats
-------
RIDL won't perform semantic analysis.
Scans only at syntax level. Therefore, *RIDL won't solve types aliases*.
**Use type names directly.**







Design Choices
--------------
- Scan Rust code and collect minified schema.
- Mini schema is limited feature subset of Rust type system.
- Generates other language codes from mini schema.
- No module/namespace support.
    - Everything will go to a top-level flatten namespace.
- No `serde` attribute support.
    - `serde` behavior is fully customizable by users. 
    - It's impossible to reproduce all possibilities.
- No semantic analysis.
    - No name reference resolution or dependency check.
    - As Rust compiler will check all for them we don't need duplicated features.
- Sum-type serialization follows default form of Rust/Swift/Swithy.




Supported Schema Model
----------------------
Only these things are supported.
- Primitve types. (`bool`, `i32`, `i64`, `f32`, `f64`, `String`)
- New-type. (`type`)
- Enum-type. (`enum`, finite constant set)
- Sum-type. (`enum`, tagged union, variant name-based discrimination)
- Product-type. (`struct`)




Code-Gen Skipping
-----------------
Schema is a declarative representation of data structures.
It's difficult or inefficient to represent everything in declarative form.
You frequently need to define special types with special behaviors.
For that, you can skip code-gen for certain types.

Here's a command example.
```sh
cat prelude.swift > dst.swift
cat src.rs | ridl swift5 --skip Tuna --skip Salmon >> dst.swift
```

Now generated `dst.swift` file does not contain definitions for 
`Tuna` and `Salmon`. You can provide your custom code to import
your custom implementation in `prelude.swift` file.
x
With this `prelude.swift` file.
```swift
import Hawaii
typealias Tuna = HawaiianTuna
typealias Salmon = HawaiianSalmon
```

RIDL generates following parts to `dst.swift` file.
```swift
/// Edible objects.
enum Fish: Equatable, Codable {
    case Tuna(Tuna)
    /// Good salmons are reddish.
    case Salmon(Salmon)
}
```

Therefore combined, you can bind your custom implementation 
to the generated code.







License
-------
Using this code is licensed under "MIT License".
Contributions will also be licensed under same license.
Contributions mean agreement on this licensing terms.
Copyright(c) 2022, Eonil. All rights reserved.