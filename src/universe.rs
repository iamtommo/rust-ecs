use std::any::TypeId;
use std::collections::{HashMap, LinkedList};
use std::mem;

use crate::archetype::{Archetype, ArchetypeManager, ArchetypeStorage, DEFAULT_ARCHETYPE};
use crate::cmd::{CmdChain, CmdCreateEntity};
use crate::component::Component;
use crate::entity::Entity;

/// top level unit of isolation
pub struct Universe {
    pub(crate) free_entity_indices: LinkedList<u64>,
    pub(crate) entity_versions: HashMap<u64, u64>,
    pub(crate) archetype_manager: ArchetypeManager
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

    pub fn add_component<T: Component + 'static>(&mut self, entity: Entity) {
        if !self.is_valid(entity) {
            panic!("ecs: add_component failed: invalid entity {}", entity);
        }
        let component_type_id = TypeId::of::<T>();
        let entity_archetype = self.archetype_manager.get_archetype_id(entity);
        if entity_archetype == DEFAULT_ARCHETYPE {
            let existing_archetype = self.archetype_manager.find_archetype(vec![component_type_id]);
            if existing_archetype.is_some() {

            } else {
                // fresh new archetype
                let archetype = Archetype {
                    component_types: vec![component_type_id],
                    component_sizes: vec![mem::size_of::<T>()]
                };
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

    pub fn add_component_data<T: Component + 'static>(&mut self, entity: Entity, component: T) {
        self.add_component::<T>(entity);
        self.set_component::<T>(entity, component);
    }

    pub fn has_component<T: Component + 'static>(&mut self, entity: Entity) -> bool {
        if !self.is_valid(entity) {
            panic!("ecs: has_component failed: invalid entity {}", entity);
        }
        let component_type_id = TypeId::of::<T>();
        let entity_archetype_id = self.archetype_manager.get_archetype_id(entity);
        if entity_archetype_id == DEFAULT_ARCHETYPE {
            return false;
        }
        return self.archetype_manager.get_archetype(entity_archetype_id).unwrap()
            .component_types.contains(&component_type_id);
    }

    pub fn set_component<T: Component + 'static>(&mut self, entity: Entity, component: T) {
        if !self.is_valid(entity) {
            panic!("ecs: set_component failed: invalid entity {}", entity);
        }
        let archetype_id = self.archetype_manager.get_archetype_id(entity);
        let archetype = self.archetype_manager.get_archetype(archetype_id).unwrap();
        let component_type_id = TypeId::of::<T>();
        if !archetype.component_types.contains(&component_type_id) {
            panic!("ecs: set_component failed: entity has no such component");
        }
        let component_type_index = archetype.component_types.iter()
            .position(|c| *c == component_type_id).unwrap();
        let mut archetype_storage: &mut ArchetypeStorage = self.archetype_manager.storage
            .get_mut(&archetype_id).unwrap();

        let data_ptr: *mut T = compute_ptr_to_component_data(entity, component_type_index,
                                                                   archetype, &mut archetype_storage) as *mut T;
        unsafe {
            std::ptr::write_unaligned::<T>(data_ptr, component);
        }
    }

    pub fn get_component<T: Component + 'static>(&mut self, entity: Entity) -> T {
        if !self.is_valid(entity) {
            panic!("ecs: get_component failed: invalid entity {}", entity);
        }
        let archetype_id = (&mut self.archetype_manager).get_archetype_id(entity);
        let archetype = (&mut self.archetype_manager).get_archetype(archetype_id).unwrap();
        let component_type_id = TypeId::of::<T>();
        if !archetype.component_types.contains(&component_type_id) {
            panic!("ecs: get_component failed: entity has no such component");
        }
        let component_type_index = archetype.component_types.iter()
            .position(|c| *c == component_type_id).unwrap();
        let mut archetype_storage: &mut ArchetypeStorage = (&mut self.archetype_manager).storage.get_mut(&archetype_id).unwrap();

        let data_ptr: *const u8 = compute_ptr_to_component_data(entity, component_type_index,
                                                          archetype, &mut archetype_storage) as *const u8;
        let component: T = unsafe { std::ptr::read_unaligned::<T>(data_ptr as *const _) };
        return component;
    }

}

fn compute_ptr_to_component_data(entity: Entity, component_type_index: usize,
                                     archetype: &Archetype, storage: &mut ArchetypeStorage) -> *mut u8 {
    let archetype_total_size: usize = archetype.component_sizes.iter().sum();
    // compute base pointer into archetype storage data
    let entity_data_index: usize = storage.entity_indices[&entity];
    // compute offset into archetype component data
    let mut component_data_offset: usize = 0;
    for i in 0..component_type_index as usize {
        component_data_offset += archetype.component_sizes[i]
    }
    let data_offset: isize = ((entity_data_index * archetype_total_size) + component_data_offset) as isize;
    unsafe {
        return storage.data.as_mut_ptr().offset(data_offset) as *mut u8;
    }
}