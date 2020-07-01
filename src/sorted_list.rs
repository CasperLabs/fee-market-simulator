#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![deny(unused_imports)]

//! Simple sorted list collection like the one found in the .NET collections library.

use std::fmt;

use std::ops::RangeBounds;

use std::iter::FromIterator;

/// `SortedList` stores multiple `(K, V)` tuples ordered by K, then in the order of insertion for `V`.
/// Implmented using two `Vec` this should be fast for in-order inserts and quite bad in the
/// worst-case of reverse insertion order.
///
/// # Example
///
/// ```
/// use fee_market_simulator::sorted_list::SortedList;
///
/// let mut list: SortedList<u32, u8> = SortedList::new();
/// list.insert(0, 0);
/// list.insert(1, 1);
/// list.insert(0, 2);
///
/// assert_eq!(
///     list.iter().collect::<Vec<_>>(),
///     vec![(&0, &0), (&0, &2), (&1, &1)]);
/// ```
pub struct SortedList<K: Ord, V: PartialEq> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K: Ord, V: PartialEq> SortedList<K, V> {
    /// Creates a new as small as possible `SortedList`
    pub fn new() -> Self {
        SortedList {
            keys: Vec::new(),
            values: Vec::new(),
        }
    }

    /// Creates `SortedList` with preallocated capacity of `len`
    pub fn with_capacity(len: usize) -> Self {
        SortedList {
            keys: Vec::with_capacity(len),
            values: Vec::with_capacity(len),
        }
    }

    /// Returns the number of tuples
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Returns true if the collection is empty.
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Returns `true` if the `(key, value)` did not exist in the sorted list before and it exists now,
    /// `false` otherwise.
    pub fn insert(&mut self, key: K, value: V) -> bool {
        match self.keys.binary_search(&key) {
            Ok(found_at) => {
                let insertion_position = self.find_insertion_positition(found_at, &key, &value);

                if let Some(insertion_position) = insertion_position {
                    insertion_position.insert(key, value, &mut self.keys, &mut self.values);
                    true
                } else {
                    false
                }
            }
            Err(insert_at) => {
                self.keys.insert(insert_at, key);
                self.values.insert(insert_at, value);

                true
            }
        }
    }

    /// Returns the values of a specific key as a slice
    pub fn values_of(&self, key: &K) -> &[V] {
        let first = self.find_first_position(key).ok();
        match first {
            Some(first) => {
                let last = self.find_last_position(key).unwrap();
                &self.values[first..last]
            }
            None => &self.values[0..0],
        }
    }

    fn find_insertion_positition(
        &self,
        from: usize,
        key: &K,
        value: &V,
    ) -> Option<InsertionPosition> {
        let mut keys = self.keys.iter().skip(from);
        let mut values = self.values.iter().skip(from);

        let mut index: usize = from;

        loop {
            index += 1;

            match (keys.next(), values.next()) {
                (Some(other_key), Some(other_value)) => {
                    if key == other_key {
                        if value == other_value {
                            // found it already
                            return None;
                        }
                    } else {
                        // we ran past the matching keys, insert before
                        return Some(InsertionPosition::Before(index));
                    }
                }
                (None, None) => {
                    return Some(InsertionPosition::Last);
                }
                (_, _) => unreachable!(),
            };
        }
    }

    /// Iterate all stored tuples, keys in order, values in insertion order
    pub fn iter(&self) -> Tuples<K, V> {
        Tuples {
            keys: &self.keys,
            values: &self.values,
            low: 0,
            high: self.len(),
        }
    }

    /// Iterate over all keys, can contain duplicates
    pub fn keys(&self) -> ::std::slice::Iter<K> {
        self.keys.iter()
    }

    /// Iterate over all values
    pub fn values(&self) -> ::std::slice::Iter<V> {
        self.values.iter()
    }

    /// Returns the first (in insertion order) value of `key`
    pub fn first_value_of(&self, key: &K) -> Option<&V> {
        self.find_first_position(key)
            .ok()
            .map(|idx| &self.values[idx])
    }

