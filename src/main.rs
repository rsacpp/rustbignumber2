use env_logger::Builder;
use log::LevelFilter;
use log::info;
mod bn64;
use bn64::Bn64;

fn main() {
    Builder::new().filter_level(LevelFilter::Info).init();

    let mut bn0: Bn64 = Bn64::from(String::from("111124"));
    bn0.to_hex();

    let mut p_1: Bn64 = Bn64::new(1);
    p_1.add_at(0, 8190);
    let mut p: Bn64 = Bn64::new(1);
    p.add_at(0, 8191);

    let result = bn64::npmod(&mut bn0, &mut p_1, &mut p);
    result.to_hex();
    // left_push();
}

fn left_push() {
    let v: u64 = 0x1;
    for index in 0..70 {
        let (v, b) = v.overflowing_sub(index);
        info!("{v}, {b}");
    }
}
