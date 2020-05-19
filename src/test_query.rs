use crate::query::EntityQuery;
use std::any::TypeId;

#[test]
fn test_query() {
    struct Position;
    let query = EntityQuery {
        all: vec![TypeId::of::<Position>()],
        none: vec![],
        any: vec![]
    };
}