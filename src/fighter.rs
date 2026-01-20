use crate::*;

#[derive(Debug)]
pub struct Fighter {
    pub test: i32,
}

impl Fighter {
    pub fn new() -> Self {
        return Self {
            test: Default::default(),
        };
    }
}
