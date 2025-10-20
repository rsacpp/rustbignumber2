use env_logger::Builder;
use log::LevelFilter;
use std::time::{Duration, SystemTime};

use log::info;
mod bn64;
use bn64::Bn64;

mod bn128;
use bn128::Bn128;

// #[tokio::main]
fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    let mersenne: usize = 2281;
    let mut p = bn64::mersenne(mersenne);

    p.to_hex();
    let mut p_1 = p.clone();
    p_1.sub_at(0, 1);
    let mut bn0: Bn64 = Bn64::from(String::from("abcdef0123456789"));
    let start = SystemTime::now();
    let r = bn64::npmod2(&mut bn0, &mut p_1, &mut p);
    let end = SystemTime::now();
    info!("Time elapsed {:?}", end.duration_since(start));
    r.to_hex();
    /*
     let r0 = bn64::npmod(&mut bn0, &mut p_1, &mut p);
     r0.to_hex();
    */
    let mut p_2 = bn128::mersenne(mersenne);
    p_2.to_hex();
    let mut p_3 = p_2.clone();
    p_3.sub_at(0, 1);
    let mut b128: Bn128 = Bn128::from(String::from("abcdef0123456789"));
    let start = SystemTime::now();
    let r = bn128::npmod2(&mut b128, &mut p_3, &mut p_2);
    let end = SystemTime::now();
    info!("Time elapsed {:?}", end.duration_since(start));
    r.to_hex();
}
