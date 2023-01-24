use std::fmt;

use float_cmp::approx_eq;

#[derive(Debug, Clone, Copy)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        return Point { x, y };
    }

    pub fn zero() -> Self {
        Point { x: 0.0, y: 0.0 }
    }

    pub fn new_max() -> Self {
        Point {
            x: f64::MAX,
            y: f64::MAX,
        }
    }

    pub fn new_min() -> Self {
        Point {
            x: f64::MIN,
            y: f64::MIN,
        }
    }

    pub fn min(self, other: Point) -> Point {
        Point {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }

    pub fn max(self, other: Point) -> Point {
        Point {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn invert(self) -> Point {
        Point {
            x: -self.x,
            y: -self.y,
        }
    }

    // offset / translate this point by another one.
    pub fn translate(self, by: Point) -> Point {
        Point {
            x: self.x + by.x,
            y: self.y + by.y,
        }
    }

    // Return the angle in radians to another point
    pub fn angle_to(self, other: &Point) -> f64 {
        let translated = other.translate(self.invert());

        translated.x.atan2(translated.y)
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
        approx_eq!(f64, self.x, other.x, ulps = 2) && approx_eq!(f64, self.y, other.y, ulps = 2)
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

    use std::f64::consts::PI;

    use float_cmp::approx_eq;

    use crate::point::Point;
    #[test]
    fn angle_to() {
        let p = Point::new(2.0, 1.0);
        let target = Point::new(3.0, 2.0);

        let result = p.angle_to(&target);
        println!("{}", result);
        assert!(approx_eq!(f64, result, 45.0 * (PI / 180.0), ulps = 2))
    }
}
