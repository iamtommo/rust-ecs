use crate::universe::Universe;
use crate::system::System;
use std::cell::Cell;

// test create/get/has systems
#[derive(Default)]
struct TestSystem {
    pub val: i32
}
impl System for TestSystem {
    fn create(&mut self, universe: &mut Universe) {}
    fn update(&mut self, universe: &mut Universe) {}
    fn destroy(&mut self, universe: &mut Universe) {}
}
#[test]
fn test_systems() {
    let mut u = Universe::new();
    u.create_system::<TestSystem>();
    assert!(u.has_system::<TestSystem>());
    assert_eq!(u.get_system::<TestSystem>().val, i32::default());
}


// test system mutation
#[derive(Default)]
struct TestSystem2 {
    pub val: Cell<i32>
}
impl System for TestSystem2 {
    fn create(&mut self, universe: &mut Universe) {}
    fn update(&mut self, universe: &mut Universe) {}
    fn destroy(&mut self, universe: &mut Universe) {}
}
#[test]
fn test_systems_mutation() {
    let mut u = Universe::new();
    let mut sys = u.create_system::<TestSystem2>();
    sys.val.set(69);
    assert_eq!(u.get_system::<TestSystem2>().val.get(), 69);
}