use crate::*;

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct Character<'a> {
    pub ancestory: Ancestory,
    pub first_name: &'a str,
    pub last_name: &'a str,
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
    pub heritage: Heritage,
    // pub general_feats: Vec<String>,
    // pub skill_feats: Vec<String>,
}

impl Character<'_> {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn choose_gender(&mut self) {
        let rng = Rng::new();

        let genders = match rng.gen_bool(0.5) {
            true => Gender::Male,
            false => Gender::Female,
        };
        self.gender = genders
    }

    pub fn choose_name(&mut self) {
    let rng = Rng::new();
    let data = Data::new();

    let chosen_gender = self.gender;

    let first_names;

    if chosen_gender == Gender::Male {
        let mut first_names = data.male_first_names.random_variant(rng);
    } else if chosen_gender == Gender::Female {
        let mut first_names = data.FemaleFirstNames::random_variant(rng);
    };

    self.first_name = first_names
}
}