    /// Returns the last (in insertion order) value of `key`
    pub fn last_value_of(&self, key: &K) -> Option<&V> {
        self.find_last_position(key)
            .ok()
            .map(|idx| &self.values[idx - 1])
    }

    fn find_first_position(&self, key: &K) -> Result<usize, usize> {
        match self.keys.binary_search(key) {
            Ok(mut pos) => {
                while pos > 0 && key == &self.keys[pos] {
                    pos -= 1;
                }

                if pos == 0 {
                    if key == &self.keys[0] {
                        Ok(0)
                    } else {
                        Ok(1)
                    }
                } else {
                    Ok(pos + 1)
                }
            }
            Err(pos) => Err(pos),
        }
    }

    fn find_last_position(&self, key: &K) -> Result<usize, usize> {
        match self.keys.binary_search(key) {
            Ok(mut pos) => {
                while pos < self.keys.len() && key == &self.keys[pos] {
                    pos += 1;
                }

                if pos == self.keys.len() {
                    // this is off by one ...
                    Ok(pos)
                } else {
                    Ok(pos)
                }
            }
            Err(pos) => Err(pos),
        }
    }

    /// Shrinks excess capacity from underlying vecs.
    pub fn shrink_to_fit(&mut self) {
        self.keys.shrink_to_fit();
        self.values.shrink_to_fit();
    }

    /// Removes element at a given index
    pub fn remove(&mut self, index: usize) {
        fn assert_failed(index: usize, len: usize) -> ! {
            panic!("removal index (is {}) should be < len (is {})", index, len);
        }

        let len = self.keys.len();
        if index >= len {
            assert_failed(index, len);
        }

        self.keys.remove(index);
        self.values.remove(index);

        // unsafe {
        //     // infallible
        //     let ret;
        //     {
        //         // the place we are taking from.
        //         let ptr = self.as_mut_ptr().add(index);
        //         // copy it out, unsafely having a copy of the value on
        //         // the stack and in the vector at the same time.
        //         ret = ptr::read(ptr);

        //         // Shift everything down to fill in that spot.
        //         ptr::copy(ptr.offset(1), ptr, len - index - 1);
        //     }
        //     self.set_len(len - 1);
        //     ret
        // }
    }

    /// Get the value of the element at a given index
    pub fn get(&self, index: usize) -> &V {
        &self.values[index]
    }
}

impl<K: Ord + Clone, V: PartialEq + Clone> Clone for SortedList<K, V> {
    fn clone(&self) -> Self {
        SortedList {
            keys: self.keys.clone(),
            values: self.values.clone(),
        }
    }
}

impl<K: Ord, V: PartialEq> FromIterator<(K, V)> for SortedList<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut this = Self::new();

        for (k, v) in iter {
            this.insert(k, v);
        }

        this
    }
}

trait ResultExt<A> {
    fn either(self) -> A;
}

impl<A> ResultExt<A> for Result<A, A> {
    fn either(self) -> A {
        match self {
            Ok(x) => x,
            Err(x) => x,
        }
    }
}

impl<K: Ord + PartialEq, V: PartialEq> SortedList<K, V> {
    /// Returns an iterator over the specified range of tuples
    pub fn range<R>(&self, range: R) -> Tuples<K, V>
    where
        R: RangeBounds<K>,
    {
        use std::ops::Bound::*;
        let start = match range.start_bound() {
            Included(key) => self.find_first_position(key).either().into(),
            Excluded(key) => self.find_last_position(key).either().into(),
            Unbounded => Some(0),
        };

        let end = match range.end_bound() {
            Included(key) => self.find_last_position(key).either(),
            Excluded(key) => self.find_first_position(key).either(),
            Unbounded => self.len(),
        };

        let skip = start.unwrap_or(self.keys.len());
        let take = if end <= skip { 0 } else { end };

        Tuples {
            keys: &self.keys,
            values: &self.values,
            low: skip,
            high: take,
        }
    }
}

