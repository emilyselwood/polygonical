# Polygonical

A library for interacting with polygons on a 2d plane.

## Examples

Rotate a polygon:
```rust
    use polygonical::polygon::Polygon;
    use polygonical::point::Point;

    let poly = Polygon::new(vec![
        Point::new(0.0, 1.0),
        Point::new(1.0, 1.0),
        Point::new(1.0, 0.0),
    ]);

    // get the area of the polygon
    let area = poly.area();

    // rotate the polygon around its own center by 90 degrees
    let rotated = poly.rotate_around_center(90.0_f64.to_radians());

    println!("area: {} rotated: {}", area, rotated);
```


Create an approximation of a circle:

```rust
    use polygonical::polygon::Polygon;
    use polygonical::point::Point;

    let num_points = 16;
    let radius = 2.0;
    let center = Point::new(10.0, 20.0);

    // Note: we use an integer number of degrees here because rust won't let you iterate over floats like this.
    let points = (0..360).step_by(360/num_points)
        .map(|a| Point::new(radius, 0.0).rotate((a as f64).to_radians()).translate(center))
        .collect();

    let circle = Polygon::new(points);
     
    let approx_area = circle.area();
    let area = std::f64::consts::PI * radius * radius;

    println!("area: {} aprox_area: {} difference: {}", area, approx_area, area - approx_area);
```

## Features

* Points
* Polygons
* Bounding boxes
* Translations of points
* Polygons contain points
* Polygon is_self_intersecting
* Polygon area
* Translations of polygons
* Rotations of points
* Rotations of polygons

## Wanted Features

Things we want to implement but haven't yet.

* Scale of points
* Scale of polygons
* Overlap detection
* Contains detection
* Polygon unions
* Polygon subtraction


## Unwanted Features

Things this library won't do.

* 3d Geometry
* Output to things like svg (that is for another library)
* Coordinate system transforms, epsg codes, pixel space to world space etc.

## Design goals

* Correct
* Safe
* Fast

In that order