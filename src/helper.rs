use ordered_float::OrderedFloat;
// use sorted_list::SortedList;

pub struct LinearInterpolator {
    X: Vec<f64>,
    Y: Vec<f64>,
    xmax: f64,
    xmin: f64,
}

impl LinearInterpolator {
    pub fn new(X: &Vec<f64>, Y: &Vec<f64>) -> LinearInterpolator {
        assert!(X.len() == Y.len());

        let xmax = *X.iter().max_by_key(|n| OrderedFloat(n.abs())).unwrap();
        let xmin = *X.iter().min_by_key(|n| OrderedFloat(n.abs())).unwrap();

        // X needs to be sorted, so we zip X & Y and sort the tuples
        let mut both: Vec<(&f64, &f64)> = X.iter().zip(Y.iter()).collect();
        both.sort_by(|a, b| a.0.partial_cmp(b.0).unwrap());

        let X_ = both.iter().map(|x| *x.0).collect();
        let Y_ = both.iter().map(|x| *x.1).collect();

        LinearInterpolator {
            X: X_,
            Y: Y_,
            xmax: xmax,
            xmin: xmin,
        }
    }
    pub fn interpolate(&self, a: f64) -> f64 {
        assert!(self.xmin <= a && a <= self.xmax);

        let result: f64 = 0.;

        let mut idx: usize = 0;
        for i in 0..self.X.len() - 1 {
            if self.X[i] <= a && a <= self.X[i + 1] {
                idx = i;
                break;
            }
        }

        self.Y[idx]
            + (self.Y[idx + 1] - self.Y[idx]) / (self.X[idx + 1] - self.X[idx]) * (a - self.X[idx])
    }
}

// pub fn linear_interpolation(X: &Vec<f64>, Y: &Vec<f64>, a: f64) -> f64 {
//     assert!(X.len() == Y.len());
//     let xmax = *X.iter().max_by_key(|n| OrderedFloat(n.abs())).unwrap();
//     let xmin = *X.iter().min_by_key(|n| OrderedFloat(n.abs())).unwrap();
//     assert!(xmin <= a && a <= xmax);
//     let result: f64 = 0.;
//     let mut idx: usize = 0;
//     for i in 0..X.len() - 1 {
//         if X[i] <= a && a <= X[i + 1] {
//             idx = i;
//             break;
//         }
//     }
//     Y[idx] + (Y[idx + 1] - Y[idx]) / (X[idx + 1] - X[idx]) * (a - X[idx])
// }

// pub trait RemoveElem {
//     fn remove(&mut self, index: usize);
// }


// impl<K: Ord, V: PartialEq> RemoveElem for SortedList<K, V> {
//     fn remove(&mut self, index: usize) {
//         fn assert_failed(index: usize, len: usize) -> ! {
//             panic!("removal index (is {}) should be < len (is {})", index, len);
//         }

//         let len = self.keys.len();
//         if index >= len {
//             assert_failed(index, len);
//         }

//         self.keys.remove(index);
//         self.values.remove(index);


//         // unsafe {
//         //     // infallible
//         //     let ret;
//         //     {
//         //         // the place we are taking from.
//         //         let ptr = self.as_mut_ptr().add(index);
//         //         // copy it out, unsafely having a copy of the value on
//         //         // the stack and in the vector at the same time.
//         //         ret = ptr::read(ptr);

//         //         // Shift everything down to fill in that spot.
//         //         ptr::copy(ptr.offset(1), ptr, len - index - 1);
//         //     }
//         //     self.set_len(len - 1);
//         //     ret
//         // }
//     }
// }