impl<K: Ord, V: PartialEq> IntoIterator for SortedList<K, V> {
    type Item = (K, V);
    type IntoIter = IntoTuples<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        IntoTuples {
            keys: self.keys.into_iter(),
            values: self.values.into_iter(),
        }
    }
}

/// IntoIterator version of `Tuples`
pub struct IntoTuples<K, V> {
    keys: ::std::vec::IntoIter<K>,
    values: ::std::vec::IntoIter<V>,
}

impl<K, V> fmt::Debug for IntoTuples<K, V> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "IntoTuples {{ remaining: {} }}",
            self.keys.size_hint().0
        )
    }
}

impl<K, V> Iterator for IntoTuples<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        match (self.keys.next(), self.values.next()) {
            (Some(k), Some(v)) => (k, v).into(),
            (None, None) => None,
            _ => unreachable!(),
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.keys.size_hint()
    }
}

impl<K, V> DoubleEndedIterator for IntoTuples<K, V> {
    fn next_back(&mut self) -> Option<(K, V)> {
        match (self.keys.next_back(), self.values.next_back()) {
            (Some(k), Some(v)) => (k, v).into(),
            (None, None) => None,
            _ => unreachable!(),
        }
    }
}

impl<K, V> ExactSizeIterator for IntoTuples<K, V> {}

impl<K: Clone + Ord, V: PartialEq> Extend<(K, V)> for SortedList<K, V> {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = (K, V)>,
    {
        let mut temp = iter.into_iter().collect::<Vec<_>>();
        temp.sort_by_key(|&(ref k, _)| k.clone());

        for (k, v) in temp {
            self.insert(k, v);
        }
    }
}

impl<K: Ord + fmt::Debug, V: PartialEq + fmt::Debug> fmt::Debug for SortedList<K, V> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "SortedList {{ {:?} }}", &self.iter())
    }
}

/// Helper value for knowning where to insert the value
enum InsertionPosition {
    Before(usize),
    Last,
}

impl InsertionPosition {
    fn insert<K, V>(self, key: K, value: V, keys: &mut Vec<K>, values: &mut Vec<V>) {
        match self {
            InsertionPosition::Before(index) => {
                keys.insert(index - 1, key);
                values.insert(index - 1, value);

                assert_eq!(keys.len(), values.len());
            }
            InsertionPosition::Last => {
                keys.push(key);
                values.push(value);

                assert_eq!(keys.len(), values.len());
            }
        }
    }
}

/// Iterator over tuples stored in `SortedList`
pub struct Tuples<'a, K: 'a, V: 'a> {
    keys: &'a Vec<K>,
    values: &'a Vec<V>,
    low: usize,
    high: usize,
}

impl<'a, K, V> Iterator for Tuples<'a, K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<Self::Item> {
        if self.low < self.high {
            let low = self.low;
            self.low += 1;
            Some((&self.keys[low], &self.values[low]))
        } else {
            None
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.high - self.low;
        (len, Some(len))
    }
}

impl<'a, K, V> DoubleEndedIterator for Tuples<'a, K, V> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.high > self.low {
            self.high -= 1;
            let high = self.high;
            Some((&self.keys[high], &self.values[high]))
        } else {
            None
        }
    }
}

impl<'a, K, V> Clone for Tuples<'a, K, V> {
    fn clone(&self) -> Self {
        Tuples {
            keys: self.keys,
            values: self.values,
            low: self.low,
            high: self.high,
        }
    }
}

impl<'a, K: Ord + fmt::Debug, V: PartialEq + fmt::Debug> fmt::Debug for Tuples<'a, K, V> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let remaining = self.size_hint().0;
        let mut clone = self.clone();
        let mut idx = 0;
        write!(fmt, "[")?;
        while let Some(tuple) = clone.next() {
            if idx == remaining - 1 {
                write!(fmt, "{:?}", tuple)?;
            } else {
                write!(fmt, "{:?}, ", tuple)?;
            }
            idx += 1;
        }
        write!(fmt, "]")
    }
}

impl<'a, K, V> ExactSizeIterator for Tuples<'a, K, V> {}

#[cfg(test)]
mod tests {
    use super::SortedList;
    use std::fmt::Debug;

