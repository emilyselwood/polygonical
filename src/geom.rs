/// Geometry helper functions for various things. None of this is exposed outside the library
use crate::point::Point;

// calculate the area of a triangle
pub fn area_of_triangle(a: Point, b: Point, c: Point) -> f64 {
    0.5 * matrix_determinant(b.y - a.x, b.x - a.x, c.y - a.y, c.x - a.y)
}

/// lines_intersect returns true if the line between a and b intersects with a line between c and d.
/// Note: if the lines intersect past the two points false will be returned.
pub fn lines_intersect(a: Point, b: Point, c: Point, d: Point) -> bool {
    let o1 = orientation(a, b, c);
    let o2 = orientation(a, b, d);

    let o3 = orientation(c, d, a);
    let o4 = orientation(c, d, b);

    if o1 != o2 && o3 != o4 {
        return true;
    }

    if o1 == Orientation::Collinear && on_segment(a, c, b) {
        return true;
    }
    if o2 == Orientation::Collinear && on_segment(a, d, b) {
        return true;
    }
    if o3 == Orientation::Collinear && on_segment(c, a, d) {
        return true;
    }
    if o4 == Orientation::Collinear && on_segment(c, b, d) {
        return true;
    }

    false
}

#[derive(PartialEq)]
enum Orientation {
    Collinear,
    Clockwise,
    AntiClockwise,
}

fn orientation(a: Point, b: Point, c: Point) -> Orientation {
    let v: f64 = (b.y - a.y) * (c.x - b.x) - (b.x - a.x) * (c.y - b.y);

    if v == 0.0 {
        return Orientation::Collinear;
    } else if v > 0.0 {
        return Orientation::Clockwise;
    }
    Orientation::AntiClockwise
}

fn on_segment(a: Point, b: Point, c: Point) -> bool {
    b.x <= a.x.max(c.x) && b.x >= a.x.min(c.x) && b.y <= a.y.max(c.y) && b.y >= a.y.min(c.y)
}

fn matrix_determinant(a: f64, b: f64, c: f64, d: f64) -> f64 {
    a * d - b * c
}

#[cfg(test)]
mod tests {
    use float_cmp::approx_eq;

    use crate::geom::area_of_triangle;
    use crate::geom::lines_intersect;
    use crate::geom::Point;

    #[test]
    fn does_not_intersect() {
        assert!(!lines_intersect(
            Point::zero(),
            Point::new(1.0, 1.0),
            Point::new(1.0, 0.0),
            Point::new(2.0, 1.0)
        ));
    }

    #[test]
    fn does_intersect() {
        assert!(lines_intersect(
            Point::zero(),
            Point::new(1.0, 1.0),
            Point::new(1.0, 0.0),
            Point::new(0.0, 1.0)
        ));
    }

    macro_rules! triangle_area_test {
        ($($name:ident: $value:expr,$expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let result = area_of_triangle($value.0, $value.1, $value.2);
                    assert!(approx_eq!(f64, result, $expected, ulps = 2))
                }
            )*
        };
    }

    triangle_area_test!(
        ta_test1: (Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(0.0, 0.0)), 0.0,
        ta_test2: (Point::new(0.0, 0.0), Point::new(0.0, 0.0), Point::new(0.0, 1.0)), 0.0,
        ta_test3: (Point::new(0.0, 1.0), Point::new(3.0, 6.0), Point::new(6.0, 2.0)), 13.5,

        ta_test4: (Point::new(0.0, 0.0), Point::new(1.0, 0.0), Point::new(0.0, 0.0)), 0.0,
        ta_test5: (Point::new(0.0, 0.0), Point::new(0.0, 0.0), Point::new(0.0, 1.0)), 0.0,
        ta_test6: (Point::new(0.0, 0.0), Point::new(0.0, 1.0), Point::new(1.0, 1.0)), 0.5,
        ta_test7: (Point::new(0.0, 0.0), Point::new(1.0, 1.0), Point::new(1.0, 0.0)), 0.5,
    );
}
