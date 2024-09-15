use crate::fruit::TERMINAL_VELOCITY;
use alloc::boxed::Box;
use alloc::vec::Vec;
use core::mem::swap;
use agb::display::object::{AffineMatrixInstance, OamManaged, Object, ObjectUnmanaged, SpriteVram};
use agb::display::object::AffineMode::AffineDouble;
use agb::fixnum::{num, Num, Vector2D};
use agb::input::ButtonController;
use agb::println;
use crate::{affine_index, Fixed};
use crate::fruit::{Fruit, FruitState, Rotation};
use crate::math_helpers::{fsplat, isplat};
use crate::physics::{clamp, Circle, PLAY_AREA};

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
    
    // pub fn get_fruits(&mut self, index: usize) -> (Option<&mut &mut Fruit>, Vec<&mut Fruit>) {
    // 
    //     let (mut first, rest): (Vec<_>, Vec<_>) = {
    //         self.fruits
    //             .iter_mut()
    //             .enumerate()
    //             .filter(|(n, _)| *n == index)
    //             .map(|x| x.1)
    //             .partition(|x1| {x1.state != FruitState::Rolling})
    //     };
    //     
    //     assert!(first.len() >= 1);
    //     assert_eq!(first.len(), 0, "you'er stupid");
    //     
    //     (first.get_mut(0), rest)
    //     // let (mut first_fruit_object, fruit_objects): (Vec<_>, Vec<_>) = {
    //     //     let objects = self.objects.iter_mut();
    //     //     let fruits = self.fruits.iter_mut();
    //     //     fruits.zip(objects).enumerate().filter(|(n, _)| {*n == index}).map(|(n, x)| x).partition(|(x, y)| {x.state != FruitState::Rolling})
    //     // };
    // }
    
    // pub fn update_fruit(&mut self, index: usize, ground_timer: &mut i32, input: &ButtonController) {
    // 
    //     let fruit = &mut self.fruits[index];
    //     let fruit_object = &mut self.objects[fruit.object];
    //     let offset_object_position = fruit_object.position() + Vector2D::new(16, 16);
    // 
    //     let colliding = self.collide_fruit(index);
    //     println!("nudge {colliding:?}");
    //     
    //     let active_fruit = fruit.state != FruitState::Rolling;
    //     let gravity_affected = fruit.state != FruitState::Held;
    // 
    //     let velocity = if gravity_affected {
    //         let gravity: Fixed = Num::new(98)/1000;
    //         clamp(
    //             fruit.velocity.0 + Vector2D::new(num!(0.), gravity),
    //             fsplat(-TERMINAL_VELOCITY),
    //             fsplat(TERMINAL_VELOCITY)
    //         )
    //     } else { fsplat(0.0) };
    // 
    //     // let other_fruits = s.fruits
    //     //     .iter_mut()
    //     //     .enumerate()
    //     //     .filter(|(index, _)| fruit.object != *index)
    //     //     .map(|(i, x)| x)
    //     //     .collect();
    // 
    // 
    //     // Only runs if it's the "controlled" fruit
    //     if active_fruit {
    // 
    //         fruit.control(input, &mut isplat(30));
    // 
    //         if let (Some(nudge), FruitState::Falling { timeout: _ }) = (&colliding, &fruit.state) {
    //             if nudge.0.y != 0 {
    //                 *ground_timer -= 1;
    //             }
    //             if *ground_timer <= 0 {
    //                 fruit.state = FruitState::Rolling;
    //                 return;
    //             }
    //         }
    //     }
    // 
    //     if !gravity_affected { return };
    // 
    //     let gravity: Fixed = Num::new(98)/1000;
    //     fruit.velocity.0 = clamp(
    //         fruit.velocity.0 + Vector2D::new(num!(0.), gravity),
    //         fsplat(-TERMINAL_VELOCITY),
    //         fsplat(TERMINAL_VELOCITY)
    //     );
    //     if colliding.is_some() {
    //         fruit.velocity.0 = fsplat(0.0);
    //     }
    //     fruit_object.set_position(fruit_object.position() + fruit.velocity.0.floor() - colliding.unwrap_or(Nudge(isplat(0))).0);
    // }


    // pub fn collide_fruit(&mut self, index: usize) -> Option<Nudge> {
    //     
    //     let (mut first_fruit_object, fruit_objects): (Vec<_>, Vec<_>) = {
    //         let objects = self.objects.iter_mut();
    //         let fruits = self.fruits.iter_mut();
    //         fruits.zip(objects).enumerate().filter(|(n, _)| {*n == index}).map(|(n, x)| x).partition(|(x, y)| {x.state != FruitState::Rolling})
    //     };
    //     
    //     assert!(first_fruit_object.len() <= 1);
    //     
    //     if first_fruit_object.is_empty() {
    //         return None;
    //     }
    //     
    //     let (ref mut fruit, ref mut object) = &mut first_fruit_object[0];
    // 
    //     let mut nudge = fsplat(0.0);
    // 
    //     for (other_fruit, other_object) in fruit_objects.into_iter().filter(|(x, _)| {x.state == FruitState::Rolling}) {
    //         if let Some(intersection) =  fruit.circle(object.position()).intersects(other_fruit.circle(other_object.position())) {
    //             nudge -= intersection;
    // 
    //             swap(&mut fruit.velocity, &mut other_fruit.velocity)
    //         }
    //     }
    // 
    //     for wall in [&PLAY_AREA.0, &PLAY_AREA.1, &PLAY_AREA.2] {
    //         if let Some(intersection) = fruit.circle(object.position()).in_wall(*wall) {
    //             nudge -= intersection;
    //         }
    //     }
    // 
    //     if nudge != fsplat(0.0) {
    //         Some(Nudge(nudge.floor()))
    //     } else {
    //         None
    //     }
    // }

    // pub fn circle_collide(state: &mut State, mut c: Circle) -> Option<Nudge> {
    // 
    //     let others: Vec<Fruit> = vec![];
    // 
    //     todo!();
    // 
    //     let mut nudge = fsplat(0.0);
    // 
    //     for mut other_circle in others {
    //         if let Some(intersection) =  c.intersects(other_circle.circle(&state)) {
    //             nudge -= intersection;
    // 
    //             swap(&mut c.velocity, &mut other_circle.velocity)
    //         }
    //     }
    // 
    //     for wall in [&PLAY_AREA.0, &PLAY_AREA.1, &PLAY_AREA.2] {
    //         if let Some(intersection) = c.in_wall(*wall) {
    //             nudge -= intersection;
    //         }
    //     }
    // 
    //     if nudge != fsplat(0.0) {
    //         Some(Nudge(nudge.floor()))
    //     } else {
    //         None
    //     }
    // }
}

#[derive(Debug)]
pub struct Nudge(pub Vector2D<i32>);
