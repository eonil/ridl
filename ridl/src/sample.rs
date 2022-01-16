#![allow(unused)]

use ridl_derive::*;

trait Service1 {
    #[rest(GET,"/service1/func1")]
    fn func1(&self, input:Func1Input) -> Func1Output;
}

#[derive(RIDL)]
#[rest(input)]
struct Func1Input {
    #[location(query)]
    field1: String,
    #[location(path)]
    field2: i32,
    #[location(body)]
    field3: i32,
}
#[derive(RIDL)]
#[rest(output)]
enum Func1Output {
    #[status(200)]
    Case1(String),
    #[status(401)]
    Case2(i32),
}



