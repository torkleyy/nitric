#![allow(unused)]

pub trait System {
    fn setup(&mut self);

    fn on_start(&mut self);

    // * build `MergeSet` and merge it later
    // * could go from existing resources / clone them
    fn run(&mut self);

    fn on_stop(&mut self);
}

pub struct SystemGraph {

}
