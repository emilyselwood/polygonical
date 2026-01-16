use std::fmt;

use getset::Getters;

use crate::{geom, point::Point};

/// Represents a line between two points.
#[derive(Debug, Clone, Getters)]
#[getset(get = "pub")]
pub struct Line {
    a: Point,
    b: Point,
}

impl Line {
    pub fn new(a: Point, b: Point) -> Self {
        Line { a, b }
    }

    pub fn intersects_with(&self, other: &Self) -> bool {
        geom::lines_intersect(self.a, self.b, other.a, other.b)
    }

    pub fn point_of_intersection(&self, other: &Self) -> Option<Point> {
        geom::point_of_intersection(self.a, self.b, other.a, other.b)
    }
}

impl fmt::Display for Line {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "line({} -> {})", self.a, self.b)
    }
}
