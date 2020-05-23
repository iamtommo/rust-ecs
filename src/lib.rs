pub mod component;
pub mod universe;
pub mod entity;
pub mod cmd;
pub mod archetype;
pub mod query;
pub mod system;
pub mod singleton;

#[cfg(test)]
mod test_full;
#[cfg(test)]
mod test_archetype;
#[cfg(test)]
mod test_universe;
#[cfg(test)]
mod test_query;
#[cfg(test)]
mod test_systems;