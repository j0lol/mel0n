#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
#![cfg_attr(test, test_runner(agb::test_runner::test_runner))]

use alloc::borrow::ToOwned;
use agb::display::tiled::TiledMap;
pub mod physics;
pub mod fruit;
pub mod math_helpers;
pub mod world;

extern crate alloc;

use agb::display::tiled::RegularBackgroundSize;
use crate::world::State;
use alloc::boxed::Box;
use alloc::vec;
use alloc::vec::Vec;
use core::mem::swap;
use agb::display::object::{AffineMatrixInstance, Graphics, ObjectUnmanaged};
use agb::display::{Priority, HEIGHT, WIDTH};
use agb::fixnum::{num, Num, Vector2D};
use agb::{include_aseprite, include_background_gfx, println, Gba};
use agb::display::affine::{AffineMatrix};
use agb::input::{Button, ButtonController};
use agb::interrupt::VBlank;
use crate::fruit::{FruitState, TERMINAL_VELOCITY};
use crate::math_helpers::{fsplat, fvec, iclamp, isplat};
use crate::physics::{clamp, Velocity};

const FLOOR: i32 = 148;
const WALL_L: i32 = 62;
const WALL_R: i32 = 179;

static GRAPHICS: &Graphics = include_aseprite!("gfx/fruits.aseprite");
static MISC_GRAPHICS: &Graphics = include_aseprite!("gfx/misc.aseprite");

include_background_gfx!(generated_background, "000000", DATA => "gfx/test_logo_basic.png");

type Fixed = Num<i32, 8>;

fn falling_block_game(gba: &'static mut Gba) -> ! {

    let (mut unmanaged, mut sprites) = gba.display.object.get_unmanaged();

    let mut s = State {
        affines: make_affines(),
        objects: Default::default(),
        cursor_position: isplat(0),
        sprites: Default::default(),
        fruits: Default::default(),
    };
    
    // let mut misc_objects: Vec<ObjectUnmanaged> = vec![];

    s.cursor_position = Vector2D::new(90, 30);
    
    let melon_sprite = GRAPHICS.sprites().get(3).unwrap();
    
    s.sprites.push(
        Box::new(sprites.get_vram_sprite(melon_sprite))
    );
    
    // let cross_sprite = MISC_GRAPHICS.sprites().first().unwrap();
    // 
    // s.sprites.push(
    //     Box::new(sprites.get_vram_sprite(cross_sprite))
    // );

    let mut input = ButtonController::new();

    let v_blank = VBlank::get();
    
    const NEW_MELON_HEIGHT: i32 = 16;

    s.new_fruit(Vector2D::new(90, NEW_MELON_HEIGHT));

    let mut my_melon = 0;

    let mut ground_timer = 30;

    let mut aim = 90;

    loop {
        v_blank.wait_for_vblank();
        input.update();

        println!("GT {ground_timer}");

        if input.is_pressed(Button::START) {
            // return;
        }
        if input.is_pressed(Button::LEFT) {
            aim -= 1;
        }
        if input.is_pressed(Button::RIGHT) {
            aim += 1;
        }
        aim = iclamp(aim, WALL_L + 8, WALL_R - 8);

        for fruit in s.fruits.iter_mut() {
            fruit.collided_with_fruits = vec![];
        }

        for current_fruit in 0..s.fruits.len() {

            // Get fruit & change state
            let fruit = &mut s.fruits[current_fruit];
            if fruit.state == FruitState::Held {
                if input.is_pressed(Button::A) {
                    fruit.state = FruitState::Falling ;
                } else {
                    if input.is_pressed(Button::LEFT) {
                        fruit.set_position(fruit.get_position() + fvec(-1., 0.));
                    }
                    if input.is_pressed(Button::RIGHT) {
                        fruit.set_position(fruit.get_position() + fvec(1., 0.));
                    }
                    fruit.set_position(clamp(fruit.get_position(), fvec(WALL_L as f32 + 8.0, 0.0), fvec( WALL_R as f32 - 8.0, WIDTH as f32)));
                    continue;
                }
            }

            fruit.rotation.angle += fruit.rotation.speed;
            fruit.world_object.set_affine_matrix(s.affines[affine_index(fruit.rotation.angle)].clone());

            let mut collided_with = vec![];

            let mut nudge: Vector2D<_> = Default::default();

            // Make potential physics object
            let mut fruit_physics_object = fruit.circle();
            fruit_physics_object.velocity = Velocity(if fruit.state != FruitState::Held {
                let gravity: Fixed = Num::new(98)/1000;
                clamp(
                    fruit.velocity.0 + Vector2D::new(num!(0.), gravity),
                    fsplat(-TERMINAL_VELOCITY),
                    fsplat(TERMINAL_VELOCITY)
                )
            } else { fsplat(0.0) });
            fruit_physics_object.position += fruit.velocity.0;

            // Collide with other fruits
            {
                let rest: Vec<_> = {
                    s.fruits
                        .iter_mut()
                        .enumerate()
                        .filter(|(n, fruit)| {*n != current_fruit
                            // && !fruit.collided_with_fruits.contains(&current_fruit)
                        })
                        .collect()
                };

                for (n, other_fruit) in rest.into_iter() {
                    let circle_nudge = fruit_physics_object.intersects(other_fruit.circle());
                    if circle_nudge.is_some() {
                        
                        // let mut marker_obj = ObjectUnmanaged::new(*s.sprites[1].to_owned());
                        // marker_obj.set_position((fruit_physics_object.position + circle_nudge.unwrap()).floor());
                        // marker_obj.show();
                        // misc_objects.push(marker_obj);
                        
                        // collided_with.push(n);

                        // swap(&mut fruit_physics_object.velocity.0, &mut other_fruit.velocity.0);
                        // other_fruit.set_position(
                        //     other_fruit.get_position() +
                        //         other_fruit.circle().intersects(fruit_physics_object).unwrap_or(fsplat(0.0))
                        // 
                        // );
                        // other_fruit.set_position(
                        //     other_fruit.get_position() +
                        //         other_fruit.circle().in_playfield().unwrap_or(fsplat(0.0))
                        // );
                    }
                    // nudge += circle_nudge.unwrap_or(fsplat(0.0));
                };
            }

            // Collide with walls
            let wall_nudge = fruit_physics_object.in_playfield().unwrap_or(fsplat(0.0));

            if wall_nudge.y != num!(0.) {
                fruit_physics_object.velocity.0.y *= num!(-0.2);

                if fruit_physics_object.velocity.0.y.abs() <= num!(0.2) {
                    fruit_physics_object.velocity.0.y = num!(0.);
                }
            }
            
            if wall_nudge.x != num!(0.) {
                fruit_physics_object.velocity.0.x *= num!(-0.2);
                if fruit_physics_object.velocity.0.x.abs() <= num!(0.2) {
                    fruit_physics_object.velocity.0.x = num!(0.);
                }
            }

            // nudge += wall_nudge;

            // Resolve problem
            let fruit = &mut s.fruits[current_fruit];

            if fruit.state == FruitState::Falling {
                let still = fruit_physics_object.velocity.0 == -nudge;
                if still {
                    ground_timer -= 1;
                }
                if ground_timer == 0 {
                    fruit.state = FruitState::Rolling;
                }
            }

            println!("FRUIT {current_fruit} VELOCITY {:?}", fruit_physics_object.velocity.0);
            fruit.set_position(fruit.get_position() + fruit_physics_object.velocity.0 + nudge);
            fruit.velocity = fruit_physics_object.velocity;
            fruit.collided_with_fruits = collided_with;
        }

        if s.fruits.get(my_melon).is_some_and(|fruit| fruit.state == FruitState::Rolling) {
            ground_timer = 30;
            s.new_fruit(Vector2D::new(aim, NEW_MELON_HEIGHT));
            my_melon += 1;
        }

        write_to_oam(unmanaged.iter(), &s.fruits.iter().map(|x| {x.world_object.clone()}).collect());
        
        // write_to_oam(unmanaged.iter(), &misc_objects);
    }
}

