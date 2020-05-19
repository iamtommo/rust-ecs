use crate::universe::Universe;

pub trait System {
    fn create(&mut self, universe: &mut Universe);
    fn update(&mut self, universe: &mut Universe);
    fn destroy(&mut self, universe: &mut Universe);
}