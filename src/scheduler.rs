use std::{
    any::{Any, TypeId},
    collections::HashMap,
};
use crate::system::stored_system::StoredSystem;

struct Scheduler {
    systems: Vec<StoredSystem>,
    resources: HashMap<TypeId, Box<dyn Any>>,
}
