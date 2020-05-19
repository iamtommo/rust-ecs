use std::any::TypeId;
use std::collections::{HashSet, HashMap, LinkedList};
use crate::entity::Entity;

/// unique identifies an archetype
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ArchetypeId {
    pub index: usize
}

/// uniquely identifies a set of components
pub struct Archetype {
    pub component_types: Vec<TypeId>,
    pub component_sizes: Vec<usize>
}

/// default archetype for empty entities
/// makes more sense than checking for archetype existence on every entity operation
pub const DEFAULT_ARCHETYPE: ArchetypeId = ArchetypeId { index: 0 };

/// linear component layout
pub(crate) struct ArchetypeStorage {
    pub(crate) archetype_id: ArchetypeId,
    pub(crate) archetype_size: usize,
    pub(crate) free_indices: LinkedList<usize>,
    pub(crate) entity_indices: HashMap<Entity, usize>,
    pub(crate) data: Vec<u8>
}

impl ArchetypeStorage {
    pub(crate) fn create(archetype_id: ArchetypeId, archetype_size: usize) -> ArchetypeStorage {
        let data: Vec<u8> = Vec::new();
        let mut free_indices: LinkedList<usize> = LinkedList::new();
        for i in 0..10000 {
            free_indices.push_back(i);
        }

        ArchetypeStorage {
            archetype_id,
            archetype_size,
            free_indices,
            entity_indices: HashMap::with_capacity(10000),
            data
        }
    }
}

pub struct ArchetypeManager {
    pub(crate) entity_archetypes: HashMap<Entity, ArchetypeId>,
    pub(crate) archetypes: Vec<Archetype>,
    pub(crate) archetype_index_seq: usize,
    pub(crate) storage: HashMap<ArchetypeId, ArchetypeStorage>
}

impl Default for ArchetypeManager {
    fn default() -> Self {
        ArchetypeManager {
            entity_archetypes: HashMap::new(),
            archetypes: vec![Archetype {
                component_types: vec![TypeId::of::<i32>()],
                component_sizes: vec![std::mem::size_of::<i32>()] }],
            archetype_index_seq: 1, // begin at 1 after DEFAULT_ARCHETYPE,
            storage: HashMap::new()
        }
    }
}
impl ArchetypeManager {
    pub fn get_archetype_id(&self, entity: Entity) -> ArchetypeId {
        let arch = self.entity_archetypes.get(&entity);
        if arch.is_none() {
            return DEFAULT_ARCHETYPE;
        }
        return *arch.unwrap();
    }

    pub fn get_archetype(&self, archetype_id: ArchetypeId) -> Option<&Archetype> {
        return self.archetypes.get(archetype_id.index);
    }

    pub fn get_archetype_for_entity(&self, entity: Entity) -> Option<&Archetype> {
        let archetype_id = self.get_archetype_id(entity);
        return self.get_archetype(archetype_id);
    }

    pub fn find_archetype(&self, components: Vec<TypeId>) -> Option<&Archetype> {
        return self.archetypes.iter().find(|arch| {
            if arch.component_types.len() != components.len() {
                return false;
            }
            return components.iter().all(|c| arch.component_types.contains(c));
        });
    }

    /// register a new archetype, returns the unique archetype id
    pub fn register_archetype(&mut self, archetype: Archetype) -> ArchetypeId {
        // create storage
        let archetype_id = ArchetypeId { index: self.archetype_index_seq };
        let archetype_size = (&archetype).component_sizes.iter().sum();
        self.storage.insert(archetype_id, ArchetypeStorage::create(archetype_id, archetype_size));
        // store & increment index counter
        self.archetypes.push(archetype);
        self.archetype_index_seq += 1;
        return archetype_id;
    }

    pub fn set_entity_archetype(&mut self, entity: Entity, archetype_id: ArchetypeId) {
        self.entity_archetypes.insert(entity, archetype_id);
    }
}