use std::hash::{Hash, Hasher};

#[derive(Debug, Clone, Copy)]
pub struct UnorderedPair<T>(pub T, pub T);

impl<T: Ord> UnorderedPair<T> {
    fn new(a: T, b: T) -> Self {
        if a <= b {
            UnorderedPair(a, b)
        } else {
            UnorderedPair(b, a)
        }
    }
}

impl<T: Ord> PartialEq for UnorderedPair<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.0 == other.0 && self.1 == other.1) || (self.0 == other.1 && self.1 == other.0)
    }
}

impl<T: Ord> Eq for UnorderedPair<T> {}

impl<T: Ord + Hash> Hash for UnorderedPair<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut vec = vec![&self.0, &self.1];
        vec.sort();
        vec.hash(state);
    }
}
