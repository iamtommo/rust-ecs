use std::any::TypeId;
use std::collections::{HashSet, HashMap};
use crate::entity::Entity;

/// unique identifies an archetype
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ArchetypeId {
    pub index: usize
}

/// uniquely identifies a set of components
pub struct Archetype {
    pub component_types: Vec<TypeId>
}

/// default archetype for empty entities
/// makes more sense than checking for archetype existence on every entity operation
pub const DEFAULT_ARCHETYPE: ArchetypeId = ArchetypeId { index: 0 };

pub struct ArchetypeManager {
    pub(crate) entity_archetypes: HashMap<Entity, ArchetypeId>,
    pub(crate) archetypes: Vec<Archetype>,
    pub(crate) archetype_index_seq: usize
}

impl Default for ArchetypeManager {
    fn default() -> Self {
        ArchetypeManager {
            entity_archetypes: HashMap::new(),
            archetypes: vec![Archetype { component_types: vec![TypeId::of::<i32>()]}],
            archetype_index_seq: 1 // begin at 1 after DEFAULT_ARCHETYPE
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
        self.archetypes.push(archetype);
        let new_archetype_id = ArchetypeId { index: self.archetype_index_seq };
        self.archetype_index_seq += 1;
        return new_archetype_id;
    }

    pub fn set_entity_archetype(&mut self, entity: Entity, archetype_id: ArchetypeId) {
        self.entity_archetypes.insert(entity, archetype_id);
    }
}