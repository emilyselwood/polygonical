#![doc = include_str!("../README.md")]
pub mod boundingbox;
pub mod point;
pub mod polygon;

mod geom;
mod maths;

#[cfg(test)]
mod tests {
    
    macro_rules! assert_f64 {
        ($actual:expr, $expected:expr) => {
            use float_cmp::approx_eq;
            assert!(approx_eq!(f64, $actual, $expected, ulps = 2), "got:{} expected:{}", $actual, $expected);
        };
    }
    pub(crate) use assert_f64;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
