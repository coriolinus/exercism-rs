
/// A custom set backed by a sorted Vec
///
/// Keeping the backing Vec sorted at all times should enable using
/// binary search to get better than linear perforance, without
/// simply duplicating HashSet or BTreeSet.
///
/// Performance should be generally comparable to that of a BTreeSet,
/// if a little slower, but it should take somewhat less space than a HashSet
#[derive(Clone, PartialEq, Debug)]
pub struct CustomSet<T> {
    items: Vec<T>,
}

impl<T> CustomSet<T> where T: PartialEq + Ord + Sized {
    pub fn new(items: Vec<T>) -> CustomSet<T> {
        // uses FromIterator to add all items in their proper places
        items.into_iter().collect()
    }
}

impl<T> CustomSet<T> where T: PartialEq + Ord {
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn contains(&self, item: &T) -> bool {
        self.items.binary_search(item).is_ok()
    }

    pub fn is_subset(&self, other: &CustomSet<T>) -> bool {
        self.iter().all(|i| other.contains(&i))
    }

    pub fn is_disjoint(&self, other: &CustomSet<T>) -> bool {
        self.iter().all(|i| !other.contains(&i))
    }

    pub fn add(&mut self, item: T) {
        if let Err(index) = self.items.binary_search(&item) {
            // docs: if the value is not found then Err is returned,
            // containing the index where a matching element could
            // be inserted while maintaining sorted order.
            //
            // We don't do anything unless it's an Err
            self.items.insert(index, item);
        }
    }

    pub fn iter(&self) -> std::slice::Iter<T> {
        self.items.iter()
    }
}

impl<T> CustomSet<T> where T: PartialEq + Ord + Clone {
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
    where T: PartialEq + Ord {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        iter.into_iter().fold(CustomSet { items: Vec::new() }, |mut ret, item| {
            ret.add(item);
            ret
        })
    }
}
