/*
Geometry helper functions for various things. None of this is exposed outside the library
*/

use crate::point::Point;

/*
lines_intersect returns true if the line between a and b intersects with a line between c and d.

Note: if the lines intersect past the two points false will be returned.
*/
pub fn lines_intersect(a:Point, b: Point, c: Point, d: Point) -> bool {

    let o1 = orientation(a, b, c);
    let o2 = orientation(a, b, d);
    
    let o3 = orientation(c, d, a);
    let o4 = orientation(c, d, b);

    if o1 != o2 && o3 != o4 {
        return true
    }

    if o1 == Orientation::Collinear && on_segment(a, c, b) {
        return true
    }
    if o2 == Orientation::Collinear && on_segment(a, d, b) {
        return true
    }
    if o3 == Orientation::Collinear && on_segment(c, a, d) {
        return true
    }
    if o4 == Orientation::Collinear && on_segment(c, b, d) {
        return true
    }

    return false
}


#[derive(PartialEq)]
enum Orientation {
    Collinear,
    Clockwise,
    AntiClockwise,
}

fn orientation(a:Point, b: Point, c: Point) -> Orientation {

    let v:f64 = (b.y - a.y) * (c.x - b.x) - (b.x - a.x) * (c.y - b.y);

    if v == 0.0 {
        return Orientation::Collinear;
    } else if v > 0.0 {
        return Orientation::Clockwise;
    }
    return Orientation::AntiClockwise;

}

fn on_segment(a:Point, b: Point, c: Point) -> bool {
    b.x <= a.x.max(c.x) && b.x >= a.x.min(c.x) &&
        b.y <= a.y.max(c.y) && b.y >= a.y.min(c.y) 
}



#[cfg(test)]
mod tests {
    use crate::geom::lines_intersect;
    use crate::geom::Point;

    #[test]
    fn does_not_intersect() {
        assert!(!lines_intersect( Point::zero(), Point::new(1.0, 1.0), Point::new(1.0, 0.0), Point::new(2.0, 1.0)));
    }

    #[test]
    fn does_intersect() {
        assert!(lines_intersect( Point::zero(), Point::new(1.0, 1.0), Point::new(1.0, 0.0), Point::new(0.0, 1.0)));
    }
}