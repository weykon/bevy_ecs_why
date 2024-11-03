use std::{
    any::{Any, TypeId},
    collections::HashMap,
};
use super::system::System;

trait ErasedSystem {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>);
}

impl<S: System<I>, I> ErasedSystem for S {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) {
        <Self as System<I>>::run(self, resources);
    }
}
