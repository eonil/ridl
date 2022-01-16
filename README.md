RIDL
====
Eonil, 2022.

[![Rust](https://github.com/eonil/ridl/actions/workflows/rust.yml/badge.svg)](https://github.com/eonil/ridl/actions/workflows/rust.yml)

Rust code as IDL.

- Define your interface data schema in Rust.
- Your schema will be checked by the Rust type system.
- Ruthlessly precise. Nothing ambiguous.




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
This Rust code.
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
      description: Edible objects.
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

No support for modules or namespaces.
All types will be placed in single root namespace.






Design Choices
--------------
- Scan Rust code and collect minified schema.
- Mini schema is limited feature subset of Rust type system.
- Generates other language codes from mini schema.
- No module/namespace support.
    - Everything will go to a top-level flatten namespace.
- No `serde` behavior customization support.
    - `serde` behavior is fully customizable by users.
    - It's impossible to reproduce all possibilities.
- No semantic analysis.
    - No name reference resolution or dependency check.
    - As Rust compiler will check all for them we don't need duplicated features.
- Sum-type serialization follows default form of Rust/Swift/Swithy.




Supported Schema Model
----------------------
Only these things are supported.
- Primitve types. (`bool`, `i32`, `f64`, `String`, JSON container does not support `i64` or `f32`)
- New-type. (`type`, type-alias in fact)
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






Camel Case Renaming
-------------------
Rust's default field naming is `snake_case`.
This is fine if your system is all in Rust, but can be a problem if you need to interact with other languages.
In other languages, dominant convention is `camelCase`.
You can rename all fields to `camelCase` with command line option like this.

    ridl swift5 --rename camel
  
All enum-type cases, sum-type variants and prod-type fields will be renamed accordingly.
This does not modify existing Rust code. 
You are responsible to make Rust code to produce `camelCase`d output.







Schema Export
-------------
- Scanned minified RIDL schema can be exported as an YAML file.
- You can use the file to make your own automation tool.
- RIDL schema is precisely defined in `src/model`.






REST Attribute Support
----------------------
RIDL provides scanning of extra annoatation for REST input/output.
This is going to affect OpenAPI3 code-gen.
Annotated types will be decomposed and generate Parameters, RequestBody, Responses objects
instead of Schema object.

```rust
#[derive(RIDL)]
[rest(in)]
struct Input {
    #[query]
    walk: bool,
    #[path]
    living_address: Option<Address>,
}
#[derive(RIDL)]
[rest(out)]
enum Output {
    #[status(200)]
    #[mime("application/json")]
    Sushi(Tuna),
    #[status(401)]
    #[mime("application/json")]
    PanFriedSteak(Salmon),
}
```







License
-------
Using this code is licensed under "MIT License".
Contributions will also be licensed under same license.
Contributions mean agreement on this licensing terms.
Copyright(c) 2022, Eonil. All rights reserved.