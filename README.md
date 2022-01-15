RIDL
====
Eonil, 2022.

Rust as IDL.



Quickstart
----------
Sum-type.
```rust
#[ridl(tag=type)]
enum Fish {
    Tuna(Tuna),
    Salmon(Salmon),
}
type Tuna = String;
type Salmon = String;
```



Caveats
-------
First version won't perform semantic analysis.
Scans only at syntax level. Therefore, *RIDL won't solve types aliases*.
**Use type names directly.**



Attributes
----------
`#[ridl tag=ident]`
- Optional attribute to control form of serializtion.
- If defined, RIDL assumes the type will be serialized in ["internally tagged" (serde)](https://serde.rs/enum-representations.html#internally-tagged) manner.
- Therefore generates code to read/write internally tagged form of data instaces.











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

