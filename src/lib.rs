#![doc = include_str!("../README.md")]
pub mod boundingbox;
pub mod point;
pub mod polygon;

mod geom;
mod maths;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
