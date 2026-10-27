use std::{error::Error, fmt};

#[derive(Debug)]
pub enum PolygonicalError {
    NotEnoughPoints(usize),
    SelfIntersecting,
}

impl fmt::Display for PolygonicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PolygonicalError::NotEnoughPoints(n) => {
                write!(
                    f,
                    "Trying to create a polygon with {} points. Requires at least 3",
                    n
                )
            }
            PolygonicalError::SelfIntersecting => {
                write!(f, "Polygon is self intersecting.",)
            }
        }
    }
}

impl Error for PolygonicalError {}
