// end to end tests

use crate::universe::Universe;
use crate::component::Component;

struct Position {
    value: f32
}
impl Component for Position {}

#[test]
fn test_add_get_component() {
    let mut uni = Universe::new();
    let entity = uni.create_entity();
    let position = Position { value: 1337f32 };
    uni.add_component(entity, position);
    //let position2 = uni.get_component()
}