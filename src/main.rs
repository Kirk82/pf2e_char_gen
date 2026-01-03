mod character;
mod data;
mod magus;
mod sorcerer;

use character::*;
use data::*;
use magus::*;
use rng::*;
use sorcerer::*;

pub use std::collections::HashMap;

fn main() {
    let data = Data::new();

    //randomly selecting Ancestory
    // let chosen_ancestory = Ancestory::random_variant(rng);

    // //Using a bool to randomly select male or female sex

    // let sex_decider = rng.gen_bool(0.5);

    // let mut chosen_sex: String = "Male".to_owned();

    // if sex_decider == true {
    //     chosen_sex = "Male".to_owned()
    // } else if sex_decider == false {
    //     chosen_sex = "Female".to_owned()
    // };

    // //Randomly choosing a first name from data.rs based on sex
    // let mut chosen_first_name = data.male_first_names.sample(rng).unwrap();

    // if sex_decider == true {
    //     chosen_first_name = data.male_first_names.sample(rng).unwrap();
    // } else if sex_decider == false {
    //     chosen_first_name = data.female_first_names.sample(rng).unwrap();
    // };

    // //Randomly choosing a last name from data.rs
    // let chosen_last_name = data.last_names.sample(rng).unwrap();

    // // Randomly selecting an from defined ranged based on Ancestory
    // let chosen_age = match chosen_ancestory {
    //     Ancestory::Elf => rng.gen_range(19..700),
    //     Ancestory::Dwarf => rng.gen_range(19..400),
    //     Ancestory::Human => rng.gen_range(19..100),
    // };

    // //Randomly selecting a heritage from a hashmap in data.rs based on Ancestory
    // let potential_heritage: &Vec<Heritage> = data.heritage_map.get(&chosen_ancestory).unwrap();

    // let chosen_heritage: &Heritage = potential_heritage.sample(rng).unwrap();

    let mut character = Character::new();

    character.choose_gender();
    character.choose_name(&data);
    character.choose_last_name(&data);
    character.choose_ancestory();
    character.choose_age();
    character.choose_heritage(&data);

    //Printing all the data store in Character Struct
    // let character = Character {
    //     ancestory: chosen_ancestory,
    //     first_name: chosen_first_name,
    //     last_name: chosen_last_name,
    //     heritage: chosen_heritage,
    //     gender: choose_gender(),
    //     age: chosen_age,
    // };

    println!("{:?}", character)
}
