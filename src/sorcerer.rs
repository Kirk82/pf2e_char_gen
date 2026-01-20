use crate::*;

#[derive(Debug)]
pub struct Sorcerer {
    pub saving_throws_map: HashMap<SavingThrows, u32>,
}
