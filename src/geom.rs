/// Geometry helper functions for various things. None of this is exposed outside the library
use crate::point::Point;

// calculate the area of a triangle
pub fn area_of_triangle(a: Point, b: Point, c: Point) -> f64 {
    0.5 * matrix_determinant(b.y - a.x, b.x - a.x, c.y - a.y, c.x - a.y)
}

/// Find the point of intersection of two lines a->b and c->d
/// Will return None if the lines don't intersect.
pub fn point_of_intersection(a: Point, b: Point, c: Point, d: Point) -> Option<Point> {
    let a1 = b.y - a.y;
    let b1 = a.x - b.x;
    let c1 = (a1 * a.x) + (b1 * a.y);

    let a2 = d.y - c.y;
    let b2 = c.x - d.x;
    let c2 = (a2 * c.x) + (b2 * c.y);

    let determinant = a1 * b2 - a2 * b1;

    if determinant == 0.0 {
        // they might be parallel but overlap so check if the start of the second line is in side the first.
        let x_r = (b.x - c.x) / (b.x - a.x);
        let y_r = (b.y - c.y) / (b.y - a.y);
        if x_r > 0.0 && x_r < 1.0 && y_r > 0.0 && y_r < 1.0 {
            return Some(c);
        }
        return None;
    }

    let x = (b2 * c1 - b1 * c2) / determinant;
    let y = (a1 * c2 - a2 * c1) / determinant;

    // check that x,y is inside the two points a->b and c->d
    let x_r1 = (b.x - x) / (b.x - a.x);
    let y_r1 = (b.y - y) / (b.y - a.y);

    let x_r2 = (d.x - x) / (d.x - c.x);
    let y_r2 = (d.y - y) / (d.y - c.y);

    // The clippy suggestion here is less obvious to me
    #[allow(clippy::manual_range_contains)]
    if x_r1 < 0.0
        || x_r1 > 1.0
        || y_r1 < 0.0
        || y_r1 > 1.0
        || x_r2 < 0.0
        || x_r2 > 1.0
        || y_r2 < 0.0
        || y_r2 > 1.0
    {
        return None;
    }

    Some(Point::new(x, y))
}

pub fn line_intersects_others(a: (Point, Point), others: &[(Point, Point)]) -> Option<usize> {
    for (i, el) in others.iter().enumerate() {
        if lines_intersect(a.0, a.1, el.0, el.1) {
            return Some(i);
        }
    }

    None
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
    use super::area_of_triangle;
    use super::lines_intersect;
    use super::point_of_intersection;
    use crate::point::Point;
    use crate::tests::assert_f64;

    macro_rules! intersection_tests {
        ($($name:ident: $line_a:expr,$line_b:expr,$expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let result = lines_intersect($line_a.0, $line_a.1, $line_b.0, $line_b.1);
                    assert_eq!(result, $expected);
                }
            )*
        };
    }

    intersection_tests!(
        does_not_intersect: (Point::zero(), Point::new(1.0, 1.0)), (Point::new(1.0, 0.0), Point::new(2.0, 1.0)), false,
        does_intersect: (Point::zero(), Point::new(1.0, 1.0)), (Point::new(1.0, 0.0), Point::new(0.0, 1.0)), true,
        does_intersect_but_not: (Point::new(1.0, 0.0), Point::new(1.0, 2.0)), (Point::new(0.0, 3.0), Point::new(2.0, 3.0)), false,
    );

    macro_rules! triangle_area_test {
        ($($name:ident: $value:expr,$expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let result = area_of_triangle($value.0, $value.1, $value.2);
                    assert_f64!(result, $expected);
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

    macro_rules! intersection_point_test {
        ($($name:ident: $line_a:expr,$line_b:expr,$expected:expr,)*) => {
            $(
                #[test]
                fn $name() {
                    let result = point_of_intersection($line_a.0, $line_a.1, $line_b.0, $line_b.1);
                    assert_eq!(result, $expected);
                }
            )*
        };
    }

    intersection_point_test!(
        not_intersecting: (Point::new(0.0, 0.0), Point::new(1.0, 1.0)), (Point::new(1.0, 0.0), Point::new(2.0, 1.0)), None,
        simple_90: (Point::new(1.0, 0.0), Point::new(1.0, 2.0)), (Point::new(0.0, 1.0), Point::new(2.0, 1.0)), Some(Point::new(1.0, 1.0)),
        intersects_outside: (Point::new(1.0, 0.0), Point::new(1.0, 2.0)), (Point::new(0.0, 3.0), Point::new(2.0, 3.0)), None,
        intersection_parallel: (Point::new(0.0, 0.0), Point::new(2.0, 2.0)), (Point::new(1.0, 1.0), Point::new(3.0, 3.0)), Some(Point::new(1.0, 1.0)),
    );
}
