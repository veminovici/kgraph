use std::ops::Not;

#[derive(Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
#[repr(usize)]
pub enum Direction {
    Outgoing = 0,
    Incoming = 1,
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

impl Not for Direction {
    type Output = Direction;

    fn not(self) -> Self::Output {
        match self {
            Direction::Outgoing => Direction::Incoming,
            Direction::Incoming => Direction::Outgoing,
        }
    }
}

impl Direction {
    pub fn opposite(self) -> Self {
        let b: bool = self.into();
        (!b).into()
    }
}

impl Clone for Direction {
    fn clone(&self) -> Self {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::Direction;

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
    fn not_operator(d: Direction) -> bool {
        d != !d
    }

    #[quickcheck]
    fn non_not_operator(d: Direction) -> bool {
        d == !(!d)
    }
}
