mod character;
mod data;

use character::*;
use data::*;
use rng::*;

pub use std::collections::HashMap;

fn main() {
    let rng = Rng::new();

    let race_id = rng.gen_range(0..3);

    let ancestory: Ancestory = match race_id {
        0 => Ancestory::Elf,
        1 => Ancestory::Human,
        2 => Ancestory::Dwarf,
        _ => panic!(),
    };

    let character = Character {
        ancestory: ancestory,
    };

    println!("{:?}", character)
}
