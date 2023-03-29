use std::ops::RangeInclusive;

trait InclusiveRangeExt {
    fn contains_range(&self, other: &Self) -> bool;
}

impl<T> InclusiveRangeExt for RangeInclusive<T>
where
    T: PartialOrd,
{
    fn contains_range(&self, other: &Self) -> bool {
        self.contains(other.start()) && self.contains(other.end())
    }
}

fn main() {
    dbg!((2..=4).contains_range(&(6..=8)));
    dbg!((6..=8).contains_range(&(2..=4)));
    dbg!((4..=6).contains_range(&(6..=6)));
}