    /// Extension trait with asserting methods
    trait SortedListExt<K, V> {
        fn insert_only_new(&mut self, key: K, value: V);
    }

    impl<K: Debug + Clone + Ord, V: Debug + Clone + PartialEq> SortedListExt<K, V>
        for SortedList<K, V>
    {
        fn insert_only_new(&mut self, key: K, value: V) {
            let cloned_key = key.clone();
            let cloned_value = value.clone();

            assert!(
                self.insert(key, value),
                "pair existed already: ({:?}, {:?})",
                cloned_key,
                cloned_value
            );
        }
    }

    #[test]
    fn insert_in_order_and_iterate() {
        let mut list = SortedList::new();
        list.insert_only_new(0u32, 0u8);
        list.insert_only_new(1u32, 4u8);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some((&0, &0)));
        assert_eq!(iter.next(), Some((&1, &4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn insert_out_of_order_and_iterate() {
        let mut list = SortedList::new();
        list.insert_only_new(1u32, 4u8);
        list.insert_only_new(0u32, 0u8);

        let mut iter = list.iter();

        assert_eq!(iter.next(), Some((&0, &0)));
        assert_eq!(iter.next(), Some((&1, &4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn insert_duplicate() {
        let mut list = SortedList::new();
        assert!(list.insert(1u32, 4u8));
        assert!(!list.insert(1u32, 4u8));
    }

    #[test]
    fn insert_multiple_in_order() {
        let mut list = SortedList::new();
        list.insert_only_new(0u32, 0u8);
        list.insert_only_new(0u32, 1u8);
        list.insert_only_new(0u32, 2u8);
        list.insert_only_new(0u32, 3u8);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some((&0, &0)));
        assert_eq!(iter.next(), Some((&0, &1)));
        assert_eq!(iter.next(), Some((&0, &2)));
        assert_eq!(iter.next(), Some((&0, &3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn multiple_values_are_iterated_in_insertion_order() {
        let mut list = SortedList::new();
        list.insert_only_new(0u32, 3u8);
        list.insert_only_new(0u32, 2u8);
        list.insert_only_new(0u32, 1u8);
        list.insert_only_new(0u32, 0u8);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some((&0, &3)));
        assert_eq!(iter.next(), Some((&0, &2)));
        assert_eq!(iter.next(), Some((&0, &1)));
        assert_eq!(iter.next(), Some((&0, &0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterate_over_mixed_in_order() {
        let mut list = SortedList::new();
        list.insert_only_new(0u32, 0u8);
        list.insert_only_new(0u32, 1u8);
        list.insert_only_new(0u32, 2u8);
        list.insert_only_new(0u32, 3u8);
        list.insert_only_new(1u32, 4u8);
        list.insert_only_new(2u32, 5u8);
        list.insert_only_new(2u32, 6u8);
        list.insert_only_new(3u32, 7u8);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some((&0, &0)));
        assert_eq!(iter.next(), Some((&0, &1)));
        assert_eq!(iter.next(), Some((&0, &2)));
        assert_eq!(iter.next(), Some((&0, &3)));
        assert_eq!(iter.next(), Some((&1, &4)));
        assert_eq!(iter.next(), Some((&2, &5)));
        assert_eq!(iter.next(), Some((&2, &6)));
        assert_eq!(iter.next(), Some((&3, &7)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn iterate_over_mixed_out_of_order() {
        let mut list = SortedList::new();
        list.insert_only_new(3u32, 7u8);
        list.insert_only_new(0u32, 0u8);
        list.insert_only_new(1u32, 4u8);
        list.insert_only_new(0u32, 1u8);

        println!("{:?}", list);

        let mut iter = list.iter();
        assert_eq!(iter.next(), Some((&0, &0)));
        assert_eq!(iter.next(), Some((&0, &1)));
        assert_eq!(iter.next(), Some((&1, &4)));
        assert_eq!(iter.next(), Some((&3, &7)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn empty_values_of() {
        let list: SortedList<u32, u8> = SortedList::new();
        assert_eq!(list.values_of(&0).iter().next(), None);
    }

    #[test]
    fn iterate_values_of() {
        let mut list = SortedList::new();
        list.insert_only_new(1u32, 4u8);
        list.insert_only_new(0u32, 0u8);
        list.insert_only_new(0u32, 1u8);
        list.insert_only_new(2u32, 5u8);
        list.insert_only_new(0u32, 2u8);
        list.insert_only_new(3u32, 7u8);
        list.insert_only_new(0u32, 3u8);
        list.insert_only_new(2u32, 6u8);

        let mut values_of = list.values_of(&0).iter();
        assert_eq!(values_of.next(), Some(&0));
        assert_eq!(values_of.next(), Some(&1));
        assert_eq!(values_of.next(), Some(&2));
        assert_eq!(values_of.next(), Some(&3));
        assert_eq!(values_of.next(), None);

        let mut values_of = list.values_of(&1).iter();
        assert_eq!(values_of.next(), Some(&4));
        assert_eq!(values_of.next(), None);

        let mut values_of = list.values_of(&2).iter();
        assert_eq!(values_of.next(), Some(&5));
        assert_eq!(values_of.next(), Some(&6));
        assert_eq!(values_of.next(), None);

        let mut values_of = list.values_of(&3).iter();
        assert_eq!(values_of.next(), Some(&7));
        assert_eq!(values_of.next(), None);
    }

    #[test]
    fn extend_worst_case() {
        use std::time::Instant;

        // 1000, 100 => 4.08s (3.76s release) originally
        // 1000, 100 for copy types: 0.66s (0.23s release)
        let max_key = 1000;
        let max_val = 100;
        let mut input = Vec::with_capacity(max_key * max_val);
        for key in 0..max_key {
            for val in 0..max_val {
                input.push((max_key - key, val));
            }
        }

        let began = Instant::now();

        let mut slist = SortedList::new();
        slist.extend(input);

        let elapsed = began.elapsed();
        println!(
            "elapsed: {}.{:09}s",
            elapsed.as_secs(),
            elapsed.subsec_nanos()
        );
    }

    fn to_vec<'a, A: 'a + Copy, B: 'a + Copy, I: Iterator<Item = (&'a A, &'a B)>>(
        it: I,
    ) -> Vec<(A, B)> {
        it.map(|(a, b)| (*a, *b)).collect()
    }

    #[cfg(feature = "nightly")]
    #[test]
    fn range() {
        use std::collections::Bound::*;

        let mut list: SortedList<u32, u8> = SortedList::new();
        list.insert_only_new(1, 4);
        list.insert_only_new(0, 0);
        list.insert_only_new(0, 1);
        list.insert_only_new(2, 5);
        list.insert_only_new(0, 2);
        list.insert_only_new(3, 7);
        list.insert_only_new(0, 3);
        list.insert_only_new(2, 6);
        list.insert_only_new(4, 8);
        list.insert_only_new(6, 9);
        list.insert_only_new(6, 10);
        list.insert_only_new(9, 11);

        assert_eq!(
            to_vec(list.range((Unbounded, Included(2)))),
            vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 4), (2, 5), (2, 6)]
        );

        assert_eq!(
            to_vec(list.range((Unbounded, Excluded(2)))),
            vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 4)]
        );

        assert_eq!(
            to_vec(list.range((Included(0), Excluded(2)))),
            vec![(0, 0), (0, 1), (0, 2), (0, 3), (1, 4)]
        );

        assert_eq!(to_vec(list.range((Included(1), Excluded(2)))), vec![(1, 4)]);

        assert_eq!(to_vec(list.range((Included(2), Excluded(2)))), vec![]);

        assert_eq!(
            to_vec(list.range((Included(2), Included(2)))),
            vec![(2, 5), (2, 6)]
        );

        assert_eq!(
            to_vec(list.range((Included(2), Excluded(3)))),
            vec![(2, 5), (2, 6)]
        );

        assert_eq!(
            to_vec(list.range((Included(2), Included(3)))),
            vec![(2, 5), (2, 6), (3, 7)]
        );

        assert_eq!(
            to_vec(list.range((Included(2), Unbounded))),
            vec![(2, 5), (2, 6), (3, 7), (4, 8), (6, 9), (6, 10), (9, 11)]
        );

        assert_eq!(
            to_vec(list.range((Excluded(1), Unbounded))),
            vec![(2, 5), (2, 6), (3, 7), (4, 8), (6, 9), (6, 10), (9, 11)]
        );

        assert_eq!(
            to_vec(list.range((Excluded(0), Unbounded))),
            vec![
                (1, 4),
                (2, 5),
                (2, 6),
                (3, 7),
                (4, 8),
                (6, 9),
                (6, 10),
                (9, 11)
            ]
        );

        assert_eq!(
            to_vec(list.range((Excluded(4), Unbounded))),
            vec![(6, 9), (6, 10), (9, 11)]
        );

        assert_eq!(
            to_vec(list.range((Included(5), Unbounded))),
            vec![(6, 9), (6, 10), (9, 11)]
        );

        assert_eq!(
            to_vec(list.range((Excluded(5), Unbounded))),
            vec![(6, 9), (6, 10), (9, 11)]
        );

        assert_eq!(to_vec(list.range((Excluded(6), Unbounded))), vec![(9, 11)]);

        assert_eq!(to_vec(list.range((Excluded(6), Excluded(7)))), vec![]);

        assert_eq!(to_vec(list.range((Excluded(6), Included(8)))), vec![]);

        assert_eq!(to_vec(list.range((Excluded(6), Excluded(9)))), vec![]);

        assert_eq!(
            to_vec(list.range((Excluded(6), Included(9)))),
            vec![(9, 11)]
        );

        assert_eq!(
            to_vec(list.range((Excluded(7), Included(9)))),
            vec![(9, 11)]
        );

        assert_eq!(
            to_vec(list.range((Included(7), Included(9)))),
            vec![(9, 11)]
        );

        assert_eq!(
            to_vec(list.range((Excluded(8), Included(9)))),
            vec![(9, 11)]
        );

        assert_eq!(
            to_vec(list.range((Included(8), Included(9)))),
            vec![(9, 11)]
        );

        assert_eq!(to_vec(list.range(..)), to_vec(list.iter()));
    }

    #[test]
    fn first_value_of() {
        let mut list: SortedList<u32, u8> = SortedList::new();
        list.insert_only_new(1, 3);
        list.insert_only_new(0, 0);
        list.insert_only_new(0, 1);
        list.insert_only_new(2, 4);
        list.insert_only_new(0, 2);
        list.insert_only_new(3, 6);
        list.insert_only_new(2, 5);

        assert_eq!(list.first_value_of(&0), Some(&0));
        assert_eq!(list.first_value_of(&1), Some(&3));
        assert_eq!(list.first_value_of(&2), Some(&4));
        assert_eq!(list.first_value_of(&3), Some(&6));
    }

    #[test]
    fn last_value_of() {
        let mut list: SortedList<u32, u8> = SortedList::new();
        list.insert_only_new(1, 3);
        list.insert_only_new(0, 0);
        list.insert_only_new(0, 1);
        list.insert_only_new(2, 4);
        list.insert_only_new(0, 2);
        list.insert_only_new(3, 6);
        list.insert_only_new(2, 5);

        assert_eq!(list.last_value_of(&0), Some(&2));
        assert_eq!(list.last_value_of(&1), Some(&3));
        assert_eq!(list.last_value_of(&2), Some(&5));
        assert_eq!(list.last_value_of(&3), Some(&6));
    }

    #[test]
    fn double_ended_iter_empty() {
        let list: SortedList<u32, u8> = SortedList::new();
        assert_eq!(list.iter().next_back(), None);
    }

    #[test]
    fn double_ended_iter_single() {
        let mut list: SortedList<u32, u8> = SortedList::new();

        list.insert_only_new(1, 3);

        let mut iter = list.iter();
        assert_eq!(iter.next_back(), Some((&1, &3)));
        assert_eq!(iter.next_back(), None);
    }

    #[test]
    fn double_ended_iter_multiple() {
        let mut list: SortedList<u32, u8> = SortedList::new();

        list.insert_only_new(1, 3);
        list.insert_only_new(0, 0);
        list.insert_only_new(0, 1);
        list.insert_only_new(2, 4);
        list.insert_only_new(0, 2);
        list.insert_only_new(3, 6);
        list.insert_only_new(2, 5);

        assert_eq!(
            to_vec(list.iter().rev()),
            vec![(3, 6), (2, 5), (2, 4), (1, 3), (0, 2), (0, 1), (0, 0)]
        );
    }

    #[test]
    fn double_ended_iter_zig_zag() {
        let mut list: SortedList<u32, u8> = SortedList::new();

        list.insert_only_new(1, 3);
        list.insert_only_new(0, 0);
        list.insert_only_new(0, 1);
        list.insert_only_new(2, 4);
        list.insert_only_new(0, 2);
        list.insert_only_new(3, 6);
        list.insert_only_new(2, 5);

        let mut iter = list.iter();
        assert_eq!(iter.next(), (&0, &0).into());
        assert_eq!(iter.next_back(), (&3, &6).into());

        assert_eq!(iter.next(), (&0, &1).into());
        assert_eq!(iter.next_back(), (&2, &5).into());

        assert_eq!(iter.next(), (&0, &2).into());
        assert_eq!(iter.next_back(), (&2, &4).into());

        assert_eq!(iter.next(), (&1, &3).into());
        assert_eq!(iter.next_back(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn out_of_order_insert() {
        // this is just a reminder for myself: the values are not Ord or PartialOrd and are **not**
        // sorted.
        let mut list: SortedList<u32, u8> = SortedList::new();

        list.insert_only_new(1, 3);
        list.insert_only_new(0, 1);
        list.insert_only_new(0, 0);
        list.insert_only_new(2, 4);
        list.insert_only_new(0, 2);
        list.insert_only_new(3, 6);
        list.insert_only_new(2, 5);

        let items: Vec<(u32, u8)> = list.iter().map(|(k, v)| (*k, *v)).collect::<Vec<_>>();

        assert_eq!(
            items,
            vec![(0, 1), (0, 0), (0, 2), (1, 3), (2, 4), (2, 5), (3, 6),]
        );
    }

    #[test]
    fn double_ended_iter_zag_zig() {
        let mut list: SortedList<u32, u8> = SortedList::new();

        list.insert_only_new(1, 3);
        list.insert_only_new(0, 0);
        list.insert_only_new(0, 1);
        list.insert_only_new(2, 4);
        list.insert_only_new(0, 2);
        list.insert_only_new(3, 6);
        list.insert_only_new(2, 5);

        let mut iter = list.iter();
        assert_eq!(iter.next_back(), (&3, &6).into());
        assert_eq!(iter.next(), (&0, &0).into());

        assert_eq!(iter.next_back(), (&2, &5).into());
        assert_eq!(iter.next(), (&0, &1).into());

        assert_eq!(iter.next_back(), (&2, &4).into());
        assert_eq!(iter.next(), (&0, &2).into());

        assert_eq!(iter.next_back(), (&1, &3).into());
        assert_eq!(iter.next(), None);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn into_iter() {
        let mut list: SortedList<u32, u8> = SortedList::new();

        list.insert_only_new(1, 3);
        list.insert_only_new(0, 0);
        list.insert_only_new(0, 1);
        list.insert_only_new(2, 4);
        list.insert_only_new(0, 2);
        list.insert_only_new(3, 6);
        list.insert_only_new(2, 5);

        assert_eq!(
            list.clone().into_iter().collect::<Vec<_>>(),
            to_vec(list.iter())
        );
    }

    #[test]
    fn from_iter() {
        let coll = (0..20)
            .into_iter()
            .map(|x| (x, x + 5))
            .collect::<SortedList<_, _>>();
        assert_eq!(coll.len(), 20);
    }
}
