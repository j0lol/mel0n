use crate::physics::{Circle, Velocity};
use crate::Fixed;
use agb::display::object::ObjectUnmanaged;
use agb::fixnum::Vector2D;
use alloc::vec::Vec;

pub(crate) const TERMINAL_VELOCITY: f32 = 6.0;

pub struct Rotation {
    pub(crate) angle: Fixed,
    pub(crate) speed: Fixed,
}
#[derive(PartialEq, Debug)]
pub enum FruitState {
    Held,
    Falling,
    Rolling
}
pub struct Fruit {
    pub world_object: ObjectUnmanaged,
    pub real_position: Vector2D<Fixed>,
    pub radius: u8,
    pub rotation: Rotation,
    pub state: FruitState,
    pub velocity: Velocity,
    pub collided_with_fruits: Vec<usize>,
}
impl Fruit {
    pub fn get_position(&self) -> Vector2D<Fixed> {
        self.real_position
    }

    pub fn set_position(&mut self, position: Vector2D<Fixed>) {
        self.real_position = position;
        self.world_object.set_position(position.floor() - Vector2D::new(16, 16));
    }
    pub fn circle(&self) -> Circle {
        Circle {
            position: self.get_position(),
            radius: 8,
            velocity: self.velocity,
        }
    }
}
