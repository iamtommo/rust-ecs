use std::any::TypeId;

pub struct EntityQuery {
    pub all: Vec<TypeId>,
    pub none: Vec<TypeId>,
    pub any: Vec<TypeId>
}