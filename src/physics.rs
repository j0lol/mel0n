use crate::physics::Wall::{Horizontal, Vertical};
use crate::{Fixed, FLOOR, WALL_L, WALL_R};
use agb::fixnum::{num, Num, Vector2D};
use core::cmp::PartialEq;


#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Velocity(pub Vector2D<Fixed>);

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Circle {
    pub position: Vector2D<Fixed>,
    pub radius: i32,
    pub velocity: Velocity
}

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum CollisionDirection {
    Up,
    Down,
    Left,
    Right
}
#[derive(Default)]
pub struct Colliding {
    pub up: bool,
    pub down: bool,
    pub left: bool,
    pub right: bool
}
impl Colliding {
    pub fn add(&mut self, dir: CollisionDirection) {
        match dir {
            CollisionDirection::Up => {self.up = true}
            CollisionDirection::Down => {self.down = true}
            CollisionDirection::Left => {self.left = true}
            CollisionDirection::Right => {self.right = true}
        };
    }
}

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum WallDirectionHorizontal {
    Left,
    Right
}

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum WallDirectionVertical {
    Top,
    Bottom
}

#[derive(PartialOrd, PartialEq, Copy, Clone)]
pub enum Wall {
    Horizontal(i32, WallDirectionHorizontal),
    Vertical(i32, WallDirectionVertical)
}

pub static PLAY_AREA: (Wall, Wall, Wall) = (Horizontal(WALL_L, WallDirectionHorizontal::Left), Horizontal(WALL_R, WallDirectionHorizontal::Right), Vertical(FLOOR, WallDirectionVertical::Bottom));

impl Circle {

    /// Returns a vector containing the amount in each direction that the circle intersected with another.
    pub(crate) fn intersects(self: &Circle, b: Circle) -> Option<Vector2D<Fixed>> {
        let a = self;

        // AB: get vector pointing from A_p to B_p
        let vector_ab = (a.position - b.position).change_base::<Fixed>();
        let magnitude_ab = vector_ab.magnitude();

        let touching_radii = b.radius + a.radius;

        // we know that R_a + R_b is less than AB
        // get intersection
        let intersection_magnitude = Num::new(touching_radii) - magnitude_ab;


        // If the intersection is positive, there is a collision
        if intersection_magnitude > num!(0.) {

            // Create a vector with angle of AB and magnitude of the intersection
            let intersection_vector = vector_ab.normalise() * intersection_magnitude;
            Some(intersection_vector)

        } else {
            None
        }
    }

    fn contains(self: Circle, p: Vector2D<Fixed>) -> bool {
        let x = self;
        let distance = (p.change_base() - x.position).change_base::<Fixed>().magnitude_squared();
        distance > Num::new(x.radius^2)
    }

    pub fn in_playfield(self: Circle) -> Option<Vector2D<Fixed>> {
        
        let in_floor = self.in_wall(Wall::Vertical(FLOOR, WallDirectionVertical::Bottom));
        let in_wall_l = self.in_wall(Wall::Horizontal(WALL_L, WallDirectionHorizontal::Left));
        let in_wall_r = self.in_wall(Wall::Horizontal(WALL_R, WallDirectionHorizontal::Right));
        
        in_floor.or(in_wall_l.or(in_wall_r))
    }
    
    pub(crate) fn in_wall(self: Circle, wall: Wall) -> Option<Vector2D<Fixed>> {
        use WallDirectionVertical as V;
        use WallDirectionHorizontal as H;

        match wall {
            Horizontal(x, H::Left)  => {
                let intersection = Num::new(x) - (self.position.x - self.radius);
                (intersection > num!(0.)).then_some(Vector2D::new(intersection, num!(0.)))
            }
            Horizontal(x, H::Right)  => {
                let intersection = (self.position.x + self.radius) - Num::new(x);
                (intersection > num!(0.)).then_some(Vector2D::new(-intersection, num!(0.)))
            }
            Vertical(y, V::Bottom) => {
                let intersection = (self.position.y + self.radius) - Num::new(y);
                (intersection > num!(0.)).then_some(Vector2D::new(num!(0.), -intersection))
            }
            _ => unreachable!()
        }
    }
}

pub fn clamp<T: PartialOrd + Copy + Clone + agb::fixnum::Number>(n: Vector2D<T>, lower: Vector2D<T>, upper: Vector2D<T>) -> Vector2D<T>{
    let mut out = n;
    if n.x < lower.x { out.x = lower.x; }
    if n.y < lower.y { out.y = lower.y; }

    if n.x > upper.x { out.x = upper.x; }
    if n.y > upper.y { out.y = upper.y; }

    out
}