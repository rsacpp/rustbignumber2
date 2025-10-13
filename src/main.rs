use env_logger::Builder;
use log::LevelFilter;
// use log::info;
mod bn64;
use bn64::Bn64;

fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    let mut p = bn64::mersenne(2281);

    p.to_hex();
    let mut p_1 = p.clone();
    // p_1.sub_at(0, 1);
    let mut bn0: Bn64 = Bn64::from(String::from("abcdef0123456789"));
    let r = bn64::npmod(&mut bn0, &mut p_1, &mut p);
    r.to_hex();
}

