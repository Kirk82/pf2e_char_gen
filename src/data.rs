use crate::*;

pub struct Data {
    pub male_first_names: Vec<String>,
    pub female_first_names: Vec<String>,
    pub last_names: Vec<String>,
    pub heritage_map: HashMap<Ancestory, Vec<Heritage>>,
}

impl Data {
    pub fn new() -> Self {
        // For heritages, we create a HashMap that has a list (vec) of valid heritages for each race
        // We declare the heritage_map variable, and initialise it to an empty HashMap.
        // Variable is 'mut' because we need to modify/mutate it when we add the key-value pairs
        let mut heritage_map = HashMap::new();

        // Inserting a key: Human, and the corresponding value: vector of heritages that humans can have
        heritage_map.insert(
            Ancestory::Human,
            vec![
                Heritage::SkilledHuman,
                Heritage::VersatileHuman,
                Heritage::WintertouchedHuman,
            ],
        );
        // Inserting a key: Elf, and the corresponding value: vector of heritages that elf can have
        heritage_map.insert(
            Ancestory::Elf,
            vec![
                Heritage::AncientElf,
                Heritage::ArcticElf,
                Heritage::WoodlandElf,
            ],
        );
        // Inserting a key: Dwarf, and the corresponding value: vector of heritages that dwarf can have
        heritage_map.insert(
            Ancestory::Dwarf,
            vec![
                Heritage::RockDwarf,
                Heritage::OathkeeperDwarf,
                Heritage::ForgeDwarf,
            ],
        );

        return Self {
            female_first_names: vec![
                "Dimitra".to_string(),
                "Millicent".to_string(),
                "Sarah".to_string(),
                "Jane".to_string(),
            ],
            last_names: vec![
                "Daly".to_string(),
                "Hamilton".to_string(),
                "Baxter".to_string(),
                "Denris".to_string(),
                "Papavassiliou".to_string(),
            ],
            male_first_names: vec![
                "Kirk".to_string(),
                "Liam".to_string(),
                "Ross".to_string(),
                "Simon".to_string(),
                "Alek".to_string(),
            ],
            heritage_map: heritage_map,
        };
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum SavingThrows {
    Fortitude,
    Reflex,
    Will,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Skills {
    Acrobatics,
    Arcana,
    Athletics,
    Crafting,
    Deception,
    Diplomacy,
    Intimidation,
    Medicine,
    Nature,
    Occultism,
    Performance,
    Religion,
    Society,
    Stealth,
    Survival,
    Thievery,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Ancestory {
    Elf,
    Dwarf,
    Human,
}

#[derive(Debug)]
pub enum Heritage {
    AncientElf,
    ArcticElf,
    WoodlandElf,
    RockDwarf,
    OathkeeperDwarf,
    ForgeDwarf,
    VersatileHuman,
    WintertouchedHuman,
    SkilledHuman,
}

pub enum Archetype {
    Magus,
    Sorcerer,
    Fighter,
}
