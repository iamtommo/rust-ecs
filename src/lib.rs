mod component;
mod universe;
mod entity;
mod cmd;
mod test;

pub struct ComponentTypeId {

}

pub struct ComponentMeta {

}

/// Unique archetype id
pub struct ArchetypeId {
    pub world_id: i32,
    pub archetype_index: i32
}

pub struct ArchetypeDescription {
    pub components: Vec<(ComponentTypeId, ComponentMeta)>
}

#[cfg(test)]
mod tests {
    use crate::{ComponentMeta, ComponentTypeId};
    use std::any::Any;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_type_ids() {
       /* let thing1 = (ComponentTypeId, ComponentMeta).type_id();
        println!("type id? {:?}", thing1);
*/
        let testerino = |thing1: i32, thing2: i32| {

        };
    }
}
