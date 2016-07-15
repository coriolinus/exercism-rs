
/// A custom set backed by a sorted Vec
///
/// Keeping the backing Vec sorted at all times should enable using
/// binary search to get better than linear perforance, without
/// simply duplicating HashSet or BTreeSet.
///
/// Performance should be generally comparable to that of a BTreeSet,
/// if a little slower, but should take somewhat less space.
#[derive(Clone, PartialEq)]
pub struct CustomSet<T> {
    items: Vec<T>,
}

impl<T> CustomSet<T> where T: PartialEq + PartialOrd {
    pub fn new(items: Vec<T>) -> CustomSet<T> {
        CustomSet { items: items }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn contains(&self, item: &T) -> bool {
        unimplemented!()
    }

    pub fn is_subset(&self, other: &CustomSet<T>) -> bool {
        self.iter().all(|i| other.contains(&i))
    }

    pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
        self.iter().all(|i| !other.contains(&i))
    }

    pub fn add(&mut self, item: T) {
        if !self.contains(&item) {

        }
        unimplemented!()
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.items.iter()
    }
}

impl<T> CustomSet<T> where T: PartialEq + PartialOrd + Clone {
    pub fn intersection(&self, other: &CustomSet<T>) -> CustomSet<T> {
        self.iter().filter(|i| other.contains(&i)).cloned().collect()
    }

    pub fn difference(&self, other: &CustomSet<T>) -> CustomSet<T> {
        self.iter().filter(|i| ! other.contains(&i)).cloned().collect()
    }

    pub fn union(&self, other: &CustomSet<T>) -> CustomSet<T> {
        other.iter().fold(self.clone(), |mut ret, item| {
            ret.add(item.clone());
            ret
        })
    }
}

impl<T> IntoIterator for CustomSet<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

impl<T> std::iter::FromIterator<T> for CustomSet<T>
    where T: PartialEq + PartialOrd {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        iter.into_iter().fold(CustomSet::new(Vec::new()), |mut ret, item| {
            ret.add(item);
            ret
        })
    }
}
