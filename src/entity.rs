use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct Entity {
    pub id: u64,
    pub version: u64
}

impl Display for Entity {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Entity {{ id: {}, version: {} }}", self.id, self.version)
    }
}