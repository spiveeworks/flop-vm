use prelude::*;

use sulphate_lib::event_queue;

use algorithm;
use data;
use event;
use item;

pub struct FlopInstance {
    pub totem: Totem,
    pub event_queue: event::EventQueue,
    pub types: Dict<item::EntityType>,
}

impl FlopInstance {
    pub fn invoke_next(self: &mut Self) {
        event_queue::Simulation::invoke_next(self);
    }

    pub fn run<G: Flop>(
        game: &mut G,

        root_type: String,
        root_table: String,
        init: String,
    ) {
        let _root = algorithm::execute_init(
            game,

            root_type,
            root_table,
            init,
            Vec::new(),
        );

        while !game.event_queue().is_empty() {
            event_queue::Simulation::invoke_next(game);
        }
        println!("Nothing happened.");
    }
}

pub trait Flop: AsMut<FlopInstance> + AsMut<event::EventQueue> {
    fn extern_call(
        self: &mut Self,
        func_name: &String,
        args: Vec<data::Field>,
    ) -> Vec<data::Field>;
}

impl AsMut<FlopInstance> for FlopInstance {
    fn as_mut(self: &mut Self) -> &mut Self {
        self
    }
}

// purely for the Simulate trait, do not use
impl AsMut<event::EventQueue> for FlopInstance {
    fn as_mut(self: &mut Self) -> &mut event::EventQueue {
        &mut self.event_queue
    }
}


impl Flop for FlopInstance {
    fn extern_call(
        self: &mut Self,
        _func_name: &String,
        _args: Vec<data::Field>,
    ) -> Vec<data::Field> {
        panic!("Tried to call rust code from default Flop environment");
    }
}

pub(crate) trait FlopParts {
    fn parts(self: &mut Self) -> (
        &mut Totem,
        &mut Dict<item::EntityType>,
        &mut event::EventQueue,
    );
    fn totem(self: &mut Self) -> &mut Totem;
    fn types(self: &mut Self) -> &mut Dict<item::EntityType>;
    fn event_queue(self: &mut Self) -> &mut event::EventQueue;
}

impl<G: Flop> FlopParts for G {
    fn parts(self: &mut Self) -> (
        &mut Totem,
        &mut Dict<item::EntityType>,
        &mut event::EventQueue,
    ) {
        let instance: &mut FlopInstance = self.as_mut();
        (&mut instance.totem, &mut instance.types, &mut instance.event_queue)
    }

    fn totem(self: &mut Self) -> &mut Totem {
        self.parts().0
    }
    fn types(self: &mut Self) -> &mut Dict<item::EntityType> {
        self.parts().1
    }
    fn event_queue(self: &mut Self) -> &mut event::EventQueue {
        self.parts().2
    }
}