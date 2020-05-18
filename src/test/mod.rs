use crate::component::{Component};

pub struct Position {
    x: f32
}
impl Component for Position {}

#[cfg(test)]
mod tests {
    use crate::test::Position;
    use std::any::Any;
    use crate::component::Component;

    #[test]
    fn it_works() {
        /*let position = Position { x: 0f32 };
        let component = position as dyn Component;
        assert_eq!(position.type_id(), component.type_id());*/
    }

}
