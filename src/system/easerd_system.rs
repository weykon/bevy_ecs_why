use super::system::System;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

trait ErasedSystem {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>);
}

impl<S: System> ErasedSystem for S {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) {
        <Self as System>::run(self, resources);
    }
}
