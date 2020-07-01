use ordered_float::OrderedFloat;
// use sorted_list::SortedList;

pub struct LinearInterpolator {
    x: Vec<f64>,
    y: Vec<f64>,
    xmax: f64,
    xmin: f64,
}

impl LinearInterpolator {
    pub fn new(x: &Vec<f64>, y: &Vec<f64>) -> LinearInterpolator {
        assert!(x.len() == y.len());

        let xmax = *x.iter().max_by_key(|n| OrderedFloat(n.abs())).unwrap();
        let xmin = *x.iter().min_by_key(|n| OrderedFloat(n.abs())).unwrap();

        // x needs to be sorted, so we zip x & y and sort the tuples
        let mut both: Vec<(f64, f64)> = x.iter().copied().zip(y.iter().copied()).collect();
        both.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let (x_, y_) = both.into_iter().unzip();

        LinearInterpolator {
            x: x_,
            y: y_,
            xmax: xmax,
            xmin: xmin,
        }
    }

    pub fn interpolate(&self, a: f64) -> f64 {
        let result: f64 = 0.;

        // // Linear search for the index
        // let mut idx: usize = 0;
        // for i in 0..self.x.len() - 1 {
        //     if self.x[i] <= a && a <= self.x[i + 1] {
        //         idx = i;
        //         break;
        //     }
        // }

        // Binary search for the index
        let idx = self.lower_bound_index(a);

        self.y[idx]
            + (self.y[idx + 1] - self.y[idx]) / (self.x[idx + 1] - self.x[idx]) * (a - self.x[idx])
    }

    /// Uses binary search to find the index of the lower bound for a number in x
    fn lower_bound_index(&self, a: f64) -> usize {
        assert!(self.check_bounds(a));
        let mut bottom: usize = 0;
        let mut middle: usize = 0;
        let mut top: usize = self.x.len() - 1;

        loop {
            if top == bottom || top - 1 == bottom {
                break;
            }
            middle = ((top + bottom + 1) - (top + bottom + 1) % 2) / 2;
            // println!("{} {} {}", bottom, middle, top);

            if self.x[bottom] <= a && a < self.x[middle] {
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

    pub fn xmin(&self) -> f64 {
        self.xmin
    }

    pub fn xmax(&self) -> f64 {
        self.xmax
    }
}

// pub fn linear_interpolation(x: &Vec<f64>, y: &Vec<f64>, a: f64) -> f64 {
//     assert!(x.len() == y.len());
//     let xmax = *x.iter().max_by_key(|n| OrderedFloat(n.abs())).unwrap();
//     let xmin = *x.iter().min_by_key(|n| OrderedFloat(n.abs())).unwrap();
//     assert!(xmin <= a && a <= xmax);
//     let result: f64 = 0.;
//     let mut idx: usize = 0;
//     for i in 0..x.len() - 1 {
//         if x[i] <= a && a <= x[i + 1] {
//             idx = i;
//             break;
//         }
//     }
//     y[idx] + (y[idx + 1] - y[idx]) / (x[idx + 1] - x[idx]) * (a - x[idx])
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
