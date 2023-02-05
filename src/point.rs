use std::fmt;

use float_cmp::approx_eq;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new<T>(x: T, y: T) -> Self
    where
        T: Into<f64>,
    {
        Point {
            x: x.into(),
            y: y.into(),
        }
    }

    /// Create a point at the origin
    pub fn zero() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    /// Create a point with f64::MAX for the coords.
    /// Useful for finding min values of points
    pub fn new_max() -> Self {
        Point {
            x: f64::MAX,
            y: f64::MAX,
        }
    }

    /// Create a point with f64::MIN for the coords.
    /// Useful for finding max values of points
    pub fn new_min() -> Self {
        Point {
            x: f64::MIN,
            y: f64::MIN,
        }
    }

    /// Given another point return the min of the x and y values
    pub fn min(self, other: Point) -> Point {
        Point {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    /// Given another point return the max of the x and y values
    pub fn max(self, other: Point) -> Point {
        Point {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    /// Flip the sign of both x and y coords
    pub fn invert(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }

    /// offset / translate this point by another one.
    pub fn translate(self, by: Point) -> Point {
        Point {
            x: self.x + by.x,
            y: self.y + by.y,
        }
    }

    /// Return the angle in radians to another point
    pub fn angle_to(&self, other: &Point) -> f64 {
        let translated = other.translate(self.invert());

        let result = translated.y.atan2(translated.x);
        if result < 0.0 {
            return result + 360.0_f64.to_radians();
        }
        result
    }

    /// Rotate the given point around the origin by angle radians.
    pub fn rotate(&self, angle: f64) -> Point {
        Point {
            x: (self.x * angle.cos()) - (self.y * angle.sin()),
            y: (self.y * angle.cos()) + (self.x * angle.sin()),
        }
    }

    // TODO: bring in the travel code
}

impl fmt::Display for Point {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "({}, {})", self.x, self.y)
    }
}

impl PartialEq for Point {
    // float equal is always evil, but we will use approx_eq here to give us a reasonable answer.
    fn eq(&self, other: &Self) -> bool {
        approx_eq!(f64, self.x, other.x, epsilon = 0.000003, ulps = 2)
            && approx_eq!(f64, self.y, other.y, epsilon = 0.000003, ulps = 2)
    }
}

/*
Convert a float tuple into a point automagically
*/
impl From<(f64, f64)> for Point {
    fn from(other: (f64, f64)) -> Point {
        Point {
            x: other.0,
            y: other.1,
        }
    }
}

/*
Convert an int tuple into a point automagically
*/
impl From<(i32, i32)> for Point {
    fn from(other: (i32, i32)) -> Point {
        Point {
            x: other.0 as f64,
            y: other.1 as f64,
        }
    }
}

// TODO: compare function for points thats fuzzy for the float matching...

#[cfg(test)]
mod tests {

    use crate::point::Point;

    use crate::tests::assert_f64;

    #[test]
    fn from_int() {
        let p = Point::new(1, 5);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 5.0);
    }

    macro_rules! angle_tests {
        ($($name:ident: $point_a:expr, $point_b:expr, $expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    assert_f64!($point_a.angle_to(&$point_b), $expected.to_radians());
                }
            )*
        };
    }
    angle_tests!(
        angle_zero: Point::new(2.0, 1.0), Point::new(3.0, 1.0), 0.0_f64,
        angle_45: Point::new(2.0, 1.0), Point::new(3.0, 2.0), 45.0_f64,
        angle_90: Point::new(2.0, 1.0), Point::new(2.0, 2.0), 90.0_f64,
        angle_180: Point::new(2.0, 1.0), Point::new(1.0, 1.0), 180.0_f64,
        angle_270: Point::new(2.0, 1.0), Point::new(2.0, 0.0), 270.0_f64,
    );

    #[test]
    fn angle_to() {
        let p = Point::new(2.0, 1.0);
        let target = Point::new(3.0, 2.0);

        let result = p.angle_to(&target);
        assert_f64!(result, 45.0_f64.to_radians());
    }

    #[test]
    fn rotate_a_point() {
        let p = Point::new(1.0, 0.0);
        let result = p.rotate(90.0_f64.to_radians());

        assert_eq!(result, Point::new(0.0, 1.0))
    }

    #[test]
    fn rotate_origin() {
        let p = Point::zero();
        let result = p.rotate(90.0_f64.to_radians());

        assert_eq!(result, p);
    }
}
