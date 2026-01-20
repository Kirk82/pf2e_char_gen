use crate::*;

#[derive(Debug, Default, PartialEq, Eq, Hash)]
pub struct Character {
    pub first_name: String,
    pub last_name: String,
    pub ancestory: Ancestory,
    pub gender: Gender,
    pub age: i32,
    pub heritage: Heritage,
    // pub perception: u32,
    // pub saving_throws_map: HashMap<SavingThrows, u32>,
    // pub skills_map: HashMap<Skills, u32>,
    // pub attacks: Vec<String>,
    // pub defences: Vec<String>,
    // pub spells: Vec<String>,
    // pub speed: u32,
    pub archetype: Archetype,
    // pub background: Vec<String>,
    // pub general_feats: Vec<String>,
    // pub skill_feats: Vec<String>,
}

impl Character {
    pub fn new() -> Self {
        Default::default()
    }
    // fn for choosing gender using a bool to match with gender enum
    pub fn choose_gender(&mut self) {
        let rng = Rng::new();

        let genders = match rng.gen_bool(0.5) {
            true => Gender::Male,
            false => Gender::Female,
        };
        self.gender = genders
    }
    // fn for choosing name which samples a list from data.rs based on which gender was chosen
    pub fn choose_name(&mut self, data: &Data) {
        let rng = Rng::new();

        let chosen_gender = self.gender;

        let first_names;

        if chosen_gender == Gender::Male {
            first_names = data.male_first_names.sample(rng).unwrap();
        } else {
            first_names = data.female_first_names.sample(rng).unwrap();
        }

        self.first_name = first_names.to_string()
    }

    pub fn choose_last_name(&mut self, data: &Data) {
        let rng = Rng::new();

        let chosen_last_name = data.last_names.sample(rng).unwrap();

        self.last_name = chosen_last_name.to_string()
    }

    pub fn choose_ancestory(&mut self) {
        let rng = Rng::new();

        let chosen_ancestory = Ancestory::random_variant(rng);

        self.ancestory = chosen_ancestory
    }

    pub fn choose_age(&mut self) {
        let rng = Rng::new();

        let chosen_ancestory = &self.ancestory;

        let chosen_age = match chosen_ancestory {
            Ancestory::Elf => rng.gen_range(19..500),
            Ancestory::Dwarf => rng.gen_range(19..250),
            Ancestory::Human => rng.gen_range(19..70),
        };

        self.age = chosen_age
    }

    pub fn choose_heritage(&mut self, data: &Data) {
        let rng = Rng::new();

        let chosen_ancestory = &self.ancestory;

        let potential_heritage = data.heritage_map.get(&chosen_ancestory).unwrap();

        let chosen_heritage = potential_heritage.sample(rng).unwrap();

        let borrowed_heritage = *chosen_heritage;

        self.heritage = borrowed_heritage
    }

    pub fn choose_archetype(&mut self) {
        let rng = Rng::new();

        let random_number = rng.gen_range(0..2);

        let archetype = match random_number {
            0 => Archetype::Magus(magus),
            1 => Archetype::Sorcerer(sorcerer),
            2 => Archetype::Fighter(fighter),
        };

        self.archetype = archetype;
    }
}
