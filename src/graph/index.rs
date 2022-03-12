use std::fmt::Debug;

/// Defines the index functionality.
pub trait Index: PartialEq + Eq + PartialOrd + Ord + Default + Debug + Sized {
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

macro_rules! gindex_impl {
    ($name:ident) => {
        impl<Idx: Index> Index for $name<Idx> {
            fn new(x: usize) -> Self {
                Self(Idx::new(x))
            }
        
            fn index(&self) -> usize {
                self.0.index()
            }
        
            fn zero() -> Self {
                Self(Idx::zero())
            }
        }
        
        impl<Idx> From<Idx> for $name<Idx> {
            fn from(idx: Idx) -> Self {
                Self(idx)
            }
        }
        
        impl<Idx: std::fmt::Debug> std::fmt::Debug for $name<Idx> {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "{:?}({:?})", stringify!($name), self.0)
            }
        }
        
        impl<Idx: Index> From<usize> for $name<Idx> {
            fn from(x: usize) -> Self {
                Self(Idx::new(x))
            }
        }
    };
}

pub type DefaultIdx = u8;

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct NodeIndex<Idx = DefaultIdx>(Idx);
gindex_impl!(NodeIndex);

#[derive(Copy, Clone, Default, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct EdgeIndex<Idx = DefaultIdx>(Idx);
gindex_impl!(EdgeIndex);

#[cfg(test)]
mod tests {
    use super::Index;

    #[test]
    fn u8_zero() {
        let u = u8::zero();
        assert_eq!(u, 0);
    }

    #[test]
    fn u8_is_zero() {
        let u = u8::zero();
        assert!(u.is_zero());
    }

    #[test]
    fn u8_new() {
        let u = u8::new(10);
        assert_eq!(u, 10);
    }

    #[test]
    fn u8_next() {
        let u = u8::new(10).next();
        assert_eq!(u, 11);
    }

    #[test]
    fn u8_index() {
        let u = u8::new(10).index();
        assert_eq!(u, 10);
    }
}
