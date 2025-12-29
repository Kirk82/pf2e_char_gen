use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Character {
    // pub perception: u32,
    // pub saving_throws_map: HashMap<SavingThrows, u32>,
    // pub skills_map: HashMap<Skills, u32>,
    // pub attacks: Vec<String>,
    // pub defences: Vec<String>,
    // pub spells: Vec<String>,
    // pub speed: u32,
    // pub male_first_name: Vec<String>,
    // pub female_first_name: Vec<String>,
    // pub last_name: Vec<String>,
    // pub archetype: Archetype,
    pub ancestory: data::Ancestory,
    // pub background: Vec<String>,
    // pub heritage: HashMap<String, Vec<String>>,
    // pub general_feats: Vec<String>,
    // pub skill_feats: Vec<String>,
}

pub enum Archetype {
    Magus,
    Sorcerer,
    Fighter,
}
