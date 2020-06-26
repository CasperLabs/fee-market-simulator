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
        let result: f64 = 0.;

        // // Linear search for the index
        // let mut idx: usize = 0;
        // for i in 0..self.X.len() - 1 {
        //     if self.X[i] <= a && a <= self.X[i + 1] {
        //         idx = i;
        //         break;
        //     }
        // }

        // Binary search for the index
        let idx = self.lower_bound_index(a);

        self.Y[idx]
            + (self.Y[idx + 1] - self.Y[idx]) / (self.X[idx + 1] - self.X[idx]) * (a - self.X[idx])
    }

    fn lower_bound_index(&self, a: f64) -> usize {
        /// Uses binary search to find the index of the lower bound for a number in X
        assert!(self.check_bounds(a));
        let mut bottom: usize = 0;
        let mut middle: usize = 0;
        let mut top: usize = self.X.len() - 1;

        loop {
            if top == bottom || top - 1 == bottom {
                break;
            }
            middle = ((top + bottom + 1) - (top + bottom + 1) % 2) / 2;
            // println!("{} {} {}", bottom, middle, top);

            if self.X[bottom] <= a && a < self.X[middle] {
                top = middle;
            } else {
                bottom = middle;
            }
        }
        bottom
    }

    fn check_bounds(&self, a: f64) -> bool {
        self.xmin <= a && a <= self.xmax
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

#[cfg(test)]
mod tests {
    use super::LinearInterpolator;
    use std::fmt::Debug;

    #[test]
    fn test_interpolate1() {
        let interp =
            LinearInterpolator::new(&vec![0., 1., 2., 4., 8.], &vec![0., 1., 3., 12., 13.]);

        // interp.lower_bound_index(2.);

        assert_eq!(interp.interpolate(0.), 0.);
        assert_eq!(interp.interpolate(0.5), 0.5);
        assert_eq!(interp.interpolate(1.), 1.);
        assert_eq!(interp.interpolate(1.5), 2.);
        assert_eq!(interp.interpolate(2.), 3.);
    }
}
