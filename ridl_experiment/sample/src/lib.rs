#![no_std]

pub type Tuna = i64;
pub type Salmon = usize;

pub enum Fish {
    Tuna(Tuna),
    Salmon(Salmon),
}

struct Crawler1<T> {
    field1: T,
}


struct Crawler2 {
    field1: usize,
}