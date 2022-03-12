/// Defines the index functionality.
pub trait Index: PartialEq + Eq + PartialOrd + Ord + Sized {
    /// Constructs a new index from a specified value.
    fn new(x: usize) -> Self;

    /// Constructs a new index from a specified index.
    #[inline(always)]
    fn next(&self) -> Self {
        let u = self.index() + 1;
        Self::new(u)
    }

    /// Returns the usize value of the index
    fn index(&self) -> usize;

    /// Returns the minimum value of index.
    fn zero() -> Self;

    /// Determines if a given index is the undifined value.
    #[inline(always)]
    fn is_zero(&self) -> bool {
        *self == <Self as Index>::zero()
    }
}

macro_rules! index_impl {
    ($name:ident) => {
        impl Index for $name {
            #[inline(always)]
            fn new(x: usize) -> Self {
                x as $name
            }

            #[inline(always)]
            fn index(&self) -> usize {
                *self as usize
            }

            #[inline(always)]
            fn zero() -> Self {
                ::std::$name::MIN
            }
        }
    };
}

index_impl!(u8);
index_impl!(u16);
index_impl!(u32);
index_impl!(u64);
index_impl!(u128);

#[cfg(test)]
mod tests {
    use super::Index;

    #[test]
    fn u8_undefined() {
        let u = u8::zero();
        assert_eq!(u, 0);
    }

    #[test]
    fn u8_is_undefined() {
        let u = u8::zero();
        assert!(u.is_zero());
    }
}
