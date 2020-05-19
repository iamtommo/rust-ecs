use std::collections::LinkedList;
use crate::entity::Entity;
use std::error::Error;
use crate::universe::Universe;
use crate::component::Component;

/// ordered cmd buffer for batching entity operations (minimize chunk relayouts)
pub struct CmdChain {
    pub state: CmdChainState,
    pub cmds: LinkedList<Box<dyn Cmd>>
}

/// mutable cmd state
pub struct CmdChainState {
    pub last_created_entity: Option<Entity>
}

impl CmdChain {
    pub fn new() -> CmdChain {
        CmdChain {
            state: CmdChainState {
                last_created_entity: None
            },
            cmds: LinkedList::new()
        }
    }

    pub fn create_entity(&mut self) {
        self.cmds.push_back(Box::new(CmdCreateEntity {}))
    }

    pub fn destroy_entity(&mut self, entity: Entity) {
        self.cmds.push_back(Box::new(CmdDestroyEntity { entity: entity }));
    }
}

pub trait Cmd {
    fn exec(&self, universe: &mut Universe, state: &mut CmdChainState);
}

/// COMMAND: create entity
pub struct CmdCreateEntity {
}
impl Cmd for CmdCreateEntity {
    fn exec(&self, universe: &mut Universe, state: &mut CmdChainState) {
        let entity_id = universe.free_entity_indices.pop_front().unwrap();
        let entity_version = universe.entity_versions.get(&entity_id).unwrap();
        let entity = Entity { id: entity_id, version: *entity_version };
        state.last_created_entity = Option::Some(entity);
    }
}

/// COMMAND: destroy entity
pub struct CmdDestroyEntity {
    pub entity: Entity
}
impl Cmd for CmdDestroyEntity {
    fn exec(&self, universe: &mut Universe, state: &mut CmdChainState) {
        // increment version to invalidate previous entity handles
        let mut version = universe.entity_versions.get_mut(&self.entity.id).unwrap();
        *version += 1u64;
        universe.free_entity_indices.push_front(self.entity.id);
    }
}