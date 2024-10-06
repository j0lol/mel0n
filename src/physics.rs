use crate::physics::Wall::{Horizontal, Vertical};
use crate::{Fixed, FLOOR, WALL_L, WALL_R};
use agb::fixnum::{num, Num, Vector2D};
use core::cmp::PartialEq;
use agb::println;
use crate::math_helpers::{fsplat, sq};

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
    pub(crate) fn intersects(self: &Circle, b: Circle) -> bool {
        let a = self;

        // AB: get vector pointing from A_p to B_p
        let vector_ab = (a.position - b.position).change_base::<Fixed>();
        let magnitude_ab = vector_ab.magnitude();

        let touching_radii = b.radius + a.radius;

        // we know that R_a + R_b is less than AB
        // get intersection
        let intersection_magnitude = Num::new(touching_radii) - magnitude_ab;


        // If the intersection is positive, there is a collision
        // if intersection_magnitude > num!(0.) {
        //
        //     // Create a vector with angle of AB and magnitude of the intersection
        //     // let intersection_vector = vector_ab.normalise() * intersection_magnitude;
        //     // Some(intersection_vector)
        //
        // } else {
        //     // None
        // }

        intersection_magnitude > num!(0.)
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



#[derive(Clone, Copy)]
pub struct MovingCircle {
    before: Circle,
    pub(crate) after: (Vector2D<Fixed>, Velocity)
}

impl MovingCircle {

    pub fn new(circle: Circle) -> MovingCircle {
        MovingCircle {
            before: circle,
            after: (fsplat(0.0), Velocity(fsplat(0.0)))
        }
    }

    pub fn before(&self) -> Circle {
        self.before
    }

    pub fn after(&self) -> Circle {
        Circle {
            position: self.before.position + self.after.0 + self.after.1.0,
            radius: 0,
            velocity: self.after.1,
        }
    }

    pub fn movement(&self) -> Vector2D<Fixed> {
        let t_0 = self.before();
        let t_1 = self.after();

        t_1.position - t_0.position
    }
}

pub fn two_circle_interpolate(circle_a: MovingCircle, circle_b: MovingCircle) -> Fixed {

    // At some point, these two circles have to touch
    // Circles touch when d^2 = (r_a + r_b)^2
    // Solve for this equation
    
    // Assertations:
    // at t=0, d^2 > (r_a + r_b)^2
    // at t=1, d^2 < (r_a + r_b)^2
    // at t=n, where n > 0 & n < 1, d^2 = (r_a + r_b)^2
    
    // Constant
    let radii_squared = (circle_a.before.radius + circle_b.before.radius)^2;

    // At t=0
    let distance_squared = ((circle_b.before().position) - (circle_a.before().position)).magnitude_squared();

    
    // At t=1
    // let distance_squared = ((circle_b.after().position) - (circle_a.after().position)).magnitude_squared();
    
    
    // (x_a + (t*v_a) + x_b)  = (r_a + r_b)^2
    
    let pythagorean_distance = {
        
        let t = num!(1.0);
        
        let v_a = circle_a.after.1.0;
        let v_b = circle_b.after.1.0;
        
        let p_a = circle_a.before.position;
        let p_b = circle_b.before.position;

        let x = (p_a.x + t * v_a.x) + (p_b.x + t * v_b.x);
        let y = (p_a.y + t * v_a.y) + (p_b.y + t * v_b.y);

        (x * x + y * y).sqrt()
        
    };


    // a,b,c,d = 𝑣2(𝑥),𝑣1(𝑥),𝑣2(𝑦),𝑣1(𝑦)
    let (a, b, c, d) = (circle_b.after.1.0.x, circle_a.after.1.0.x, circle_b.after.1.0.y, circle_a.after.1.0.y);
    let (x1, x2, y1, y2) = (circle_a.before.position.x, circle_b.before.position.x, circle_a.before.position.y, circle_b.before.position.y, );

    // 𝐴=(𝑎−𝑏)^2+(𝑐−𝑑)^2
    // 𝐵=2((𝑥2−𝑥1)(𝑎−𝑏)+(𝑦2−𝑦1)(𝑐−𝑑))
    // 𝐶=𝑥1^2+𝑥2^2+𝑥3^2+𝑥4^2−2(𝑥1𝑥2+𝑥3𝑥4)−(𝑟1+𝑟2)^2
    let two: Fixed = num!(2.0);
    
    let quadr_a = sq(a - b) + sq(c - d);
    let quadr_b = two * ( (x2 - x1) * (a - b) + (y2 - y1) * (c - d));
    let quadr_c_a = sq(x1) + sq(x2) + sq(y1) + sq(y2);
    let quadr_c_b = two * ( (x1 * x2) + (y1 * y2));
    let quadr_c_c =  sq(Fixed::from(circle_a.before.radius + circle_b.before.radius));

    let quadr_c = sq(x1) + sq(x2) + sq(y1) + sq(y2) - (two * ( (x1 * x2) + (y1 * y2) )) - sq(Fixed::from(circle_a.before.radius + circle_b.before.radius));

    println!("QUAD SOLVE: A: {quadr_a}, B: {quadr_b}, C: {quadr_c}");
    println!("QUAD SOLVE: Ca: {quadr_c_a}, Cb: {quadr_c_b}, Cc: {quadr_c_c}");
    // 𝑇= ( −𝐵±√(𝐵^2−4𝐴𝐶) ) / 2𝐴
    
    let discriminant = sq(quadr_b) - num!(4.) * quadr_a * quadr_c;

    println!("discriminant: {discriminant}");
    
    let t1 = ( - quadr_b + discriminant.sqrt() ) / two * quadr_a;
    let t2 = ( - quadr_b - discriminant.sqrt() ) / two * quadr_a;

    println!("QUAD SOLVE: {t1}, {t2}");
    let t = t1.abs().min(t2.abs());
    
    
    t
    

}


pub struct Ball {
    x: Fixed,
    y: Fixed,
    xvel: Fixed,
    yvel: Fixed,
    radius: Fixed,
}

impl Ball {
    
    
    pub fn from_circle(circle: Circle) -> Ball {
        Ball {
            x: circle.position.x,
            y: circle.position.y,
            xvel: circle.velocity.0.x,
            yvel: circle.velocity.0.y,
            radius: circle.radius.into()
        }
    }
    
    pub fn time_to_collision(&self, other: &Ball) -> Num<i32, 8> {

        let radius = self.radius;
        let xvel = self.xvel;
        let yvel = self.yvel;
        let x = self.x;
        let y = self.y;
        let distance = (radius + other.radius) * (radius + other.radius);
        let a = (xvel - other.xvel) * (xvel - other.xvel) + (yvel - other.yvel) * (yvel - other.yvel);
        let b = num!(2.) * ((x - other.x) * (xvel - other.xvel) + (y - other.y) * (yvel - other.yvel));
        let c = (x - other.x) * (x - other.x) + (y - other.y) * (y - other.y) - distance;
        let d = b * b - num!(4.) * a * c;

        
        let (t1, t2) = if a == num!(0.0) {
            (num!(0.), num!(0.))
        } else {
            let e = if d > num!(0.) {d.sqrt()} else {num!(0.)};
            let t1 = (-b - e) / (num!(2.) * a);    // Collison time, +ve or -ve
            let t2 = (-b + e) / (num!(2.) * a);    // Exit time, +ve or -ve

            (t1, t2)
        };
        

        
        // println!("t1 {t1}, t2 {t2}");
        
        // if (t1 < num!(0.) && t2 > num!(0.) && b <= num!(-1e-6)) {
        //     return num!(0.);
        // }
        t1
    }
}