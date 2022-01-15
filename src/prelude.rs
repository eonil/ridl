pub type PVec<T> = im_rc::vector::Vector<T>;
pub type PString = std::rc::Rc<String>;

// #[extend::ext(name=PVecDisplay)]
// impl<T> std::fmt::Display for PVec<T> {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         for x in self.iter() {
//             write!(f, "{}", x)?;
//         }
//         Ok(())
//     }
// }