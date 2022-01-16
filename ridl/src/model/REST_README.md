STILL IN CONSIDERATION
======================
REST support is not needed for RIDL's primary goal, and difficult to define.
I'm still considering REST support, but I see no benefot by supporting REST in RIDL.
Instead, I recommend to use OpenAPI3 with separated files.

- Generate OpenAPI3 message types definition file with RIDL.
- Define REST function APIs in annother OpenAPI3 file.
  - Refer message types from the generated file.

With this way, you have to define path parameters and query string in OpenAPI3 file.
But those things are **limited to primitive types** and not very worth to define in RIDL.
Also, it's very difficult to represent those parameters in Rust code.
we cannot define path parameter query string





Secondary REST Support
----------------------
RIDL is primarily designed for streaming message I/O. (e.g. pub/sub)
Although REST support is not a primary goal, but needed to utilize existing OpenAPI based tools.
RIDL has primitive level of REST code-gen support that will be enough for most cases.
At this point, REST code-gen is supported only in these targets.

- OpenAPI3.
- Swift5.

Also, there's no Rust-based syntax for REST API definitions.
Designing and implementing in-Rust syntax for REST API takes too much cost without semantic analysis.
You need to define REST API in YAML and pass it with `--rest` extra option.

    ridl openapi3 --in src.rs --out dst.yaml --rest rest.yaml

Model for the REST support is defined in `::ridl::model::rest` module.
Here's an example.

```yaml
----
functions:
  - comment: | 
      Submits an order.
    attributes:
      - private
    input:
      method: POST
      path: /order/submit
      query: OrderSubmitInput
    output:
      - status: 200
        body: OrderSubmitOutput
        comment: OK.
      - status: 401
        body: APIError
        comment: Unauthorized.
```
