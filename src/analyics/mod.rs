use crate::system::system::System;

struct Function {
    f: fn(),
}
impl System for Function {
    fn run(&mut self, resources: &mut std::collections::HashMap<std::any::TypeId, Box<dyn std::any::Any>>) {
        todo!()
    }
}