use crate::fruit::{Fruit, FruitState, Rotation};
use crate::affine_index;
use agb::display::object::AffineMode::AffineDouble;
use agb::display::object::{AffineMatrixInstance, ObjectUnmanaged, SpriteVram};
use agb::fixnum::{num, Vector2D};
use alloc::boxed::Box;
use alloc::vec::Vec;

pub struct State {
    pub affines: [AffineMatrixInstance; 32],
    pub objects: Vec<ObjectUnmanaged>,
    pub cursor_position: Vector2D<i32>,
    pub sprites: Vec<Box<SpriteVram>>,
    pub fruits: Vec<Fruit>,
}

impl State {
    pub fn new_fruit(&mut self, position: Vector2D<i32>) -> usize {
        let mut object = ObjectUnmanaged::new(*self.sprites[0].clone());

        object.set_affine_matrix(self.affines[affine_index(num!(0.))].clone());
        object.set_position(position);
        object.show_affine(AffineDouble);
        // let index = self.objects.len();
        // self.objects.push(object);
        
        let new_fruit = Fruit {
            world_object: object,
            real_position: position.change_base(),
            radius: 8,
            rotation: Rotation { angle: num!(0.), speed: num!(-360.0)/32 },
            state: FruitState::Held,
            velocity: Default::default(),
            collided_with_fruits: Default::default()
        };

        let index = self.fruits.len();
        self.fruits.push(new_fruit);

        index

    }
}

#[derive(Debug)]
pub struct Nudge(pub Vector2D<i32>);
