use std::collections::{LinkedList, HashMap};
use crate::entity::Entity;
use crate::cmd::{CmdChain, CmdCreateEntity};
use crate::archetype::{Archetype, ArchetypeManager, DEFAULT_ARCHETYPE};
use crate::component::Component;
use std::any::TypeId;

/// top level unit of isolation
pub struct Universe {
    pub(crate) free_entity_indices: LinkedList<u64>,
    pub(crate) entity_versions: HashMap<u64, u64>,
    pub archetype_manager: ArchetypeManager
}

impl Universe {
    pub fn new() -> Universe {
        let mut free_entity_indices: LinkedList<u64> = LinkedList::new();
        let mut entity_versions: HashMap<u64, u64> = HashMap::new();
        for i in 1..10000/*2u64^64-1*/ {
            free_entity_indices.push_back(i);
            entity_versions.insert(i, 1);
        }
        Universe {
            free_entity_indices,
            entity_versions,
            archetype_manager: ArchetypeManager::default()
        }
    }

    pub fn is_valid(&self, entity: Entity) -> bool {
        return entity.id > 0 && entity.version == *self.entity_versions.get(&entity.id).unwrap_or(&0u64);
    }

    /// execute cmd chain
    /// mutates chain so state is retained and available for reading afterwards
    pub fn exec(&mut self, cmd_chain: &mut CmdChain) {
        loop {
            if cmd_chain.cmds.is_empty() {
                break;
            }

            let cmd = cmd_chain.cmds.pop_front().unwrap();
            cmd.exec(self, &mut cmd_chain.state)
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let mut cmds = CmdChain::new();
        cmds.create_entity();
        self.exec(&mut cmds);
        return cmds.state.last_created_entity.unwrap();
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        let mut cmds = CmdChain::new();
        cmds.destroy_entity(entity);
        self.exec(&mut cmds);
    }

    pub fn add_component<T: Component + 'static>(&mut self, entity: Entity, component: T) {
        if !self.is_valid(entity) {
            panic!("ecs: failed to add component to invalid entity {}", entity);
        }
        let component_type_id = TypeId::of::<T>();
        let entity_archetype = self.archetype_manager.get_archetype_id(entity);
        if entity_archetype == DEFAULT_ARCHETYPE {
            let existing_archetype = self.archetype_manager.find_archetype(vec![component_type_id]);
            if existing_archetype.is_some() {

            } else {
                // fresh new archetype
                let archetype = Archetype { component_types: vec![component_type_id] };
                let archetype_id = self.archetype_manager.register_archetype(archetype);
                self.archetype_manager.set_entity_archetype(entity, archetype_id);
            }
        }
       /* if entity_archetype.is_some() {
            let archetype = self.archetype_manager.get_archetype(entity_archetype).unwrap();
            if archetype.component_types.contains(&component_type_id) {
                panic!("ecs: component already exists on entity {}", entity);
            }
        } else {

        }*/
    }

    pub fn has_component<T: Component + 'static>(&mut self, entity: Entity) -> bool {
        if !self.is_valid(entity) {
            panic!("ecs: failed to check has component for invalid entity {}", entity);
        }
        let component_type_id = TypeId::of::<T>();
        let entity_archetype_id = self.archetype_manager.get_archetype_id(entity);
        if entity_archetype_id == DEFAULT_ARCHETYPE {
            return false;
        }
        return self.archetype_manager.get_archetype(entity_archetype_id).unwrap()
            .component_types.contains(&component_type_id);
    }

    /*pub fn get_component<T: Component + 'static>(&mut self, entity: Entity) -> T {
        if !self.is_valid(entity) {
            panic!("ecs: failed to get component {:?} for invalid entity {}", T, entity);
        }

    }*/
}