use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use super::function_system::FunctionSystem;

// 从理解上，单体的系统接受关于一切还未知的输入，
pub trait System {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>);
}

// 这个实现意味着任何可以作为 FnMut() 调用的闭包
// 都可以被视为一个 System<()>。
impl<F: FnMut()> System for FunctionSystem<(), F> {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) {
        (self.f)()
    }
}
impl<F: FnMut(T1), T1: 'static> System for FunctionSystem<(T1,), F> {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) {
        let _0 = *resources
            .remove(&TypeId::of::<T1>())
            .unwrap()
            .downcast::<T1>()
            .unwrap();

        (self.f)(_0)
    }
}
impl<F: FnMut(T1, T2), T1: 'static, T2: 'static> System for FunctionSystem<(T1, T2), F> {
    fn run(&mut self, resources: &mut HashMap<TypeId, Box<dyn Any>>) {
        let _0 = *resources
            .remove(&TypeId::of::<T1>())
            .unwrap()
            .downcast::<T1>()
            .unwrap();
        let _1 = *resources
            .remove(&TypeId::of::<T2>())
            .unwrap()
            .downcast::<T2>()
            .unwrap();

        (self.f)(_0, _1)
    }
}
