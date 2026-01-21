use crate::*;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct Fighter {
    pub test: i32,
}

impl Fighter {
    pub fn new() -> Self {
        let test_1 = 2;

        return Self { test: test_1 };
    }
}

// impl Fighter {
//     pub fn new() -> Self {
//         return Self {
//             test: Default::default(),
//         };
//     }
// }
