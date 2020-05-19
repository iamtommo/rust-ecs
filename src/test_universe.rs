use crate::universe::Universe;
use crate::component::Component;
use crate::entity::Entity;

struct TestComponent {}
impl Component for TestComponent {}

#[test]
fn create_entity() {
    let entity = Universe::new().create_entity();
    assert_eq!(entity.id, 1);
    assert_eq!(entity.version, 1);
}

#[test]
fn create_destroy_reuses_index_increments_version() {
    let mut u = Universe::new();
    let entity1 = u.create_entity();
    assert_eq!(u.is_valid(entity1), true);

    u.destroy_entity(entity1);
    let entity2 = u.create_entity();
    assert_eq!(entity2.id, entity1.id);
    assert_eq!(entity2.version, entity1.version + 1)
}

#[test]
#[should_panic]
fn add_component_to_invalid_entity() {
    let mut u = Universe::new();
    u.add_component(Entity { id: 0, version: 0}, TestComponent {});
}

#[test]
fn add_component_has_component() {
    let mut u = Universe::new();
    let entity = u.create_entity();
    assert_eq!(false, u.has_component::<TestComponent>(entity));
    u.add_component(entity, TestComponent {});
    assert_eq!(true, u.has_component::<TestComponent>(entity));
}