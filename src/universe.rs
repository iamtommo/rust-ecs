use std::collections::{LinkedList, HashMap};
use crate::entity::Entity;
use crate::cmd::{CmdChain, CmdCreateEntity};

/// top level unit of isolation
pub struct Universe {
    pub(crate) free_entity_indices: LinkedList<u64>,
    pub(crate) entity_versions: HashMap<u64, u64>
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
            entity_versions
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
}


#[cfg(test)]
mod tests {
    use super::*;

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

}