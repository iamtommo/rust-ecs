use crate::query::EntityQuery;
use std::any::TypeId;
use crate::component::Component;
use crate::universe::Universe;

#[test]
fn test_query() {
    struct Position { pos: i32 }
    impl Component for Position {}

    let mut u = Universe::new();
    let entity = u.create_entity();
    u.add_component_data(entity, Position { pos: 1337 });

    let query = EntityQuery {
        all: vec![TypeId::of::<Position>()],
        none: vec![],
        any: vec![]
    };
    let data = u.get_entities(query);
}