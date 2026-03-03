use core::fmt;
use core::num::NonZeroU32;

#[derive(Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id(NonZeroU32);

impl Id {
    pub fn from_index(index: u32) -> Self {
        Self(NonZeroU32::new(index + 1).expect("index+1 is nonzero"))
    }

    pub fn index(self) -> u32 {
        self.0.get() - 1
    }
}

impl fmt::Debug for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Id({})", self.index())
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.index())
    }
}

pub type NodeId = Id;
pub type CompId = Id;
pub type PortId = Id;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn id_round_trip_index() {
        for i in [0_u32, 1, 2, 42, 10_000] {
            let id = Id::from_index(i);
            assert_eq!(id.index(), i);
        }
    }

    #[test]
    fn option_id_is_small() {
        assert_eq!(
            core::mem::size_of::<Id>(),
            core::mem::size_of::<Option<Id>>()
        );
    }
}
