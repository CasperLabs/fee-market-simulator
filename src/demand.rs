use csv;
use std::fs::File;

use is_sorted;
use itertools_num::linspace;
use ordered_float::OrderedFloat;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::iter::FromIterator;

use crate::helper::*;

// struct CsvRecord {
//     price: String,
//     quantity: u32,
// }

pub struct DemandCurve {
    price: Vec<u64>,
    quantity: Vec<u64>,
    inverse_transform: Vec<u64>,
    rng: ThreadRng,
}

impl DemandCurve {
    pub fn new(price: Vec<u64>, quantity: Vec<u64>, interp_resolution: u64) -> DemandCurve {
        if !price.is_sorted() {
            panic!("Input price vector must be sorted in increasing order");
        }

        if quantity.last().unwrap().clone() != 0 {
            panic!("Input quantity vector must have 0 as the last element");
        }

        let price_interp = Vec::from_iter(linspace(
            price.iter().min().unwrap().clone() as f64,
            price.iter().max().unwrap().clone() as f64,
            interp_resolution as usize,
        ));

        let price_f64 = price.iter().map(|&x| x as f64).collect();
        let quantity_f64 = quantity.iter().map(|&x| x as f64).collect();

        let interpolator1 = LinearInterpolator::new(&price_f64, &quantity_f64);

        let mut quantity_interp: Vec<f64> = price_interp
            .iter()
            .map(|x| interpolator1.interpolate(*x))
            .collect();

        let quantity_interp_max = *quantity_interp
            .iter()
            .max_by_key(|n| OrderedFloat(n.abs()))
            .unwrap();
        quantity_interp = quantity_interp
            .iter()
            .map(|x| x / quantity_interp_max)
            .collect();

        let X = linspace(0., 1., interp_resolution as usize);

        let interpolator2 = LinearInterpolator::new(&quantity_interp, &price_interp);

        let inverse_transform: Vec<u64> = X.map(|x| interpolator2.interpolate(x) as u64).collect();

        // println!("{:?}", quantity_interp);
        // println!("{:?}", price_interp);
        // println!("{:?}", inverse_transform);

        DemandCurve {
            price: price,
            quantity: quantity,
            inverse_transform: inverse_transform,
            rng: rand::thread_rng(),
        }
    }

    pub fn from_csv(path: &str, interp_resolution: u64) -> DemandCurve {
        let file = File::open(path).expect("Couldn't open input CSV file");
        let mut reader = csv::ReaderBuilder::new()
            .has_headers(true)
            .from_reader(file);

        let mut price: Vec<u64> = Vec::new();
        let mut quantity: Vec<u64> = Vec::new();

        for record in reader.records() {
            let record = record.unwrap();
            price.push(record[0].parse().unwrap());
            quantity.push(record[1].parse().unwrap());
        }

        DemandCurve::new(price, quantity, interp_resolution)
    }

    pub fn sample_price(&mut self, size: usize) -> Vec<u64> {
        // self.inverse_transform
        //     .choose_multiple(&mut self.rng, size)
        //     .cloned()
        //     .collect()
        let result: Vec<u64> = (0..size)
            .map(|x| *(self.inverse_transform.choose(&mut self.rng).unwrap()))
            .collect();
        result
    }
}
