// pub fn add(left: usize, right: usize) -> usize {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }

pub mod core;
pub mod data_storing;
pub mod input_handling;
pub mod text_rendering;
pub mod ui;

use std::error::Error;
pub type DynResult<T = (), E = Box<dyn Error>> = Result<T, E>;
