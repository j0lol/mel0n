use crate::Fixed;
use agb::fixnum::{num, Num, Vector2D};

pub fn fvec<T: agb::fixnum::Number + agb::fixnum::FixedWidthUnsignedInteger, const N: usize>(
    x: f32,
    y: f32,
) -> Vector2D<Num<T, N>>
where
    Num<T, N>: agb::fixnum::Number,
{
    Vector2D::new(Num::from_f32(x), Num::from_f32(y))
}
pub fn isplat<T: Clone + Copy + agb::fixnum::FixedWidthUnsignedInteger>(x: T) -> Vector2D<T> {
    Vector2D::new(x, x)
}

pub fn fsplat<T: agb::fixnum::Number + agb::fixnum::FixedWidthUnsignedInteger, const N: usize>(
    x: f32,
) -> Vector2D<Num<T, N>> {
    fvec(x, x)
}

pub fn iclamp<T: PartialOrd + Copy + Clone + agb::fixnum::Number>(n: T, lower: T, upper: T) -> T {
    let mut out = n;

    if n < lower {
        out = lower;
    }
    if n > upper {
        out = upper;
    }

    out
}

pub trait FixedExtend {
    fn acos(self) -> Self;
}

impl FixedExtend for Fixed {
    fn acos(self) -> Self {
        let x = self;
        num!(1.57079) - num!(1.57079) * x
    }
}

pub fn sq(num: Fixed) -> Fixed {
    num * num
}
