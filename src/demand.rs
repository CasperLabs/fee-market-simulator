use csv;
use std::fs::File;

use is_sorted;
use itertools_num::linspace;
use ordered_float::OrderedFloat;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use std::iter::FromIterator;

use crate::helper::*;

struct CsvRecord {
    p: String,
    q: u32,
}


pub struct DemandCurve {
    p: Vec<u64>,
    q: Vec<u64>,
    interp_resolution: u64,
    Finv_arr: Vec<u64>,
    rng: ThreadRng,
}

impl DemandCurve {
    pub fn new(p: Vec<u64>, q: Vec<u64>, interp_resolution: u64) -> DemandCurve {
        if !p.is_sorted() {
            panic!("Input price vector must be sorted in increasing order");
        }

        if q.last().unwrap().clone() != 0 {
            panic!("Input quantity vector must have 0 as the last element");
        }

        let P_int = Vec::from_iter(linspace(
            p.iter().min().unwrap().clone() as f64,
            p.iter().max().unwrap().clone() as f64,
            interp_resolution as usize,
        ));

        // let P_int = Vec::from_iter(P_int);

        let p_f64 = p.iter().map(|&x| x as f64).collect();
        let q_f64 = q.iter().map(|&x| x as f64).collect();

        let Q_val_interp = LinearInterpolator::new(&p_f64, &q_f64);

        let mut Q_val: Vec<f64> = P_int.iter().map(|x| Q_val_interp.interpolate(*x)).collect();

        let Q_val_max = *Q_val.iter().max_by_key(|n| OrderedFloat(n.abs())).unwrap();
        Q_val = Q_val.iter().map(|x| x / Q_val_max).collect();

        let X = linspace(0., 1., interp_resolution as usize);

        let Finv_interp = LinearInterpolator::new(&Q_val, &P_int);

        let Finv_arr_f64: Vec<f64> = X.map(|x| Finv_interp.interpolate(x)).collect();

        let Finv_arr = Finv_arr_f64.iter().map(|&x| x as u64).collect();
        // println!("{:?}", Q_val);
        // println!("{:?}", P_int);
        // println!("{:?}", Finv_arr);

        // let mut rng = &mut rand::thread_rng();

        DemandCurve {
            p: p,
            q: q,
            interp_resolution: interp_resolution,
            Finv_arr: Finv_arr,
            rng: rand::thread_rng(),
        }
    }

    pub fn from_csv(path: &str, interp_resolution: u64) -> DemandCurve {
        let file = File::open(path).expect("Couldn't open input CSV file");
        let mut reader = csv::ReaderBuilder::new().has_headers(true).from_reader(file);

        let mut p: Vec<u64> = Vec::new();
        let mut q: Vec<u64> = Vec::new();

        for record in reader.records() {
            let record = record.unwrap();
            p.push(record[0].parse().unwrap());
            q.push(record[1].parse().unwrap());
        }

        DemandCurve::new(p, q, interp_resolution)
    }

    pub fn sample_price(&mut self, size: usize) -> Vec<u64> {
        self.Finv_arr
            .choose_multiple(&mut self.rng, size)
            .cloned()
            .collect()
    }
}
