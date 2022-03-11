pub mod prelude;

pub use crate::prelude::Direction;

#[cfg(test)]
mod tests {
    use quickcheck_macros::quickcheck;

    #[quickcheck]
    fn opposite_is_diff(b: bool) -> bool {
        let x = !b;
        x != b
    }

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