#[agb::entry]
fn main(mut gba: Gba) -> ! {
    {
        let (t0, mut vram) = gba.display.video.tiled0();

        let mut map = t0.background(
            Priority::P0,
            RegularBackgroundSize::Background32x32,
            generated_background::DATA.tiles.format()
        );
        vram.set_background_palettes(generated_background::PALETTES);

        map.fill_with(&mut vram, &generated_background::DATA);
        map.commit(&mut vram);
        map.set_visible(true);
    }

    let gba: &'static mut Gba = Box::leak(Box::new(gba));

    falling_block_game(gba);
}


fn make_affines() -> [AffineMatrixInstance; 32] {

    let mut fractions: [Fixed; 32] = [Num::new(0); 32];
    for i in 0..32 {
        fractions[i] = Num::new(i as _)/32;
    }
    let mut affines = [ const { None }; 32];
    for (n, frac) in fractions.into_iter().enumerate() {
        affines[n] = Some(AffineMatrixInstance::new(AffineMatrix::from_rotation(frac.clone().change_base::<_, 4>()).to_object_wrapping()));
    }
    affines.map(|x| x.unwrap())
}

fn affine_index(degrees: Fixed) -> usize {
    let rounded = degrees / (360 / 32);
    let rounded = rounded.floor().rem_euclid(32);

    rounded as usize
}



use agb::display::object::{OamIterator};

fn write_to_oam(oam_iterator: OamIterator, objects: &Vec<ObjectUnmanaged>) {
    for (object, slot) in objects.iter().zip(oam_iterator) {
        slot.set(&object);
    }
}