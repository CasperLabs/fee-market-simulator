use std::f64;
use std::fs::File;
use std::io::prelude::*;

static N_DAY: u64 = 700;
static BLOCKS_IN_DAY: u64 = 144;

fn profile(i_: u64) -> u64 {
    let i = i_ as f64;
    let base = 5000.;
    let osc_amplitude = 2000.;
    let osc = osc_amplitude * (2. * f64::consts::PI / BLOCKS_IN_DAY as f64 * i).sin();
    (base + osc) as u64
}

fn main() {
    let mut file = File::create("demand_profile.csv").unwrap();
    for i in 1..(BLOCKS_IN_DAY * N_DAY) {
        file.write_all(format!("{}\n", profile(i)).as_bytes()).unwrap();
    }
}
