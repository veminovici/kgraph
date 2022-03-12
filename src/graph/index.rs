pub trait Index : Ord + Sized {
    /// Constructs a new index from a specified 
    fn new(x: usize) -> Self;

    /// Returns the usize value of the index
    fn index(&self) -> usize;

    /// Returns the maximum value of the index.
    fn max() -> Self;

    /// Returns the minimum value of index.
    fn undefined() -> Self;

    /// Determines if a given index is the max value.
    fn is_max(&self) -> bool {
        self.index() == <Self as Index>::max().index()
    }

    /// Determines if a given index is the undifined value.
    fn is_undefined(&self) -> bool {
        self.index() == <Self as Index>::undefined().index()
    }

    /// Determines if the given index has over-flowed
    fn is_valid(&self) -> bool {
        !self.is_max() && !self.is_undefined()
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
            fn max() -> Self {
                ::std::$name::MAX
            }
        
            #[inline(always)]
            fn undefined() -> Self {
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
        let u = u8::undefined();
        assert_eq!(u, 0);
    }
    
    #[test]
    fn u8_max() {
        let u = <u8 as Index>::max();
        assert_eq!(u8::MAX, u)
    }
}
