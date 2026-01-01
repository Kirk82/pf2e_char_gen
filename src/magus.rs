use crate::*;

pub struct Magus {
    pub saving_throws_map: HashMap<SavingThrows, u32>,
    pub attacks: Vec<String>,
    pub defences: Vec<String>,
    pub spells: Vec<String>,
    pub cantrips: Vec<String>,
    pub perception: u32,
    pub spell_dc: u32,
    pub subclass: HybridStudy,
}

impl Magus {
    pub fn new() -> Self {
        let mut saving_throws_map = HashMap::new();

        saving_throws_map.insert(SavingThrows::Fortitude, 5);
        saving_throws_map.insert(SavingThrows::Reflex, 3);
        saving_throws_map.insert(SavingThrows::Will, 5);

        let perception = 3;

        let spell_dc = 13;

        let rng = Rng::new();

        let subclass = HybridStudy::random_variant(rng);

        return Self {
            perception: perception,
            saving_throws_map: saving_throws_map,
            attacks: vec![
                "Simple Weapons".to_string(),
                "Martial Weapons".to_string(),
                "Unarmed".to_string(),
            ],
            defences: vec![
                "light armour".to_string(),
                "medium armour".to_string(),
                "unarmoured".to_string(),
            ],
            spells: vec![
                "Create Water".to_string(),
                "Fear".to_string(),
                "Gust of Wind".to_string(),
            ],
            cantrips: vec![
                "Bull Horn".to_string(),
                "Detect Magic".to_string(),
                "Electric Arc".to_string(),
            ],

            spell_dc: spell_dc,

            subclass,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Hash, RandomVariant)]
pub enum HybridStudy {
    LaughingShadow,
    SparklingTarge,
    TwistingTree,
}
