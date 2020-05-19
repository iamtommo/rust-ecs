pub mod component;
pub mod universe;
pub mod entity;
pub mod cmd;
pub mod archetype;
pub mod storage;

#[cfg(test)]
mod test_full;
#[cfg(test)]
mod test_archetype;
#[cfg(test)]
mod test_universe;

/*
pub struct ComponentMeta {

}
*/

/*pub struct ArchetypeId {
    pub world_id: i32,
    pub archetype_index: i32
}*/

/*pub struct ArchetypeDescription {
    pub components: Vec<(ComponentTypeId, ComponentMeta)>
}*/