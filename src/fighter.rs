use crate::*;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Fighter {
    pub test: i32,
}

impl Default for Fighter {
    fn default() -> Self {
        Self { test: 2 }
    }
}

// impl Fighter {
//     pub fn new() -> Self {
//         return Self {
//             test: Default::default(),
//         };
//     }
// }
