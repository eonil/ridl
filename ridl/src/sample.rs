#![allow(unused)]

use ridl_derive::*;



#[derive(RIDL)]
#[rest(in)]
struct AAA {
    #[query]
    field1: String,
}

#[derive(RIDL)]
#[rest(out)]
enum BBB {
    #[status(200)]
    OK(String),
    #[status(401)]
    Err1(i64),
}
