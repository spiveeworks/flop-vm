extern crate civil_vm;

use std::fs;

use civil_vm::prelude::*;

use civil_vm::data::EntityData;
use civil_vm::data::EntityType;
use civil_vm::game::Game;
use civil_vm::parser::TypeParser;
use civil_vm::time::EventQueue;

fn get_types() -> Dict<EntityType> {
    let parser = TypeParser::new();
    let paths = fs::read_dir("./types")
        .expect("Failed to open \"types\" directory");

    let mut types = Dict::new();
    for path in paths {
        use std::io::Read;

        let path = path.expect("IO error getting paths").path();
        let path_str = path.to_str().expect("invalid path name");
        let file_name = path.file_stem().unwrap().to_str().unwrap().into();
        let mut file = fs::File::open(path_str).expect("Failed to open file");
        let mut content = String::new();
        file.read_to_string(&mut content).expect("Failed to read file");

        let parsed = parser.parse(&content).unwrap();
        types.insert(file_name, parsed);
    }

    types
}

fn main() {
    let now = Time::try_from(0.0).unwrap();
    // only do this once ok?
    let totem = unsafe { Totem::new() };
    let event_queue = EventQueue::new(now);
    let types = get_types();

    let root = EntityData::new("Root".into());

    let mut game = Game { totem, event_queue, types, root };

    game.run();
}
