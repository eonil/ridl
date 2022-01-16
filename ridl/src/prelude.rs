#![allow(unused)]

use linear_map::LinearMap;
pub use extend::ext;

pub type List<T> = Vec<T>;
pub type Map<K,V> = LinearMap<K,V>;
pub type PVec<T> = im_rc::vector::Vector<T>;
pub type PString = std::rc::Rc<String>;
