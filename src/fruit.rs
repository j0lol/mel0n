use crate::world::State;
use alloc::boxed::Box;
use alloc::vec::Vec;
use agb::display::object::AffineMode::AffineDouble;
use agb::display::object::{Object, ObjectUnmanaged};
use agb::fixnum::{num, Num, Vector2D};
use agb::input::{Button, ButtonController};
use agb::interrupt::VBlank;
use agb::println;
use crate::{affine_index, Fixed, GRAPHICS};
use crate::math_helpers::{fsplat, fvec, isplat};
use crate::physics::{clamp, Circle, Colliding, Velocity};

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
        // self.world_object.position() + Vector2D::new(16, 16)
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

    // #[deprecated]
    // pub(crate) fn new<'gba, 'obj>(state: &'gba mut State, position: Vector2D<i32>, spin: Fixed) -> Fruit {
    //     let v_blank = VBlank::get();
    //     let sprite = GRAPHICS.sprites().get(3).unwrap();
    //     // let mut object = state.gfx.object_sprite(sprite);
    // 
    //     object.set_affine_matrix(state.affines[affine_index(num!(0.))].clone());
    //     object.set_position(position);
    //     object.show_affine(AffineDouble);
    // 
    //     Fruit {
    //         object: {
    //             let index = state.objects.len();
    //             state.objects.push(object);
    //             index
    //         },
    //         radius: 8,
    //         rotation: Rotation { angle: num!(0.), speed: spin },
    //         state: FruitState::Held,
    //         velocity: Default::default()
    //     }
    // }

    pub(crate) fn move_and_rotate(&mut self, input: Option<(&ButtonController, &mut i32)>) {

        let active_fruit = self.state != FruitState::Rolling;
        let gravity_affected = self.state != FruitState::Held;
    
        let velocity = if gravity_affected {
            let gravity: Fixed = Num::new(98)/1000;
            clamp(
                self.velocity.0 + Vector2D::new(num!(0.), gravity),
                fsplat(-TERMINAL_VELOCITY),
                fsplat(TERMINAL_VELOCITY)
            )
        } else { fsplat(0.0) };
        
        if let Some(input) = input {

            self.control(input.0, &mut isplat(40));
        }
        
        self.velocity.0 = velocity;
        self.set_position(self.get_position() + self.velocity.0);
    }

    pub(crate) fn control(&mut self, input: &ButtonController, position: &mut Vector2D<i32>) {

        
        if !input.is_pressed(Button::LEFT.intersection(Button::RIGHT)) {
            if input.is_pressed(Button::LEFT) {
                position.x = position.x - 1;
                // cursor.set_position(Vector2D::new(cursor.position().x - 1, cursor.position().y));
            }
            if input.is_pressed(Button::RIGHT) {
                position.x = position.x + 1;
            }
        }
        
        if self.state == FruitState::Held {
            if !input.is_pressed(Button::LEFT.intersection(Button::RIGHT)) {
                if input.is_pressed(Button::LEFT) {
                    position.x = position.x - 1;
                }
                if input.is_pressed(Button::RIGHT) {
                    position.x = position.x + 1;
                }
            }
        
            if input.is_just_pressed(Button::A) {
                self.state = FruitState::Falling
            }
        }
    }
}
