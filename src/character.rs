use crate::*;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Character<'a> {
    pub ancestory: Ancestory,
    pub first_name: &'a String,
    pub last_name: &'a String,
    pub gender: Gender,
    // pub perception: u32,
    // pub saving_throws_map: HashMap<SavingThrows, u32>,
    // pub skills_map: HashMap<Skills, u32>,
    // pub attacks: Vec<String>,
    // pub defences: Vec<String>,
    // pub spells: Vec<String>,
    // pub speed: u32,
    pub age: i32,
    // pub archetype: Archetype,
    // pub background: Vec<String>,
    pub heritage: &'a Heritage,
    // pub general_feats: Vec<String>,
    // pub skill_feats: Vec<String>,
}

impl Character<'_> {
    pub fn new() -> Self {
        pub fn choose_gender(character: &mut Character) {
            let rng = Rng::new();

            let gender = match rng.gen_bool(0.5) {
                true => Gender::Male,
                false => Gender::Female,
            };
            character.gender = gender
        }

        return Self {
            gender: choose_gender(&mut Character),
            ancestory: todo!(),
            first_name: todo!(),
            last_name: todo!(),
            age: todo!(),
            heritage: todo!(),
        };
    }
}
