use std::ops::Not;

pub trait Indexer {
    type Output;
    fn to_index(&self) -> Self::Output;
}

#[derive(Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(usize)]
pub enum Direction {
    Outgoing = 0,
    Incoming = 1,
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        *self
    }
}

impl From<Direction> for bool {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Outgoing => false,
            Direction::Incoming => true,
        }
    }
}

impl From<bool> for Direction {
    fn from(b: bool) -> Self {
        if b { Direction::Incoming} else {Direction::Outgoing}
    }
}

impl From<Direction> for usize {
    fn from(d: Direction) -> Self {
        d as usize
    }
}

impl From<&Direction> for usize {
    fn from(d: &Direction) -> Self {
        *d as usize
    }
}

impl From<usize> for Direction {
    fn from(u: usize) -> Self {
        if u == 0 { Direction::Outgoing } else { Direction::Incoming }
    }
}

impl Indexer for Direction {
    type Output = usize;

    fn to_index(&self) -> Self::Output {
        self.into()
    }
}

impl Not for Direction {
    type Output = Direction;

    fn not(self) -> Self::Output {
        match self {
            Direction::Outgoing => Direction::Incoming,
            Direction::Incoming => Direction::Outgoing,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Direction, Indexer};

    use quickcheck::Arbitrary;
    use quickcheck_macros::quickcheck;

    impl Arbitrary for Direction {
        fn arbitrary(g: &mut quickcheck::Gen) -> Self {
            let x = i8::arbitrary(g);
            if x % 2 == 0 { Direction::Outgoing } else { Direction:: Incoming }
        }
    }

    #[quickcheck]
    fn from_bool(b: bool) -> bool {
        let x: Direction = b.into();
        b == x.into()
    }

    #[quickcheck]
    fn to_bool(d: Direction) -> bool {
        let b: bool = d.into();
        d == b.into()
    }

    #[quickcheck]
    fn from_usize(u: usize) -> bool {
        let d: Direction = u.into();
        if u == 0 { u == d.into() } else { 1usize == d.into() }
    }

    #[quickcheck]
    fn to_usize(d: Direction) -> bool {
        let u: usize = d.into();
        d == u.into()
    }

    #[quickcheck]
    fn ref_to_usize(d: Direction) -> bool {
        let u: usize = (&d).into();
        d == u.into()
    }

    #[quickcheck]
    fn not_operator(d: Direction) -> bool {
        d != !d
    }

    #[quickcheck]
    fn non_not_operator(d: Direction) -> bool {
        d == !(!d)
    }

    #[quickcheck]
    fn indexer(d: Direction) -> bool {
        let idx = d.to_index();
        let u: usize = d.into();

        idx == u
    }
}
