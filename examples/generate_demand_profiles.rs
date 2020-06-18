use std::f64;
use std::fs::File;
use std::io::prelude::*;

static BLOCKS_IN_DAY: u64 = 144;

fn profile1(i_: u64) -> u64 {
    let i = i_ as f64;
    let base = 5000.;
    let osc_amplitude = 2000.;
    let osc = osc_amplitude * (2. * f64::consts::PI / BLOCKS_IN_DAY as f64 * i).sin();
    (base + osc) as u64
}

fn profile2(i_: u64) -> u64 {
    let i = i_ as f64;
    let base = 5000.;
    // long term
    let trend_peak = 3000.;
    let peak_center = BLOCKS_IN_DAY as f64 * 100.;
    let rate_param = BLOCKS_IN_DAY as f64 * 27.;
    let trend = trend_peak * (-((i - peak_center).powi(2)) / 2. / rate_param.powi(2)).exp();
    // short term
    let osc_amplitude = 2000. * (base + trend) / base;
    let osc = osc_amplitude * (2. * f64::consts::PI / BLOCKS_IN_DAY as f64 * i).sin();
    (base + osc + trend) as u64
}

fn main() {
    let mut file1 = File::create("profile1.csv").unwrap();

    for i in 1..(BLOCKS_IN_DAY * 40) {
        file1.write_all(format!("{}\n", profile1(i)).as_bytes());
    }
}
